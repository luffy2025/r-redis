use crate::RespEncode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNullBulkString;

// - null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(&self) -> Vec<u8> {
        "$-1\r\n".as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resp_null_bulk_string() {
        assert_eq!(RespNullBulkString.encode(), b"$-1\r\n");
    }
}
