use crate::{extract_end_and_length, is_single_complete, RespDecode, RespEncode, RespError};
use bytes::{Buf, BytesMut};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<String> for BulkString {
    fn from(s: String) -> Self {
        BulkString::new(s.into_bytes())
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString::new(s.as_bytes())
    }
}

impl From<&[u8]> for BulkString {
    fn from(data: &[u8]) -> Self {
        BulkString(data.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(data: &[u8; N]) -> Self {
        BulkString(data.to_vec())
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl BulkString {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_bulk_string_encode() {
        let bs: BulkString = b"hello".into();
        assert_eq!(bs.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"$5\r\nhello\r\n"[..]);
        let bs = BulkString::decode(&mut buf)?;
        assert_eq!(bs, b"hello".into());
        Ok(())
    }
}
