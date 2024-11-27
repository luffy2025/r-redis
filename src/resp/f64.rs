use crate::RespEncode;

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

impl RespF64 {
    pub fn new(f: f64) -> Self {
        RespF64(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_encode() {
        assert_eq!(RespF64(0.0).encode(), b",+0e0\r\n");
        assert_eq!(RespF64(1.0).encode(), b",+1\r\n");
        assert_eq!(RespF64(-1.0).encode(), b",-1\r\n");
        assert_eq!(RespF64(1.23456e+8).encode(), b",+1.23456e8\r\n");
        assert_eq!(RespF64(-1.23456e+8).encode(), b",-1.23456e8\r\n");
        assert_eq!(RespF64(-1.23456e-9).encode(), b",-1.23456e-9\r\n");
    }
}
