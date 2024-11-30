use crate::{extract_simple_frame_data, is_fixed_complete, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        let sign = if *self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).as_bytes().to_vec()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static u8 = &b':';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        is_fixed_complete(buf)?;

        let end = extract_simple_frame_data(buf, &[*Self::PREFIX])?;
        let s = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&s[1..end]);

        Ok(s.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_i64_encode() {
        assert_eq!(0.encode(), b":+0\r\n");
        assert_eq!(1.encode(), b":+1\r\n");
        assert_eq!((-1).encode(), b":-1\r\n");
        assert_eq!((1_000).encode(), b":+1000\r\n");
    }

    #[test]
    fn test_i64_decode() -> Result<()> {
        let mut buf = BytesMut::from(&b":+1000\r\n"[..]);
        let ret = i64::decode(&mut buf)?;
        assert_eq!(ret, 1000);

        let mut buf = BytesMut::from(&b":-1000\r\n"[..]);
        let ret = i64::decode(&mut buf)?;
        assert_eq!(ret, -1000);

        let mut buf = BytesMut::from(&b":abc\r\n"[..]);
        let ret = i64::decode(&mut buf);
        assert_eq!(
            ret.unwrap_err(),
            RespError::ParseIntError("abc".parse::<i64>().unwrap_err())
        );

        Ok(())
    }
}
