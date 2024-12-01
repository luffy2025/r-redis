use crate::cmd::{Command, CommandExecutor};
use crate::{Backend, RespDecode, RespEncode, RespError, RespFrame};
use anyhow::Result;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(Ok(req)) => {
                info!(
                    "Received request: {:?}",
                    String::from_utf8_lossy(req.encode().as_slice())
                );
                let resp = request_handler(RedisRequest {
                    frame: req,
                    backend: backend.clone(),
                })
                .await?;
                info!(
                    "Send response: {:?}",
                    String::from_utf8_lossy(resp.frame.encode().as_slice())
                );
                framed.send(resp.frame).await?;
            }
            Some(Err(e)) => return Err(e),
            None => {
                info!("Connection closed");
                return Ok(());
            }
        }
    }
}

async fn request_handler(req: RedisRequest) -> Result<RedisResponse> {
    let cmd = Command::try_from(req.frame)?;
    info!("Execute command: {:?}", cmd);
    let resp = cmd.execute(&req.backend)?;
    Ok(RedisResponse { frame: resp })
}

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

#[derive(Debug)]
struct RedisResponse {
    frame: RespFrame,
}

#[derive(Debug)]
struct RespFrameCodec;

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> Result<()> {
        let data = item.encode();
        dst.extend_from_slice(&data);
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(RespError::Empty) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
