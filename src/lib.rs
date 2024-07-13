/*
13th July 2024
Dan Bretherton
===================================================================
               _  _  _         _  _  _         _  _  _
              | || || |       | || || |       | || || |
              |_______|       |_______|       |_______|
              \__ ___ /       \__ ___ /       \__ ___ /
               |___|_|         |___|_|         |___|_|
               |_|___|         |_|___|         |_|___|
               |___|_|         |___|_|         |___|_|
              (_______)       (_______)       (_______)
              /_______\       /_______\       /_______\
===================================================================

The communal chess voting application
*/

use std::sync::Arc;
use termion::event::Key;
use termion::input::TermRead;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub mod client {
    use super::*;

    #[derive(Debug)]
    pub struct Client {
        pub team: Option<Team>,
        pub ip: String,
        pub stream: Arc<Mutex<TcpStream>>, // This will be used for communication
    }

    impl Client {
        pub fn new(team: Option<Team>, ip: String, stream: Arc<Mutex<TcpStream>>) -> Self {
            Client { team, ip, stream }
        }

        pub fn select_team(&mut self) -> Team {
            use std::io::{self, Write};
            use termion::event::Key;
            use termion::input::TermRead;

            let stdin = io::stdin();
            let mut stdout = io::stdout();

            write!(stdout, "Choose your team: [White] [Black]").unwrap();
            stdout.flush().unwrap();

            let mut selected_team = Team::WHITE;

            for key in stdin.keys() {
                match key.unwrap() {
                    Key::Left | Key::Right => {
                        selected_team = if selected_team == Team::WHITE {
                            Team::BLACK
                        } else {
                            Team::WHITE
                        };
                        write!(
                            stdout,
                            "\rChoose your team: [{}] [{}]",
                            if selected_team == Team::WHITE {
                                "White"
                            } else {
                                "White"
                            },
                            if selected_team == Team::BLACK {
                                "Black"
                            } else {
                                "Black"
                            }
                        )
                        .unwrap();
                        stdout.flush().unwrap();
                    }
                    Key::Char('\n') => {
                        break;
                    }
                    _ => {}
                }
            }

            selected_team
        }

        pub async fn send_team(&self) {
            let message = format!("Selected team: {:?}", self.team.as_ref().unwrap());
            let mut stream = self.stream.lock().await;
            stream.write_all(message.as_bytes()).await.unwrap();
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum Team {
        WHITE,
        BLACK,
    }
}

pub mod server {
    use super::*;
    use client::{Client, Team};

    #[derive(Debug)]
    pub struct Server {
        pub port: u16,
        pub ip: String,
        pub clients: Arc<Mutex<Vec<Client>>>,
    }

    impl Server {
        pub fn new(port: u16, ip: String) -> Self {
            Server {
                port,
                ip,
                clients: Arc::new(Mutex::new(Vec::new())),
            }
        }

        async fn broadcast_to_team(&self, team: &Team, message: &str) {
            let clients = self.clients.lock().await;
            for client in clients.iter() {
                if let Some(client_team) = &client.team {
                    if client_team == team {
                        let stream = Arc::clone(&client.stream);
                        let message = message.to_string();
                        tokio::spawn(async move {
                            let mut stream = stream.lock().await;
                            stream.write_all(message.as_bytes()).await.unwrap();
                        });
                    }
                }
            }
        }

        async fn broadcast_to_all(&self, message: &str) {
            let clients = self.clients.lock().await;
            for client in clients.iter() {
                let stream = Arc::clone(&client.stream);
                let message = message.to_string();
                tokio::spawn(async move {
                    let mut stream = stream.lock().await;
                    stream.write_all(message.as_bytes()).await.unwrap();
                });
            }
        }
    }
}
