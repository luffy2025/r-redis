use crate::resp::f64::RespF64;
use crate::resp::map::RespMap;
use crate::resp::set::RespSet;
use crate::resp::simple_error::SimpleError;
use crate::{BulkString, RespArray, RespDecode, RespNull, SimpleString};
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    Array(RespArray),
    Null(RespNull),
    Boolean(bool),
    Double(RespF64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not compete")]
    NotComplete,
    #[error("Frame is empty")]
    Empty,
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static u8 = &b'_';
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(&SimpleString::PREFIX) => Ok(SimpleString::decode(buf)?.into()),
            Some(&SimpleError::PREFIX) => Ok(SimpleError::decode(buf)?.into()),
            Some(&i64::PREFIX) => Ok(i64::decode(buf)?.into()),
            Some(&BulkString::PREFIX) => Ok(BulkString::decode(buf)?.into()),
            Some(&RespArray::PREFIX) => Ok(RespArray::decode(buf)?.into()),
            Some(&RespSet::PREFIX) => Ok(RespSet::decode(buf)?.into()),
            Some(&RespMap::PREFIX) => Ok(RespMap::decode(buf)?.into()),
            Some(&RespNull::PREFIX) => Ok(RespNull::decode(buf)?.into()),
            Some(&bool::PREFIX) => Ok(bool::decode(buf)?.into()),
            Some(&RespF64::PREFIX) => Ok(RespF64::decode(buf)?.into()),
            Some(e) => Err(RespError::InvalidFrame(
                format!("Invalid prefix: {}", e.to_ascii_lowercase()).to_string(),
            )),
            None => Err(RespError::Empty),
        }
    }
}
