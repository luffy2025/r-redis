use crate::{extract_simple_frame_data, is_fixed_complete, RespDecode, RespEncode, RespError};
use bytes::BytesMut;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleString(String);

// - simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        format!("+{}\r\n", self).as_bytes().to_vec()
    }
}

impl RespDecode for SimpleString {
    const PREFIX: &'static u8 = &b'+';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        is_fixed_complete(buf)?;

        let end = extract_simple_frame_data(buf, &[*Self::PREFIX])?;

        let s = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&s[1..end]);

        Ok(SimpleString(s.into()))
    }
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for SimpleString {
    fn from(s: String) -> Self {
        SimpleString(s)
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string())
    }
}

impl Display for SimpleString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use bytes::BufMut;

    #[test]
    fn test_simple_string() {
        let s = SimpleString::new("OK");
        assert_eq!(s.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_simple_string_decode() -> Result<()> {
        let mut buf = BytesMut::from(&b"+OK\r\n"[..]);
        let ret = SimpleString::decode(&mut buf)?;
        assert_eq!(ret, "OK".into());

        buf.extend_from_slice(b"+Hello\r".as_ref());
        let ret = SimpleString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let ret = SimpleString::decode(&mut buf)?;
        assert_eq!(ret, "Hello".into());

        Ok(())
    }
}
