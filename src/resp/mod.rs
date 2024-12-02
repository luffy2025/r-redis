pub use crate::resp::array::RespArray;
pub use crate::resp::bulk_string::BulkString;
use crate::resp::f64::RespF64;
pub use crate::resp::frame::RespError;
pub use crate::resp::frame::RespFrame;
use crate::resp::map::RespMap;
pub use crate::resp::null::RespNull;
use crate::resp::set::RespSet;
use crate::resp::simple_error::SimpleError;
pub use crate::resp::simple_string::SimpleString;
use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

mod array;
mod bool;
mod bulk_string;
mod f64;
mod frame;
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
