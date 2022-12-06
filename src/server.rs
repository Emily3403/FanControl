use std::os::unix::net::UnixListener;
use std::thread::sleep;
use std::time::Duration;
use log::debug;
use crate::{ClientMessage, get_message_from_client, ServerMessage};
use crate::ipc::send_message_to_client;

pub fn server_handle_messages(server: &UnixListener) {
    for stream in server.incoming() {
        let Ok(stream) = stream else {
            break;
        };

        debug!("Server: Got a new connection!");
        let Ok(client_message) = get_message_from_client(&stream) else {
            break;
        };

        debug!("Message is of Type: {:?}", client_message);

        match client_message {
            ClientMessage::IsAlive => send_message_to_client(&stream, ServerMessage::Ok).unwrap_or_default(),
            ClientMessage::Test => todo!(),

        };

    }
}