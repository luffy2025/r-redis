use crate::{RespEncode, RespFrame};
use std::collections::BTreeSet;
use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct RespSet(BTreeSet<RespFrame>);

impl Hash for RespSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let buf = self.encode();
        buf.hash(state);
    }
}

impl Eq for RespSet {}

impl PartialOrd for RespSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RespSet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.encode().cmp(&other.encode())
    }
}

const SET_CAP: usize = 4096;

// - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespSet {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(SET_CAP);
        buf.extend_from_slice(format!("~{}\r\n", self.len()).as_bytes());
        for frame in &self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

impl Deref for RespSet {
    type Target = BTreeSet<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::array::RespArray;
    use crate::resp::bulk_string::BulkString;

    #[test]
    fn test_resp_set() {
        let mut set = BTreeSet::new();
        set.insert(BulkString::new(b"hello".to_vec()).into());
        set.insert(BulkString::new(b"world".to_vec()).into());
        set.insert(10.into());
        set.insert(
            RespArray::new(vec![(-123).into(), BulkString::new("arr".into()).into()]).into(),
        );

        let set = RespSet(set);
        assert_eq!(
            set.encode(),
            b"~4\r\n:+10\r\n$5\r\nhello\r\n$5\r\nworld\r\n*2\r\n:-123\r\n$3\r\narr\r\n"
        );
    }
}
