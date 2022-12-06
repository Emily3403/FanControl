use std::io::Error;
use std::os::unix::net::UnixListener;
use std::thread::sleep;
use std::time::Duration;
use log::debug;
use crate::{ClientMessage, get_message_from_client, ServerMessage};
use crate::ipc::{send_message_to_client, Status};
use crate::strategies::Strategy;

fn get_current_status() -> Status {
    todo!()
}

fn swap_current_strategy(strategy: Strategy) -> Result<(), Error> {
    todo!()
}

fn reset_fan_percentage() -> Result<(), Error> {
    todo!()
}

fn set_fan_percentage(percentage: i32) -> Result<(), Error> {
    todo!()
} 

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

        let message = match client_message {
            ClientMessage::IsAlive => ServerMessage::Ok,
            ClientMessage::Status => ServerMessage::Status(get_current_status()),
            ClientMessage::Reset => if reset_fan_percentage().is_ok() {ServerMessage::Ok} else {ServerMessage::Err},
            ClientMessage::Swap(strategy) => if swap_current_strategy(strategy).is_ok() { ServerMessage::Ok } else { ServerMessage::Err }
            ClientMessage::SetFanPercent(percent) => if set_fan_percentage(percent).is_ok() { ServerMessage::Ok } else { ServerMessage::Err }
        };

        let Ok(_) = send_message_to_client(&stream, message) else {
            break;
        };
    }
}