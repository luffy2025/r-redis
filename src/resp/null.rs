use crate::RespEncode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RespNull;

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(&self) -> Vec<u8> {
        "$_\r\n".as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resp_null() {
        assert_eq!(RespNull.encode(), b"$_\r\n");
    }
}
