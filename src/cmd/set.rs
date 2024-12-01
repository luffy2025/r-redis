use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, Executor, RESP_OK};
use crate::{RespArray, RespFrame};

#[allow(dead_code)]
pub struct Set {
    key: String,
    value: RespFrame,
}

impl Executor for Set {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        backend.set(self.key, self.value);
        Ok(RESP_OK.clone())
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
    use crate::backend::Backend;
    use crate::cmd::Get;
    use crate::cmd::RESP_EMPTY;
    use crate::{BulkString, RespDecode};
    use anyhow::Result;

    #[test]
    fn test_set_command() -> Result<()> {
        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let set = Set::try_from(cmd)?;
        assert_eq!(set.key, "hello");
        assert_eq!(set.value, BulkString::new(b"world").into());
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

    #[test]
    fn test_execute_set_get() -> Result<()> {
        let backend = Backend::new();

        // set
        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let set = Set::try_from(cmd)?;
        assert_eq!(set.key, "hello");
        assert_eq!(set.value, BulkString::new(b"world").into());

        let ret = set.execute(&backend)?;
        assert_eq!(ret, RESP_OK.clone());

        // get
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let get = Get::try_from(cmd)?;

        let ret = get.execute(&backend)?;
        assert_eq!(ret, BulkString::new(b"world").into());

        // get not exist key
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$3\r\nget\r\n$5\r\nnot_exist_key\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let get = Get::try_from(cmd)?;
        let ret = get.execute(&backend)?;
        assert_eq!(ret, RESP_EMPTY.clone());

        Ok(())
    }
}
