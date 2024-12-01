use anyhow::Result;
use r_redis::{network, Backend};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    let backend = Backend::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        let cloned_backend = backend.clone();
        tokio::spawn(async move {
            match network::stream_handler(stream, cloned_backend).await {
                Ok(_) => info!("Connection from {} closed", raddr),
                Err(e) => warn!("Connection from {} error: {:?}", raddr, e),
            }
        });
    }
}
