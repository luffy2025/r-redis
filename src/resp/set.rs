use crate::{
    extract_end_and_length, is_combine_complete, RespDecode, RespEncode, RespError, RespFrame,
};
use bytes::{Buf, BytesMut};
use std::collections::BTreeSet;
use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct RespSet(BTreeSet<RespFrame>);

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

impl RespDecode for RespSet {
    const PREFIX: &'static u8 = &b'~';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = extract_end_and_length(buf, &[*Self::PREFIX])?;
        is_combine_complete(buf, len)?;

        buf.advance(end + 2);
        let mut frames = BTreeSet::new();
        for _ in 0..len {
            let v = RespFrame::decode(buf)?;
            frames.insert(v);
        }

        Ok(RespSet(frames))
    }
}

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
    use anyhow::Result;

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

    #[test]
    fn test_resp_set_decode() -> Result<()> {
        let mut buf = BytesMut::from(
            &b"~4\r\n:+10\r\n$5\r\nhello\r\n$5\r\nworld\r\n*2\r\n:-123\r\n$3\r\narr\r\n"[..],
        );
        let set = RespSet::decode(&mut buf)?;
        let mut expected = BTreeSet::new();
        expected.insert(BulkString::new(b"hello".to_vec()).into());
        expected.insert(BulkString::new(b"world".to_vec()).into());
        expected.insert(10.into());
        expected.insert(
            RespArray::new(vec![(-123).into(), BulkString::new("arr".into()).into()]).into(),
        );

        assert_eq!(set, RespSet(expected));

        Ok(())
    }
}
