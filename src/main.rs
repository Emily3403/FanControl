mod strategies;
mod lib;

use strategies::{Strategy};

use clap::{arg, Args, Command, Parser, Subcommand, ValueEnum};
use crate::lib::{connect_to_socket, do_all_client_actions};


fn cli() -> Command {
    Command::new("fanctrl")
        .about("A FanControl Plugin for the Framework Laptop")

        .subcommand(
            Command::new("swap")
                .about("Swaps the current strategy")
                .arg(arg!(<STRATEGY> "The new strategy to apply"))

                .arg_required_else_help(true)
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
}

fn main() {
    let args = cli().get_matches();

    // First start the daemon if nothing is running. Thus, the following client code will always succeed.
    let listener = connect_to_socket();

    // If we are spawned in ClientMode, this function will talk with the daemon and then exit the program.
    do_all_client_actions(args);

    loop {
        // 1. Check if settings have to be changed due to IPC

        // 2. Check if settings have to be changed upon detection of certain programs / load

        // 3. Check the current temperatures

        // 4. Apply the new FanSpeed / PowerProfile in accordance with the current parameters


    }


}
