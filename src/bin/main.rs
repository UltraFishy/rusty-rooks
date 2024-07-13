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


use tokio::{
    net::TcpListener,
    sync::broadcast
};

use community::*;

/// We will implement the following algorithm for our server in our main function:
/// 1. Create a broadcast channel. It will be shared by all the client tasks.
/// 2. Create `TcpListener` and bind to an address & port.
/// 3. Loop:
///    - Accept socket connection, and get its `TCPStream`.
///    - Use `tokio::spawn()` to spawn a task to handle this client connection and its
///      `TCPStream`.
///
/// In the task that handles the connection:
/// 1. Get `BufReader` & `BufWriter` from the `TCPStream`. The reader and writer allow us to
///    read data from and write data to the client socket.
/// 2. Loop:
///    - Use `tokio::select!` to concurrently:
///       - Read from broadcast channel (via `recv()`):
///          - Send the message to the client (only if it is from a different client) over the
///            socket (use `BufWriter` to write the message).
///       - Read from socket (via `BufReader::read_line()`):
///          - Read `incoming` from reader.
///          - Call `process(incoming)` and generate `outgoing`.
///          - Send `incoming` message to other connected clients (via the broadcast channel).
#[tokio::main]
pub async fn main() -> IOResult<()> {
    let addr = "127.0.0.1:3000";

    // Start logging.
    femme::start();

    // Create TCP listener.
    let tcp_listener = TcpListener::bind(addr).await?;
    log::info!("Server is ready to accept connections on {}", addr);

    // Create channel shared among all clients that connect to the server loop.
    let (tx, _) = broadcast::channel::<MsgType>(10);

    // Server loop.
    loop {
        // Accept incoming socket connections.
        let (tcp_stream, socket_addr) = tcp_listener.accept().await?;

        let tx = tx.clone();
        tokio::spawn(async move {
            let result = handle_client_task(tcp_stream, tx, socket_addr).await;
            match result {
                Ok(_) => {
                    log::info!("handle_client_task() terminated gracefully")
                }
                Err(error) => log::error!("handle_client_task() encountered error: {}", error),
            }
        });
    }
}

