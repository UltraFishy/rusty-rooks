use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use community::{client::Client, server::Server};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let server = Server::new(port, "127.0.0.1".to_string());

    let listener = TcpListener::bind(format!("{}:{}", server.ip, server.port))
        .await
        .unwrap();
    println!("Server running on {}:{}", server.ip, server.port);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New client: {:?}", addr);

        let client_ip = addr.to_string();
        let server_clients = Arc::clone(&server.clients);
        let stream = Arc::new(Mutex::new(socket));

        tokio::spawn(async move {
            handle_client(stream, server_clients, client_ip).await;
        });
    }
}

async fn handle_client(
    stream: Arc<Mutex<TcpStream>>,
    clients: Arc<Mutex<Vec<Client>>>,
    ip: String,
) {
    let mut buf = [0; 1024];

    // Simulate client joining with a team
    let mut client = Client::new(None, ip.clone(), Arc::clone(&stream));

    // Prompt the client to choose a team synchronously
    let team = client.select_team();
    client.team = Some(team);

    // Send the selected team to the server asynchronously
    client.send_team().await;

    clients.lock().await.push(client);

    loop {
        let n = {
            let mut stream = stream.lock().await;
            stream.read(&mut buf).await.unwrap()
        };

        if n == 0 {
            println!("Client {} disconnected", ip);
            return;
        }

        println!("Received from {}: {:?}", ip, &buf[..n]);

        // Echo message back to client
        {
            let mut stream = stream.lock().await;
            stream.write_all(&buf[..n]).await.unwrap();
        }
    }
}
