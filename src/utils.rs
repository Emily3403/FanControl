use std::process::exit;
use clap::ArgMatches;
use crate::connect_as_client;


pub fn do_all_client_actions(args: ArgMatches) -> () {
    let client = connect_as_client();

    if match args.subcommand() {
        Some(("swap", sub_matches)) => {
            let strategy = sub_matches.get_one::<String>("STRATEGY").expect("required");

            println!("{:?}", strategy);
            true
        }

        Some(("status", _)) => {
            println!("Doing status!");
            true
        }

        Some(("fanPercent", sub_matches)) => {
            let percent = sub_matches.get_one::<u32>("NUM").expect("required");

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
