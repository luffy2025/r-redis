use crate::{RespEncode, RespFrame};
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
}
