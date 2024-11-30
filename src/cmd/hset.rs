use crate::cmd::{extract_args, validate_command, CommandError, Executor};
use crate::{RespArray, RespFrame};

#[allow(dead_code)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

impl Executor for HSet {
    fn execute(&self) -> Result<RespFrame, CommandError> {
        todo!()
    }
}

// hset map hello world
// *4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n
impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) => {
                Ok(HSet {
                    key: String::from_utf8(key.to_vec())?,
                    field: String::from_utf8(field.to_vec())?,
                    value,
                })
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
    fn test_hset_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..],
        );
        let cmd = RespArray::decode(&mut cmd)?;
        let set = HSet::try_from(cmd)?;
        assert_eq!(set.key, "map");
        assert_eq!(set.field, "hello");
        assert_eq!(set.value, BulkString::new(b"world".into()).into());
        Ok(())
    }

    #[test]
    fn test_hset_command_args_not_enough() -> Result<()> {
        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$4\r\nhset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let set = HSet::try_from(cmd);
        assert!(set.is_err());
        Ok(())
    }
}
