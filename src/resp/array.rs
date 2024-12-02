use crate::{
    extract_end_and_length, is_combine_complete, RespDecode, RespEncode, RespError, RespFrame,
};
use bytes::Buf;
use lazy_static::lazy_static;
use std::ops::{Deref, DerefMut};

const NULL_ARRAY_ENCODE: &[u8] = b"*-1\r\n";
const ARRAY_CAP: usize = 4096;

lazy_static! {
    static ref NULL_ARRAY: RespArray = RespArray::null();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespArray {
    pub(crate) data: Vec<RespFrame>,
}

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
//        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
impl RespEncode for RespArray {
    fn encode(&self) -> Vec<u8> {
        if self.is_empty() {
            return NULL_ARRAY_ENCODE.to_vec();
        }

        let mut buf = Vec::with_capacity(ARRAY_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.len()).as_bytes());
        for frame in &self.data {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static u8 = &b'*';
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        if buf == NULL_ARRAY_ENCODE {
            return Ok(NULL_ARRAY.clone());
        }

        let (end, len) = extract_end_and_length(buf, &[*Self::PREFIX])?;
        is_combine_complete(buf, len)?;

        buf.advance(end + 2);
        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let v = RespFrame::decode(buf)?;
            frames.push(v);
        }

        Ok(RespArray::new(frames))
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for RespArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl From<Vec<RespFrame>> for RespArray {
    fn from(frame: Vec<RespFrame>) -> Self {
        RespArray::new(frame)
    }
}

impl From<&[RespFrame]> for RespArray {
    fn from(frame: &[RespFrame]) -> Self {
        RespArray::new(frame.to_vec())
    }
}

impl RespArray {
    pub fn new(arr: Vec<RespFrame>) -> Self {
        RespArray { data: arr }
    }

    pub fn null() -> RespArray {
        RespArray { data: vec![] }
    }

    pub fn push(&mut self, frame: RespFrame) {
        self.data.push(frame);
    }

    pub fn first(&self) -> Option<&RespFrame> {
        self.data.first()
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
        let arr: RespArray = vec![
            BulkString::new("get").into(),
            BulkString::new("hello").into(),
            SimpleString::new("rust").into(),
        ]
        .into();
        assert_eq!(arr.encode(), b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+rust\r\n");
    }

    #[test]
    fn test_resp_array_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+world\r\n"[..]);
        let ret = RespArray::decode(&mut buf)?;
        assert_eq!(
            ret,
            vec![
                BulkString::new("get").into(),
                BulkString::new("hello").into(),
                SimpleString::new("world").into()
            ]
            .into()
        );

        Ok(())
    }

    #[test]
    fn test_resp_array_decode_not_complete() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n+China\r"[..]);
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"\n");
        let ret = RespArray::decode(&mut buf)?;
        assert_eq!(
            ret,
            vec![
                BulkString::new("get").into(),
                BulkString::new("hello").into(),
                SimpleString::new("China").into()
            ]
            .into()
        );
        Ok(())
    }

    #[test]
    fn test_resp_array_null() -> Result<()> {
        let arr = NULL_ARRAY.clone();
        assert_eq!(arr.encode(), b"*-1\r\n");

        let mut buf = bytes::BytesMut::from(&b"*-1\r\n"[..]);
        let ret = RespArray::decode(&mut buf)?;
        assert_eq!(ret, NULL_ARRAY.clone());

        Ok(())
    }
}
