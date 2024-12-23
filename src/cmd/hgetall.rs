use crate::backend::Backend;
use crate::cmd::{extract_args, validate_command, CommandError, CommandExecutor, RESP_EMPTY};
use crate::{BulkString, RespArray, RespFrame};

#[derive(Debug)]
pub struct HGetAll {
    key: String,
}

impl CommandExecutor for HGetAll {
    fn execute(self, backend: &Backend) -> Result<RespFrame, CommandError> {
        match backend.hget_all(&self.key) {
            Some(map) => {
                let mut ret: Vec<RespFrame> = Vec::with_capacity(map.len() * 2);
                map.iter().all(|item| {
                    ret.push(BulkString::from(item.key().to_string()).into());
                    ret.push(item.value().clone());
                    true
                });
                Ok(RespArray::new(ret).into())
            }
            None => Ok(RESP_EMPTY.clone()),
        }
    }
}

// hgetall key
// *2\r\n$7\r\nhgetall\r\n$3\r\nkey\r\n
impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(HGetAll {
                key: String::from_utf8(key.to_vec())?,
            }),
            _ => Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::hset::HSet;
    use crate::RespDecode;
    use anyhow::Result;

    #[test]
    fn test_hgetall_command() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(&b"*2\r\n$7\r\nhgetall\r\n$3\r\nkey\r\n"[..]);
        let cmd = RespArray::decode(&mut cmd)?;
        let hget_all = HGetAll::try_from(cmd)?;
        assert_eq!(hget_all.key, "key");

        Ok(())
    }

    #[test]
    fn test_hset_and_hgetall_command() -> Result<()> {
        let backend = Backend::new();

        let mut hset = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..],
        );
        let hset = RespArray::decode(&mut hset)?;
        let hset = HSet::try_from(hset)?;
        hset.execute(&backend)?;

        let mut hset = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nkey\r\n$2\r\nhi\r\n$4\r\nrust\r\n"[..],
        );
        let hset = RespArray::decode(&mut hset)?;
        let hset = HSet::try_from(hset)?;
        hset.execute(&backend)?;

        let mut hgetall = bytes::BytesMut::from(&b"*2\r\n$7\r\nhgetall\r\n$3\r\nkey\r\n"[..]);
        let hgetall = RespArray::decode(&mut hgetall)?;
        let hgetall = HGetAll::try_from(hgetall)?;
        let resp = hgetall.execute(&backend)?;

        let resp = match resp {
            RespFrame::Array(array) => array,
            _ => panic!("Expected Array"),
        };
        assert_eq!(resp.len() % 2, 0);

        // 排序
        let mut sorted: Vec<(RespFrame, RespFrame)> = Vec::with_capacity(resp.len() / 2);

        resp.iter().enumerate().step_by(2).for_each(|(i, key)| {
            sorted.push((key.to_owned(), resp[i + 1].to_owned()));
        });

        sorted.sort_by(|a, b| {
            let a = match a.0 {
                RespFrame::BulkString(ref a) => a,
                _ => panic!("Expected BulkString"),
            };
            let b = match b.0 {
                RespFrame::BulkString(ref b) => b,
                _ => panic!("Expected BulkString"),
            };
            a.cmp(b)
        });

        let sorted_resp = sorted
            .into_iter()
            .flat_map(|(k, v)| vec![k, v])
            .collect::<Vec<RespFrame>>();

        assert_eq!(
            sorted_resp,
            vec![
                BulkString::from("hello").into(),
                BulkString::from("world").into(),
                BulkString::from("hi").into(),
                BulkString::from("rust").into()
            ]
        );

        Ok(())
    }
}
