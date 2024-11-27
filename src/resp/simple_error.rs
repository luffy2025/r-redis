use crate::RespEncode;
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
}
