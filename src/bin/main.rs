use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use community::{client::Client, server::Server};

fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let server = Server::new(port, "127.0.0.1".to_string());

    let listener = TcpListener::bind(format!("{}:{}", server.ip, server.port)).unwrap();
    println!("Server running on {}:{}", server.ip, server.port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New client: {:?}", stream.peer_addr().unwrap());

        let client_ip = stream.peer_addr().unwrap().to_string();
        let server_clients = Arc::clone(&server.clients);
        let stream = Arc::new(Mutex::new(stream));

        thread::spawn(move || {
            Server::handle_client(stream, server_clients, client_ip);
        });
    } 
}


