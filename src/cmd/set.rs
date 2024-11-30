use crate::cmd::{extract_args, validate_command, CommandError, Executor};
use crate::{RespArray, RespFrame};

#[allow(dead_code)]
pub struct Set {
    key: String,
    value: RespFrame,
}

impl Executor for Set {
    fn execute(&self) -> Result<RespFrame, CommandError> {
        todo!()
    }
}

// set hello world
// *3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n
impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["set"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => {
                let key = String::from_utf8(key.to_vec())?;
                println!("{:?}", key);
                println!("{:?}", value);
                Ok(Set { key, value })
            }
            _ => Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BulkString, RespDecode};
    use anyhow::Result;

    #[test]
    fn test_set_command() -> Result<()> {
        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let set = Set::try_from(cmd)?;
        assert_eq!(set.key, "hello");
        assert_eq!(set.value, BulkString::new(b"world".into()).into());
        Ok(())
    }

    #[test]
    fn test_set_command_args_not_enough() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let set = Set::try_from(cmd);
        assert!(set.is_err());
        Ok(())
    }
}
