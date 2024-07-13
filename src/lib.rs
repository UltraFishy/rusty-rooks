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

use std::{fmt::format, net::SocketAddr};

use r3bl_rs_utils_core::friendly_random_id;
use r3bl_tui::ColorWheel;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, ReadHalf},
    net::{tcp::WriteHalf, TcpStream},
    sync::broadcast::{error::RecvError, Sender},
};

pub type IOResult<T> = std::io::Result<T>;

#[derive(Debug, Clone)]
pub struct MsgType {
    pub socket_addr: SocketAddr,
    pub payload: String,
    pub from_id: String,
}

pub async fn handle_client_task(
    mut tcp_stream: TcpStream,
    tx: Sender<MsgType>,
    socket_addr: SocketAddr,
) -> IOResult<()> {
    log::info!("Handle socket connection from client");

    let id = friendly_random_id::generate_friendly_random_id();
    let mut rx = tx.subscribe();

    // Set up buf reader and writer.
    let (reader, writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(writer);

    // Send welcome message to client w/ ids.
    let welcome_msg_for_client =
        ColorWheel::lolcat_into_string(&format!("addr: {}, id: {}\n", socket_addr, id));
    
    let _ = send_to_client(&mut writer, welcome_msg_for_client).await;

    let mut incoming = String::new();

    loop {
        let tx = tx.clone();
        tokio::select! {
            // Read from broadcast channel.
            result = rx.recv() => {
                read_from_broadcast_channel(result, socket_addr, &mut writer, &id).await?;
            }

            // Read from socket.
            network_read_result = reader.read_line(&mut incoming) => {
                let num_bytes_read: usize = network_read_result?;
                // EOF check.
                if num_bytes_read == 0 {
                    break;
                }
                handle_socket_read(num_bytes_read, &id, &incoming, &mut writer, tx, socket_addr).await?;
                incoming.clear();
            }
        }
    }

    Ok(())
}

pub async fn read_from_broadcast_channel(
    result: Result<MsgType, RecvError>,
    socket_addr: SocketAddr,
    writer: &mut BufWriter<WriteHalf<'_>>,
    id: &str,
) -> IOResult<()> {
    match result {
        Ok(it) => {
            let msg: MsgType = it;
            log::info!("[{}]: channel: {:?}", id, msg);
            if msg.socket_addr != socket_addr {
                writer.write(msg.payload.as_bytes()).await?;
                writer.flush().await?;
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
        }
    }

    Ok(())
}

pub async fn handle_socket_read(
    num_bytes_read: usize,
    id: &str,
    incoming: &str,
    writer: &mut BufWriter<WriteHalf<'_>>,
    tx: Sender<MsgType>,
    socket_addr: SocketAddr,
) -> IOResult<()> {
    log::info!(
        "[{}]: incoming: {}, size: {}",
        id,
        incoming.trim(),
        num_bytes_read
    );

    // Process incoming -> outgoing.
    let outgoing = process(&incoming);

    // outgoing -> Writer.
    writer.write(outgoing.as_bytes()).await?;
    writer.flush().await?;

    // Broadcast outgoing to the channel.
    let _ = tx.send(MsgType {
        socket_addr,
        payload: incoming.to_string(),
        from_id: id.to_string(),
    });

    log::info!(
        "[{}]: outgoing: {}, size: {}",
        id,
        outgoing.trim(),
        num_bytes_read
    );

    Ok(())
}

pub fn process(incoming: &str) -> String {
    // Remove new line from incoming.
    let incoming_trimmed = format!("{}", incoming.trim());
    // Colorize it.
    let outgoing = ColorWheel::lolcat_into_string(&incoming_trimmed);
    // Add new line back to outgoing.
    format!("{}\n", outgoing)
}

pub async fn send_to_client(writer: &mut BufWriter<WriteHalf<'_>>, msg: String) -> IOResult<()> {
    writer.write(msg.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

#[derive(Debug)]
pub enum Team {
    WHITE,
    BLACK
}

pub fn welcome_and_team_selection(writer: &mut BufWriter<WriteHalf<'_>>, reader: &mut BufReader<ReadHalf<String>>) {
    let clear = "\x1B[2J";
    let welcome = format!("Welcome to {}", ColorWheel::lolcat_into_string("Rusty Rooks!!!\n"));
    let select = format!("Please select a team: \n{} White\n{} Black\n>", ColorWheel::lolcat_into_string("1."), ColorWheel::lolcat_into_string("2."));
    let msg = format!("{}{} {}", clear, welcome, select);

    // Send Welcome
    let _ = send_to_client(writer, msg);

    // Get Team Back
    while true {
        if 
    }

}