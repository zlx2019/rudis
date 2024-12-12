use std::io;

use anyhow::Result;
use rudis::logger::logger_init;
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;
const ADDRESS: &str = "0.0.0.0:6379";

/// Rust Reids
#[tokio::main]
async fn main() -> Result<()>{
    // init logger
    logger_init();
    let listener = TcpListener::bind(ADDRESS).await?;
    info!("Rudis server startup.");
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move{
            if let Err(e) = process_redis_conn(stream).await {
                warn!("Error processing conn with: {}: {:?}", addr, e);
            }
        });
    }
}

/// process redis conn
async fn process_redis_conn(mut stream: TcpStream) -> Result<()>{
    loop {
        // 连接是否可读
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            // 连接关闭
            Ok(0) => break,
            // 读取产生错误
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    // 连接阻塞挂起.
                    continue;
                }
                return Err(e.into());
            },
            // 读取到了数据
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                // 响应 Redis Cli 连接报文
                stream.write_all(b"+OK\r\n").await?;
            }
        }
    }
    Ok(())
}