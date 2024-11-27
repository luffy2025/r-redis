use crate::resp::simple_string::SimpleString;
use crate::{RespEncode, RespFrame};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespMap(BTreeMap<String, RespFrame>);

const MAP_CAP: usize = 4096;

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for RespMap {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(MAP_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (key, value) in &self.0 {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

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

    #[test]
    fn test_resp_map() {
        let mut map = RespMap::new();
        map.insert(
            "get".to_string(),
            RespFrame::BulkString(BulkString::new("hello".into())),
        );
        map.insert(
            "set".to_string(),
            RespFrame::BulkString(BulkString::new("world".into())),
        );
        map.insert("add".to_string(), 10.into());

        assert_eq!(
            map.encode(),
            b"%3\r\n+add\r\n:+10\r\n+get\r\n$5\r\nhello\r\n+set\r\n$5\r\nworld\r\n"
        );
    }
}
