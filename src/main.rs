#![feature(let_else)]

mod strategies;
mod utils;
mod ipc;

use std::borrow::Borrow;
use std::io::{Read, Write};
use std::thread::{sleep};
use std::time::Duration;


use clap::{arg, Command};
use lazy_static::lazy_static;
use log::{debug, info};
use crate::utils::{do_all_client_actions};
use crate::ipc::{connect_as_server, connect_as_client, ClientMessage, ServerMessage};

lazy_static! {
    static ref EXPECTED_MESSAGE_SIZE: usize =
        bincode::serialized_size(&ClientMessage::IsAlive).unwrap().try_into().unwrap();
}


pub fn cli() -> Command {
    Command::new("fanctl")
        .about("A FanControl Plugin for the Framework Laptop")

        .subcommand(
            Command::new("swap")
                .about("Swaps the current strategy")
                .arg(arg!(<STRATEGY> "The new strategy to apply"))

                .arg_required_else_help(true)
            // TODO: Validator
        )

        .subcommand(
            Command::new("status")
                .about("Prints out the current status of the program")
        )

        .subcommand(
            Command::new("fanPercent")
                .about("Sets the fan to a percentage between 0 and 100")
                .arg(arg!(<NUM> "The Percentage number between 0 and 100")
                         .value_parser(clap::value_parser!(u32))  // TODO: Make this between 0 and 100
                )
                .arg_required_else_help(true)
        )

        .subcommand(
            Command::new("reset")
                .about("Resets the fan percentage if one was set")
        )
}

fn main() {
    pretty_env_logger::init();
    let args = cli().get_matches();

    // First start the daemon if nothing is running.
    let listener = connect_as_server();
    match listener {
        Ok(_) => info!("I could acquire the socket, I am the Server!"),
        Err(_) => info!("I could _not_ acquire the socket, I am the Client!"),
    };

    let Ok(listener) = listener else {
        // If there is already another socket connection go into client mode and communicate with it.
        println!("The socket already exists, going into client mode!");
        do_all_client_actions(args);
        return;
    };


    loop {
        // 1. Check if settings have to be changed due to IPC

        // 2. Check if settings have to be changed upon detection of certain programs / load

        // 3. Check the current temperatures

        // 4. Apply the new FanSpeed / PowerProfile in accordance with the current parameters


        for stream in listener.incoming() {
            let Ok(mut stream) = stream else {
                break;
            };
            sleep(Duration::from_secs(1));

            debug!("Got a new connection!");

            let mut buf = vec![0; *EXPECTED_MESSAGE_SIZE];
            let Ok(_) = stream.read_exact(&mut buf) else {
                break;
            };

            println!("{:?}", buf);
            let s: ClientMessage = bincode::deserialize(&buf).unwrap();

            debug!("Message has contents: {:?}", s);

            if s == ClientMessage::IsAlive {
                let msg = bincode::serialize(&ServerMessage::Ok).unwrap();
                stream.write_all(&msg).unwrap();
                debug!("Sent back the message!");
            }
        }
    }
}
