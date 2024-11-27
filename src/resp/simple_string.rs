use crate::RespEncode;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleString(String);

// - simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        format!("+{}\r\n", self).as_bytes().to_vec()
    }
}

impl Deref for SimpleString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
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

    #[test]
    fn test_simple_string() {
        let s = SimpleString::new("OK");
        assert_eq!(s.encode(), b"+OK\r\n");
    }
}
