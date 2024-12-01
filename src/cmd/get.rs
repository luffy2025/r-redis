use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, Executor, RESP_EMPTY};
use crate::{RespArray, RespFrame};

#[allow(dead_code)]
pub struct Get {
    key: String,
}

impl Executor for Get {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        match backend.get(&self.key) {
            Some(value) => Ok(value),
            None => Ok(RESP_EMPTY.clone()),
        }
    }
}

// get hello
// *2\r\n$3\r\nget\r\n$5\r\nhello\r\n
impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Get {
                key: String::from_utf8(key.to_vec())?,
            }),
            _ => Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespDecode;
    use anyhow::Result;

    #[test]
    fn test_get_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let get = Get::try_from(cmd)?;
        assert_eq!(get.key, "hello");
        Ok(())
    }
}
