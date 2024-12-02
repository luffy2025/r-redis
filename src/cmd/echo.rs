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

        let args = extract_args(value, 1)?.into_iter();
        let mut input = String::new();
        args.for_each(|arg| {
            if let RespFrame::BulkString(arg) = arg {
                input.push_str(&String::from_utf8_lossy(arg.as_slice()));
                input.push(' ');
            }
        });
        let input = input.trim().to_string();
        Ok(Echo::new(input))
    }
}

impl Echo {
    pub fn new(arg: String) -> Self {
        Echo { arg }
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

        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$4\r\necho\r\n$5\r\nhello\r\n$4\r\nboys\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let echo = Echo::try_from(cmd)?;
        let resp = echo.execute(&Backend::new())?;

        assert_eq!(resp, SimpleString::from("hello boys").into());
        Ok(())
    }
}
