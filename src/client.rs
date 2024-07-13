use team::Team;

use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut team = None;

    while team.is_none() {
        team = inquire_team();
    }

    dotenv().ok();
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not set");
    let auth_key = env::var("AUTH_KEY").expect("AUTH_KEY not set");

    let server_address = format!("server:{}", server_port);
    let mut socket = TcpStream::connect(server_address).await?;
    println!("Connected to server!");

    socket.write_all(auth_key.as_bytes()).await?;

    let team_bytes = bincode::serialize(&team.unwrap()).unwrap(); // Serialize the team enum
    socket.write_all(&team_bytes).await?; // Send serialized data to server

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("Server response: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}

fn inquire_team() -> Option<Team> {
    let options: Vec<&str> = vec!["White", "Black"];
    let ans: Result<&str, InquireError> =
        Select::new("Please select your team: ", options).prompt();

    match ans {
        Ok(choice) => match choice {
            "White" => Some(Team::WHITE),
            "Black" => Some(Team::BLACK),
            _ => panic!("Oh no..."),
        },
        Err(e) => {
            println!("There was an error: {}, please try again", e);
            None
        }
    }
}

pub mod team {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Team {
        WHITE,
        BLACK,
    }
}
