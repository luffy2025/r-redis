use crate::cmd::{extract_args, validate_command, CommandError, CommandExecutor};
use crate::{Backend, RespArray, RespFrame, SimpleString};

#[derive(Debug)]
pub struct Echo {
    arg: String,
}

impl CommandExecutor for Echo {
    fn execute(self, _: &Backend) -> Result<RespFrame, CommandError> {
        Ok(SimpleString::from(self.arg).into())
    }
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Echo {
                arg: String::from_utf8_lossy(key.as_slice()).to_string(),
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
    fn test_echo_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$4\r\necho\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let echo = Echo::try_from(cmd)?;
        let resp = echo.execute(&Backend::new())?;

        assert_eq!(resp, SimpleString::from("hello").into());
        Ok(())
    }
}
