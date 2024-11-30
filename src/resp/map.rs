use crate::resp::simple_string::SimpleString;
use crate::{is_combine_complete, RespDecode, RespEncode, RespError, RespFrame};
use bytes::{Buf, BytesMut};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespMap(BTreeMap<SimpleString, RespFrame>);

const MAP_CAP: usize = 4096;

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(MAP_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in &self.0 {
            buf.extend_from_slice(&key.encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

impl RespDecode for RespMap {
    const PREFIX: &'static u8 = &b'%';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = crate::extract_end_and_length(buf, &[*Self::PREFIX])?;
        is_combine_complete(buf, len)?;

        buf.advance(end + 2);
        let mut map = RespMap::new();
        for _ in 0..len {
            let key = SimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<SimpleString, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(BTreeMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::bulk_string::BulkString;
    use anyhow::Result;

    #[test]
    fn test_resp_map() {
        let mut map = RespMap::new();
        map.insert(
            SimpleString::new("get"),
            RespFrame::BulkString(BulkString::new("hello".into())),
        );
        map.insert(
            SimpleString::new("set"),
            RespFrame::BulkString(BulkString::new("world".into())),
        );
        map.insert(SimpleString::new("add"), 10.into());

        assert_eq!(
            map.encode(),
            b"%3\r\n+add\r\n:+10\r\n+get\r\n$5\r\nhello\r\n+set\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_resp_map_decode() -> Result<()> {
        let mut buf = BytesMut::from(
            &b"%3\r\n+add\r\n:+10\r\n+get\r\n$5\r\nhello\r\n+set\r\n$5\r\nworld\r\n"[..],
        );
        let map = RespMap::decode(&mut buf)?;
        let mut expected = RespMap::new();
        expected.insert(
            SimpleString::new("get"),
            RespFrame::BulkString(BulkString::new("hello".into())),
        );
        expected.insert(
            SimpleString::new("set"),
            RespFrame::BulkString(BulkString::new("world".into())),
        );
        expected.insert(SimpleString::new("add"), 10.into());
        assert_eq!(map, expected);

        Ok(())
    }
}
