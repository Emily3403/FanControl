#![feature(let_else)]

mod client;
mod ipc;
mod server;
mod strategies;
mod utils;

use std::thread::sleep;
use std::time::Duration;

use crate::client::do_all_client_actions;
use crate::ipc::{
    connect_as_client, connect_as_server, get_message_from_client, ClientMessage, ServerMessage,
};
use crate::server::{get_current_status, server_handle_messages};
use crate::strategies::Strategy;
use clap::{arg, Command};
use log::{debug, info};

pub fn cli() -> Command {
    Command::new("fanctl")
        .about("A FanControl Plugin for the Framework Laptop")
        .subcommand(
            Command::new("swap")
                .about("Swaps the current strategy")
                .arg(arg!(<STRATEGY> "The new strategy to apply"))
                .arg_required_else_help(true), // TODO: Validator
        )
        .subcommand(Command::new("status").about("Prints out the current status of the program"))
        .subcommand(
            Command::new("fanPercent")
                .about("Sets the fan to a percentage between 0 and 100")
                .arg(
                    arg!(<NUM> "The Percentage number between 0 and 100")
                        .value_parser(clap::value_parser!(u32)), // TODO: Make this between 0 and 100
                )
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("reset").about("Resets the fan percentage if one was set"))
}

fn main() {
    pretty_env_logger::init();
    let args = cli().get_matches();

    // First start the daemon if nothing is running.
    let listener = connect_as_server();
    match listener {
        Ok(ref it) => {
            info!("I could acquire the socket, I am the Server!");
            it.set_nonblocking(true).unwrap();
        }
        Err(_) => info!("I could _not_ acquire the socket, I am the Client!"),
    };

    let Ok(server) = listener else {
        // If there is already another socket connection go into client mode and communicate with it.
        println!("The socket already exists, going into client mode!");
        do_all_client_actions(args);
        return;
    };

    let mut strategy = Strategy::default();
    let mut status = get_current_status();

    loop {
        debug!("Changing temperature ...");
        sleep(Duration::from_secs(1));

        // 1. Check if settings have to be changed upon detection of certain programs / load

        // 2. Check the current Status

        // 3. Apply the new FanSpeed / PowerProfile in accordance with the current parameters

        // 4. Respond to messages
        server_handle_messages(&server, &mut strategy, &mut status);
    }
}
