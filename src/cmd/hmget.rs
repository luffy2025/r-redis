use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, CommandExecutor, RESP_EMPTY};
use crate::{RespArray, RespFrame};

#[derive(Debug)]
pub struct HMGet {
    key: String,
    fields: Vec<String>,
}

impl CommandExecutor for HMGet {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        match backend.hmget(&self.key, self.fields.as_slice()) {
            Some(value) => Ok(RespArray::from(value).into()),
            None => Ok(RESP_EMPTY.clone()),
        }
    }
}

// hget map hello
// *4\r\n$5\r\nhmget\r\n$3\r\nkey\r\n$5\r\nhello\r\n$2\r\nhi\r\n
impl TryFrom<RespArray> for HMGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hmget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        let key = match args.next() {
            Some(RespFrame::BulkString(key)) => String::from_utf8(key.to_vec())?,
            _ => return Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
        };

        let mut fields: Vec<String> = Vec::with_capacity(args.len() - 1);
        for f in args {
            match f {
                RespFrame::BulkString(field) => fields.push(String::from_utf8(field.to_vec())?),
                _ => return Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
            }
        }

        Ok(HMGet { key, fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BulkString, RespDecode, RespNull};
    use anyhow::Result;

    #[test]
    fn test_hmget_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(
            &b"*5\r\n$5\r\nhmget\r\n$3\r\nkey\r\n$5\r\nhello\r\n$2\r\nhi\r\n$3\r\nbye\r\n"[..],
        );

        let cmd = RespArray::decode(&mut cmd)?;
        let cmd = HMGet::try_from(cmd)?;
        assert_eq!(cmd.key, "key");
        assert_eq!(cmd.fields, ["hello", "hi", "bye"]);

        let backend = Backend::new();

        backend.hset(
            "key".into(),
            "hello".into(),
            BulkString::from("world").into(),
        );
        backend.hset("key".into(), "hi".into(), BulkString::from("rust").into());

        let ret = cmd.execute(&backend)?;
        let ret = match ret {
            RespFrame::Array(arr) => arr,
            _ => panic!("Expecting array"),
        };

        assert_eq!(
            ret,
            vec![
                BulkString::from("world").into(),
                BulkString::from("rust").into(),
                RespNull.into()
            ]
            .into(),
        );

        Ok(())
    }
}
