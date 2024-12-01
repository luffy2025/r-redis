use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, Executor, RESP_EMPTY};
use crate::{RespArray, RespFrame};

#[allow(dead_code)]
pub struct HGet {
    key: String,
    field: String,
}

impl Executor for HGet {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        match backend.hget(&self.key, &self.field) {
            Some(value) => Ok(value),
            None => Ok(RESP_EMPTY.clone()),
        }
    }
}

// hget map hello
// *3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n
impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => Ok(HGet {
                key: String::from_utf8(key.to_vec())?,
                field: String::from_utf8(field.to_vec())?,
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
    fn test_hget_command() -> Result<()> {
        let mut cmd =
            bytes::BytesMut::from(&b"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let get = HGet::try_from(cmd)?;
        assert_eq!(get.key, "map");
        assert_eq!(get.field, "hello");
        Ok(())
    }
}
