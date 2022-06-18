pub mod solve;
pub mod play;

use clap::{Command};

fn cli() -> clap::Command<'static> {
    Command::new("mastermind")
        .version("0.1.0")
        .author("Leo Blume <leoblume@gmx.de>")
        .about("Play or solve mastermind.")
        .subcommand_required(true)
        .subcommand(Command::new("play").about("Play mastermind interactively"))
        .subcommand(Command::new("solve").about("Solve a specific mastermind situation"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("play", _sub_matches)) => {
            play::run();
        },
        Some(("solve", _sub_matches)) => {
            solve::run();
        },
        _ => unreachable!()
    }
}
