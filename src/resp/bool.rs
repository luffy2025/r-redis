use crate::RespEncode;

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(&self) -> Vec<u8> {
        format!("#{}\r\n", if *self { "t" } else { "f" })
            .as_bytes()
            .to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_encode() {
        assert_eq!(true.encode(), b"#t\r\n");
        assert_eq!(false.encode(), b"#f\r\n");
    }
}
