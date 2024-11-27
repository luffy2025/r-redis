use crate::RespEncode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNullArray;

// - null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(&self) -> Vec<u8> {
        "*-1\r\n".as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resp_null_array() {
        assert_eq!(RespNullArray.encode(), b"*-1\r\n");
    }
}
