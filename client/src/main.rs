
use colored::Colorize;
use inquire::{InquireError, Select};
use termion::{clear, cursor};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use dotenv::from_filename;
use std::env;
use spinners::{Spinner, Spinners};

use client::team::Team;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    welcome();

    let mut team = None;

    while team.is_none() {
        team = inquire_team();
    }

    let team: Team = team.unwrap();

    from_filename("../.env").ok();
    
    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT not set");
    let server_host = env::var("SERVER_HOST").expect("SERVER_HOST not set");
    // let auth_key = env::var("AUTH_KEY").expect("AUTH_KEY not set");

    let server_address = format!("{}:{}", server_host, server_port); 
    let mut socket = TcpStream::connect(server_address).await?;

    // socket.write_all(auth_key.as_bytes()).await?;

    let team_bytes = bincode::serialize(&team).unwrap(); // Serialize the team enum
    socket.write_all(&team_bytes).await?; // Send serialized data to server

    let start = false;
    let mut sp = Spinner::new(Spinners::Dots9, "Waiting for game to start ...".into());
    while start {
        wait_for_start(&mut socket).await;
    }
    sp.stop();

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await?;
    println!("Server response: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}

fn welcome() -> () {
    println!("{}{}", clear::All, cursor::Goto(1, 1));
    println!("Welcome to {}", "Rusty Rooks".truecolor(183, 65, 14));
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

async fn wait_for_start(socket: &mut TcpStream) -> Option<bool> {
    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await.expect("Failed to read data");
    let start: bool = bincode::deserialize(&buf[..n]).expect("Failed to deserialize data");

    Some(start)
}

