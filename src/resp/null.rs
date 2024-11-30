use crate::{is_fixed_complete, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNull;

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        "_\r\n".as_bytes().to_vec()
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static u8 = &b'_';
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::RespError> {
        is_fixed_complete(buf)?;

        let s = buf.split_to(3).to_vec();
        if s != b"_\r\n" {
            Err(RespError::InvalidFrame(
                String::from_utf8_lossy(s.as_slice()).into(),
            ))
        } else {
            Ok(RespNull)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resp_null_encode() {
        assert_eq!(RespNull.encode(), b"_\r\n");
    }

    #[test]
    fn test_resp_null_decode() {
        let mut buf = bytes::BytesMut::from(&b"_\r\n"[..]);
        assert_eq!(RespNull::decode(&mut buf).unwrap(), RespNull);
    }
}
