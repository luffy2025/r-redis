pub use crate::resp::array::RespArray;
pub use crate::resp::bulk_string::BulkString;
use crate::resp::f64::RespF64;
use crate::resp::map::RespMap;
pub use crate::resp::null::RespNull;
use crate::resp::set::RespSet;
use crate::resp::simple_error::SimpleError;
pub use crate::resp::simple_string::SimpleString;
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

mod array;
mod bool;
mod bulk_string;
mod f64;
mod i64;
mod map;
mod null;
mod set;
mod simple_error;
mod simple_string;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static u8;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

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

pub fn extract_end_and_length(
    buf: &mut BytesMut,
    prefix: &[u8],
) -> Result<(usize, usize), RespError> {
    let end = extract_simple_frame_data(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[1..end]);
    let len = s.parse::<usize>()?;
    Ok((end, len))
}

pub fn extract_simple_frame_data(buf: &mut BytesMut, prefix: &[u8]) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix) {
        return Err(RespError::InvalidFrameType("SimpleError".into()));
    }

    let mut end = 0;
    for i in 0..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            end = i;
            break;
        }
    }
    if end == 0 {
        return Err(RespError::NotComplete);
    }
    Ok(end)
}

pub fn is_combine_complete(buf: &[u8], len: usize) -> Result<(), RespError> {
    is_fixed_complete(buf)?;

    let mut count = 0;
    let mut i = 0;
    while count < len {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
        }
        i += 1;
    }
    // The content in the item may contain \r\n, so here it is >=
    (count >= len).then_some(()).ok_or(RespError::NotComplete)
}

pub fn is_single_complete(buf: &[u8], len: usize) -> Result<(), RespError> {
    is_fixed_complete(buf)?;
    (buf.len() >= len + 4 + 2)
        .then_some(())
        .ok_or(RespError::NotComplete)
}

pub fn is_fixed_complete(buf: &[u8]) -> Result<(), RespError> {
    (buf.len() > 2)
        .then_some(())
        .ok_or(RespError::NotComplete)?;

    buf.ends_with(b"\r\n")
        .then_some(())
        .ok_or(RespError::NotComplete)
}
