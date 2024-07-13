use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::env;
use dotenv::from_filename;

use server::team::Team;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    from_filename("../.env").ok();

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not set");
    let server_host = env::var("SERVER_HOST").expect("SERVER_HOST not set");
    // let auth_key = env::var("AUTH_KEY").expect("AUTH_KEY not set");

    let server_address = format!("{}:{}", server_host, server_port);

    let listener = TcpListener::bind(server_address).await?;
    println!("Server listening on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("New client connected from {:?}", socket.peer_addr().unwrap());

            let mut buf = vec![0; 1024];
            let n = socket.read(&mut buf).await.expect("Failed to read data");

            let team: Team = bincode::deserialize(&buf[..n]).expect("Failed to deserialize data"); // Deserialize team enum
            println!("Received team: {:?}", team);

            // Process the team choice as needed

            socket.write_all(b"Hello from server!\n").await.expect("Failed to write response");
            println!("Response sent to {:?}", socket.peer_addr().unwrap());
        });
    }
}