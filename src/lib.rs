use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;
use clap::ArgMatches;

const SOCKET_ADDR: &'static str = "/tmp/fwctrl.sock";

pub fn connect_to_socket() -> Result<UnixListener, std::io::Error> {
    let listener = UnixListener::bind(SOCKET_ADDR)?;
    Ok(listener)
}


pub fn do_all_client_actions(args: ArgMatches) -> () {
    let client = UnixStream::connect(SOCKET_ADDR)

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
    }
}