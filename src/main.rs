mod strategies;

use strategies::{Strategy};

use clap::{arg, Args, Command, Parser, Subcommand, ValueEnum};



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
    println!("Hello, world!");
}
