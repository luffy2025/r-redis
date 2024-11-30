use crate::{extract_simple_frame_data, is_fixed_complete, RespDecode, RespEncode, RespError};
use bytes::BytesMut;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleError(String);

// - error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        format!("-{}\r\n", self).as_bytes().to_vec()
    }
}

impl RespDecode for SimpleError {
    const PREFIX: &'static u8 = &b'-';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        is_fixed_complete(buf)?;

        let end = extract_simple_frame_data(buf, &[*Self::PREFIX])?;
        let s = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&s[1..end]);

        Ok(SimpleError(s.into()))
    }
}

impl Deref for SimpleError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_simple_error() {
        let s: RespFrame = SimpleError::new("Error message").into();
        assert_eq!(s.encode(), b"-Error message\r\n");
    }

    #[test]
    fn test_simple_error_decode() {
        let mut buf = BytesMut::from(&b"-Error message\r\n"[..]);
        let s = SimpleError::decode(&mut buf).unwrap();
        assert_eq!(s, SimpleError::new("Error message"));
    }
}
