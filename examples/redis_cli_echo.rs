#![allow(unused_variables)]
#![allow(dead_code)]

/// 与 redis-cli 实现简单的交互
///     - 了解 redis 底层的报文格式
///     - 观测 redis-cli 发送的报文格式
/// 总是回复 redis-cli 命令执行成功报文: `+OK\r\n`
/// 
/// Commands Protocol:
///  - 获取指令列表:  COMMAND DOCS       = `*2\r\n$7\r\ncommand\r\n$4\r\nDOCS\r\n`
///  - 设置缓存      set name zhangsan  = `*3\r\n$3\r\nset\r\n$4\r\nname\r\n$8\r\nzhangsan\r\n`
///  - 获取缓存      get name           = `*2\r\n$3\r\nget\r\n$4\r\nname\r\n`

use std::io;
use anyhow::Result;
use tracing::{info, warn};
use rudis::logger::logger_init;
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};

const BUF_SIZE: usize = 4096;
const ADDRESS: &str = "0.0.0.0:6379";
const OK: &[u8;5] = b"+OK\r\n";


#[tokio::main]
async fn main() -> Result<()>{
    // init logger
    logger_init();
    // create tcp listener
    let listener = TcpListener::bind(ADDRESS).await?;
    info!("Rudis server startup.");

    // loop accept client conn
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
            Ok(0) => {
                break;
            },
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
                // 输出客户端的命令报文
                info!("{:?}", line);
                // 响应执行成功报文
                stream.write_all(OK).await?;
            }
        }
    }
    warn!("Client closed from: {}", stream.peer_addr()?);
    Ok(())
}