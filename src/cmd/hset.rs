use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, CommandExecutor, RESP_OK};
use crate::{RespArray, RespFrame};

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

impl CommandExecutor for HSet {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        backend.hset(self.key, self.field, self.value);
        Ok(RESP_OK.clone())
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
    use crate::cmd::hget::HGet;
    use crate::cmd::RESP_EMPTY;
    use crate::{BulkString, RespDecode};
    use anyhow::Result;

    #[test]
    fn test_hset_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..],
        );
        let cmd = RespArray::decode(&mut cmd)?;
        let set = HSet::try_from(cmd)?;
        assert_eq!(set.key, "key");
        assert_eq!(set.field, "hello");
        assert_eq!(set.value, BulkString::new(b"world").into());
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

    #[test]
    fn test_execute_hset_and_hget() -> Result<()> {
        let backend = Backend::new();

        let mut cmd = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..],
        );
        let cmd = RespArray::decode(&mut cmd)?;
        let set = HSet::try_from(cmd)?;
        let ret = set.execute(&backend)?;
        assert_eq!(ret, RESP_OK.clone());

        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$4\r\nhget\r\n$3\r\nkey\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let get = HGet::try_from(cmd)?;
        let ret = get.execute(&backend)?;
        assert_eq!(ret, BulkString::new(b"world").into());

        let mut cmd = bytes::BytesMut::from(
            &b"*3\r\n$4\r\nhget\r\n$3\r\nkey\r\n$5\r\nnot_exist_field\r\n"[..],
        );
        let cmd = RespArray::decode(&mut cmd)?;
        let get = HGet::try_from(cmd)?;
        let ret = get.execute(&backend)?;
        assert_eq!(ret, RESP_EMPTY.clone());

        Ok(())
    }
}
