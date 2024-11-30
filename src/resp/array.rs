use crate::{
    extract_end_and_length, is_combine_complete, RespDecode, RespEncode, RespError, RespFrame,
};
use bytes::Buf;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespArray(Vec<RespFrame>);

const ARRAY_CAP: usize = 4096;

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
//        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
impl RespEncode for RespArray {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(ARRAY_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.len()).as_bytes());
        for frame in &self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static u8 = &b'*';
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let (end, len) = extract_end_and_length(buf, &[*Self::PREFIX])?;
        is_combine_complete(buf, len)?;

        buf.advance(end + 2);
        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let v = RespFrame::decode(buf)?;
            frames.push(v);
        }

        Ok(RespArray(frames))
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RespArray {
    pub fn new(arr: Vec<RespFrame>) -> Self {
        RespArray(arr)
    }

    pub fn push(&mut self, frame: RespFrame) {
        self.0.push(frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::bulk_string::BulkString;
    use crate::resp::simple_string::SimpleString;
    use anyhow::Result;

    #[test]
    fn test_resp_array() {
        let arr: RespFrame = RespArray(vec![
            BulkString::new("get".into()).into(),
            BulkString::new("hello".into()).into(),
            SimpleString::new("rust".to_string()).into(),
        ])
        .into();
        assert_eq!(arr.encode(), b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+rust\r\n");
    }

    #[test]
    fn test_resp_array_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+rust\r\n"[..]);
        let ret = RespArray::decode(&mut buf)?;
        assert_eq!(
            ret,
            RespArray::new(vec![
                BulkString::new("get".into()).into(),
                BulkString::new("hello".into()).into(),
                SimpleString::new("rust".to_string()).into(),
            ])
        );
        Ok(())
    }

    #[test]
    fn test_resp_array_decode_not_complete() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+rust\r"[..]);
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotCompete);

        buf.extend_from_slice(b"\n");
        let ret = RespArray::decode(&mut buf)?;
        assert_eq!(
            ret,
            RespArray::new(vec![
                BulkString::new("get".into()).into(),
                BulkString::new("hello".into()).into(),
                SimpleString::new("rust".to_string()).into(),
            ])
        );
        Ok(())
    }
}
