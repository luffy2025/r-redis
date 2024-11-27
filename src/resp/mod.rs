use crate::resp::array::RespArray;
use crate::resp::bulk_string::BulkString;
use crate::resp::f64::RespF64;
use crate::resp::map::RespMap;
use crate::resp::null::RespNull;
use crate::resp::null_array::RespNullArray;
use crate::resp::null_bulk_string::RespNullBulkString;
use crate::resp::set::RespSet;
use crate::resp::simple_error::SimpleError;
use crate::resp::simple_string::SimpleString;
use anyhow::Result;
use enum_dispatch::enum_dispatch;

mod array;
mod bool;
mod bulk_string;
mod f64;
mod i64;
mod map;
mod null;
mod null_array;
mod null_bulk_string;
mod set;
mod simple_error;
mod simple_string;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<Option<RespFrame>>;
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    Null(RespNull),
    Boolean(bool),
    Double(RespF64),
    Map(RespMap),
    Set(RespSet),
}
