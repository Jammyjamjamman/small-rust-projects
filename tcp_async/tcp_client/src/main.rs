use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // 4

async fn clientboi() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    // stream.write_all(b"Another message :D").await?;
    stream.write_all(b"exit please").await?;

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Buf says: {}", String::from_utf8_lossy(&buf));
    Ok(())
}

fn main() {
    let tcp_client_task = task::spawn(clientboi());
    let _ = task::block_on(tcp_client_task);
}
