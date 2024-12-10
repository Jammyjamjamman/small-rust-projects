use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>; // 4

async fn listen() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut incoming = listener.incoming();
    
    while let Some(stream) = incoming.next().await {
        println!("Message incoming!");
        let stream = stream?;
        let (reader, writer) = &mut (&stream, &stream);
        // echo server
        let mut buf = vec![0u8; 1024];
        let datalen = reader.read(&mut buf).await?;
        if &buf[0..4] == b"exit" {
            writer.write_all(b"exciting!").await?;
            println!("exiting...");
            break;
        }
        println!("Buf says: {}", String::from_utf8_lossy(&buf));
        writer.write_all(&mut buf[..datalen]).await?;
    }
    Ok(())
}

fn main() {
    let tcp_server_task = task::spawn(listen());
    let _ = task::block_on(tcp_server_task);
}
