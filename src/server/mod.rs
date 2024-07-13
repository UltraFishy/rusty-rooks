use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;

pub async fn start_server() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or("1234".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Could not bind");

    println!("Server running on port {}", port);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("New Client connected");
                tokio::spawn(crate::client::handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
}