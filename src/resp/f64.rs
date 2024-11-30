use crate::{is_fixed_complete, RespDecode, RespEncode, RespError};

#[derive(Debug, PartialEq)]
pub struct RespF64(f64);

impl Eq for RespF64 {}

impl PartialOrd for RespF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RespF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for RespF64 {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.0.abs() > 1e+8 || self.0.abs() < 1e-8 {
            format!(",{:+e}\r\n", self.0)
        } else {
            let sign = if self.0.is_sign_negative() { "" } else { "+" };
            format!(",{}{}\r\n", sign, self.0)
        };
        buf.extend_from_slice(ret.as_bytes());
        buf
    }
}

impl RespDecode for RespF64 {
    const PREFIX: &'static u8 = &b',';
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        is_fixed_complete(buf)?;

        let end = crate::extract_simple_frame_data(buf, &[*Self::PREFIX])?;
        let s = buf.split_to(end + 2);
        let s = String::from_utf8_lossy(&s[1..end]);

        Ok(RespF64(s.parse()?))
    }
}

impl RespF64 {
    pub fn new(f: f64) -> Self {
        RespF64(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_f64_encode() {
        assert_eq!(RespF64(0f64).encode(), b",+0e0\r\n");
        assert_eq!(RespF64(10f64).encode(), b",+10\r\n");
        assert_eq!(RespF64(-10f64).encode(), b",-10\r\n");
        assert_eq!(RespF64(1.23456e+8).encode(), b",+1.23456e8\r\n");
        assert_eq!(RespF64(-1.23456e+8).encode(), b",-1.23456e8\r\n");
        assert_eq!(RespF64(1.23456e-9).encode(), b",+1.23456e-9\r\n");
        assert_eq!(RespF64(-1.23456e-9).encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_f64_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::from(&b",+0e0\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(0f64));

        let mut buf = bytes::BytesMut::from(&b",+10\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(10f64));

        let mut buf = bytes::BytesMut::from(&b",-10\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(-10f64));

        let mut buf = bytes::BytesMut::from(&b",+1.23456e8\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(1.23456e+8));

        let mut buf = bytes::BytesMut::from(&b",-1.23456e8\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(-1.23456e+8));

        let mut buf = bytes::BytesMut::from(&b",+1.23456e-9\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(1.23456e-9));

        let mut buf = bytes::BytesMut::from(&b",-1.23456e-9\r\n"[..]);
        let ret = RespF64::decode(&mut buf)?;
        assert_eq!(ret, RespF64(-1.23456e-9));

        Ok(())
    }
}
