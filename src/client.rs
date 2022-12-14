use crate::ipc::send_message_to_server;
use crate::utils::Percentage;
use crate::{connect_as_client, ClientMessage};
use clap::ArgMatches;
use std::process::exit;

pub fn do_all_client_actions(args: ArgMatches) -> () {
    let stream = connect_as_client().unwrap();

    if match args.subcommand() {
        Some(("swap", sub_matches)) => {
            let strategy = sub_matches.get_one::<String>("STRATEGY").expect("required");

            println!("{:?}", strategy);
            true
        }

        Some(("status", _)) => {
            println!("Doing status!");
            let _ = send_message_to_server(&stream, ClientMessage::IsAlive);
            true
        }

        Some(("fanPercent", sub_matches)) => {
            let percent = sub_matches.get_one::<Percentage>("NUM").expect("required");

            println!("{:?}", percent);
            true
        }

        None => {
            // Start in daemon mode
            false
        }

        _ => unreachable!(),
    } {
        exit(0);
    };

    // TODO: Acknowledge the response and if none is there kill the socket.
}
