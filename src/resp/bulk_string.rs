use crate::{extract_end_and_length, is_single_complete, RespDecode, RespEncode, RespError};
use bytes::{Buf, BytesMut};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BulkString(Vec<u8>);

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::with_capacity(1 + 2 + 1 + 1 + 1 + length + 2);
        buf.push(*Self::PREFIX);
        buf.extend_from_slice(format!("{}\r\n", length).as_bytes());
        buf.extend_from_slice(self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static u8 = &b'$';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = extract_end_and_length(buf, &[*Self::PREFIX])?;
        is_single_complete(buf, len)?;

        buf.advance(end + 2);
        let data = buf.split_to(len).to_vec();
        buf.advance(2);
        Ok(BulkString(data))
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BulkString {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;
    use anyhow::Result;

    #[test]
    fn test_bulk_string_encode() {
        let bs: RespFrame = BulkString::new(b"hello".to_vec()).into();
        assert_eq!(bs.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"$5\r\nhello\r\n"[..]);
        let bs = BulkString::decode(&mut buf)?;
        assert_eq!(bs, BulkString::new(b"hello".to_vec()));
        Ok(())
    }
}
