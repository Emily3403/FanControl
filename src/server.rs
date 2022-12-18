use crate::ipc::{send_message_to_client, Status};
use crate::strategies::Strategy;
use crate::utils::{get_current_temp, Percentage};
use crate::{get_message_from_client, ClientMessage, ServerMessage};
use log::debug;
use std::io::Error;
use std::os::unix::net::UnixListener;

pub fn get_current_status() -> Status {
    let temp = get_current_temp();
    println!("{:?}", temp);
    todo!()
}

fn swap_current_strategy(strategy: Strategy) -> Result<(), Error> {
    todo!()
}

fn reset_fan_percentage() -> Result<(), Error> {
    todo!()
}

fn set_fan_percentage(percentage: Percentage) -> Result<(), Error> {
    todo!()
}

pub fn server_handle_messages(
    server: &UnixListener,
    current_strategy: &mut Strategy,
    current_status: &mut Status,
) {
    for stream in server.incoming() {
        let Ok(stream) = stream else {
            break;
        };

        // Attempt to get a message from the client
        debug!("Server: Got a new connection!");
        let Ok(client_message) = get_message_from_client(&stream) else {
            debug!("Client message was not OK!");
            break;
        };

        let message = match client_message {
            ClientMessage::IsAlive => ServerMessage::Ok,
            ClientMessage::Status => ServerMessage::Status(current_status.clone()),
            ClientMessage::Reset => {
                if reset_fan_percentage().is_ok() {
                    ServerMessage::Ok
                } else {
                    ServerMessage::Err
                }
            }
            ClientMessage::Swap(strategy) => {
                if swap_current_strategy(strategy).is_ok() {
                    ServerMessage::Ok
                } else {
                    ServerMessage::Err
                }
            }
            ClientMessage::SetFanPercent(percent) => {
                if set_fan_percentage(percent).is_ok() {
                    ServerMessage::Ok
                } else {
                    ServerMessage::Err
                }
            }
        };

        debug!("ClientMessage is of Type: {:?}", client_message);
        debug!("ServerMessage is of Type: {:?}", message);

        let Ok(_) = send_message_to_client(&stream, message) else {
            debug!("Client response was not OK!");
            break;
        };
    }
}
