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

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub mod client {
    use super::*;

    #[derive(Debug)]
    pub struct Client {
        pub team: Option<Team>,
        pub ip: String,
        pub stream: Arc<Mutex<TcpStream>>, // This will be used for communication
    }

    impl Client {
        pub fn new(ip: String, stream: Arc<Mutex<TcpStream>>) -> Self {
            Client {
                team: None,
                ip,
                stream,
            }
        }

        pub fn select_team() -> Team {
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

        pub fn send_team(&self, team: Team) {
            let message = format!("Selected team: {:?}", team);
            let mut stream = self.stream.lock().unwrap();
            stream.write_all(message.as_bytes()).unwrap();
        }
    }

    #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

        pub fn handle_client(
            stream: Arc<Mutex<TcpStream>>,
            clients: Arc<Mutex<Vec<Client>>>,
            ip: String,
        ) {
            let mut buf = [0; 1024];

            // Simulate client joining
            let mut client = Client::new(ip.clone(), Arc::clone(&stream));

            // Prompt the client to choose a team synchronously
            let team = Client::select_team();
            client.team = Some(team);

            // Send the selected team to the server
            client.send_team(team);

            clients.lock().unwrap().push(client);

            loop {
                let n = {
                    let mut stream = stream.lock().unwrap();
                    stream.read(&mut buf).unwrap()
                };

                if n == 0 {
                    println!("Client {} disconnected", ip);
                    return;
                }

                println!("Received from {}: {:?}", ip, &buf[..n]);

                // Echo message back to client
                {
                    let mut stream = stream.lock().unwrap();
                    stream.write_all(&buf[..n]).unwrap();
                }
            }
        }
    }
}
