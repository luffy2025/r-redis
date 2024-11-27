use crate::RespEncode;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BulkString(Vec<u8>);

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        let length = self.len();
        let mut buf = Vec::with_capacity(1 + 2 + 1 + 1 + 1 + length + 2);
        buf.push(b'$');
        buf.extend_from_slice(format!("{}\r\n", length).as_bytes());
        buf.extend_from_slice(self);
        buf.extend_from_slice(b"\r\n");
        buf
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

    #[test]
    fn test_bulk_string() {
        let bs: RespFrame = BulkString::new(b"hello".to_vec()).into();
        assert_eq!(bs.encode(), b"$5\r\nhello\r\n");
    }
}
