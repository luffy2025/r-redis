use crate::{extract_simple_frame_data, is_fixed_complete, RespDecode, RespEncode};
use bytes::BytesMut;

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(&self) -> Vec<u8> {
        format!("#{}\r\n", if *self { "t" } else { "f" })
            .as_bytes()
            .to_vec()
    }
}

impl RespDecode for bool {
    const PREFIX: &'static u8 = &b'#';

    fn decode(buf: &mut BytesMut) -> Result<Self, crate::RespError> {
        is_fixed_complete(buf)?;

        let end = extract_simple_frame_data(buf, &[*Self::PREFIX])?;
        let s = buf.split_to(end + 2);
        Ok(s[1] == b't')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_bool_encode() {
        assert_eq!(true.encode(), b"#t\r\n");
        assert_eq!(false.encode(), b"#f\r\n");
    }

    #[test]
    fn test_bool_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b"#t\r\n"[..]);
        let ret = bool::decode(&mut buf)?;
        assert!(ret);

        let mut buf = bytes::BytesMut::from(&b"#f\r\n"[..]);
        let ret = bool::decode(&mut buf)?;
        assert!(!ret);

        Ok(())
    }
}
