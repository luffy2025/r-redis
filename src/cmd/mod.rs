use crate::cmd::get::Get;
use crate::cmd::hget::HGet;
use crate::cmd::hset::HSet;
use crate::cmd::set::Set;
use crate::{RespArray, RespFrame};
use thiserror::Error;

mod get;
mod hget;
mod hset;
mod set;

pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    // HSetAll(hset_all::HSetAll),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CommandError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidCmd(String),
    #[error("Invalid frame args: {0}")]
    InvalidArgs(String),
    #[error("From utf8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

pub trait Executor {
    fn execute(&self) -> Result<RespFrame, CommandError>;
}

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        let frame = match value {
            RespFrame::Array(array) => array,
            _ => return Err(CommandError::InvalidFrame("Invalid frame type".to_string())),
        };

        match frame.first() {
            Some(RespFrame::BulkString(cmd)) => match cmd.as_ref() {
                b"get" => Ok(Command::Get(Get::try_from(frame)?)),
                b"set" => Ok(Command::Set(Set::try_from(frame)?)),
                b"hget" => Ok(Command::HGet(HGet::try_from(frame)?)),
                b"hset" => Ok(Command::HSet(HSet::try_from(frame)?)),
                // b"hset_all" => Ok(Command::HSetAll(hset_all::HSetAll::new(cmd))),
                _ => Err(CommandError::InvalidCmd("Invalid command".to_string())),
            },
            _ => Err(CommandError::InvalidArgs("Invalid arguments".to_string())),
        }
    }
}

pub(crate) fn validate_command(
    cmd: &RespArray,
    names: &[&'static str],
    args_len: usize,
) -> Result<(), CommandError> {
    if cmd.len() < names.len() + args_len {
        return Err(CommandError::InvalidArgs("Invalid arguments".to_string()));
    }
    validate_command_names(names, cmd)?;
    Ok(())
}

fn validate_command_names(names: &[&'static str], cmds: &RespArray) -> Result<(), CommandError> {
    for (name, arg) in names.iter().zip(cmds.iter()) {
        match arg {
            RespFrame::BulkString(cmd) => {
                if cmd.to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCmd(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd)
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidArgs(format!(
                    "Invalid arguments: expected {}, got {:?}",
                    name, arg
                )))
            }
        }
    }
    Ok(())
}

pub(crate) fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value
        .data
        .into_iter()
        .skip(start)
        .collect::<Vec<RespFrame>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BulkString, RespDecode};
    use anyhow::Result;

    #[test]
    fn test_extract_args() -> Result<()> {
        let mut cmd = bytes::BytesMut::from(
            &b"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"[..],
        );
        let cmd = RespArray::decode(&mut cmd)?;
        let args = extract_args(cmd, 1)?;
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], BulkString::new(b"map".to_vec()).into());
        assert_eq!(args[1], BulkString::new(b"hello".to_vec()).into());
        assert_eq!(args[2], BulkString::new(b"world".to_vec()).into());
        Ok(())
    }
}