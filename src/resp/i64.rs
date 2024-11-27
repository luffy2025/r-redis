use crate::RespEncode;

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(&self) -> Vec<u8> {
        let sign = if *self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_encode() {
        assert_eq!(0.encode(), b":+0\r\n");
        assert_eq!(1.encode(), b":+1\r\n");
        assert_eq!((-1).encode(), b":-1\r\n");
        assert_eq!((1_000).encode(), b":+1000\r\n");
    }
}
