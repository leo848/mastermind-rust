pub mod play;
pub mod solve;

use clap::{Arg, Command};

fn cli() -> clap::Command<'static> {
    Command::new("mastermind")
        .version("0.1.0")
        .author("Leo Blume <leoblume@gmx.de>")
        .about("Play or solve mastermind.")
        .arg(
            Arg::new("no-clear")
                .long("no-clear")
                .short('C')
                .help("Don't clear the screen."),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("play")
                .about("Play mastermind interactively")
                .arg(
                    Arg::new("no-info")
                        .long("no-info")
                        .short('I')
                        .help("Don't show information."),
                )
                .arg(
                    Arg::new("show-possible")
                        .long("num")
                        .short('n')
                        .help("Show the remaining amount of possible codes."),
                ),
        )
        .subcommand(
            Command::new("solve")
                .about("Solve a specific mastermind situation")
                .arg(
                    Arg::new("no-info")
                        .long("no-info")
                        .short('I')
                        .help("Don't show information."),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    if !matches.is_present("no-clear") {
        clearscreen::clear().expect("failed to clear screen");
    }

    match matches.subcommand() {
        Some(("play", sub_matches)) => {
            play::run(sub_matches);
        }
        Some(("solve", sub_matches)) => {
            solve::run(sub_matches);
        }
        _ => unreachable!(),
    }
}
