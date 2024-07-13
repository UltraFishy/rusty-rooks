use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_client(mut stream: TcpStream) {
     let mut buffer = [0; 512];
     loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(_) => {
                if let Err(e) = stream.write_all(&buffer).await {
                    eprintln!("Failed to send data: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
     }
}