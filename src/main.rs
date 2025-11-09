mod api;
mod cards;
mod data;
mod player;
mod util;

use std::{env::args, process::exit};

use data::process_data_commands;
use player::process_player_commands;

/// The string printed when `--help` is pased.
const HELP_STR: &str = "- List of commands:\
    \n\t`-p`: Get player info:\
        \n\t\t`parse-royale -p QQUUCL -c berserker -c miner\
    \n\t`-d`: Get raw JSON data to write to a file:\
        \n\t\t`parse-royale -d -c card_data.json`";

fn main() {
    let mut args = args();
    args.next();

    let result = match args.next().as_deref() {
        // Want help.
        Some("-h") => {
            println!("{HELP_STR}");
            Ok(())
        }

        // Want JSON data to a file.
        Some("-d") => process_data_commands(args),

        // Want player info.
        Some("-p") => process_player_commands(args),

        // Errors.
        Some(s) => Err(format!(
            "Unknown command `{s}`. Run with `-h` for a list of commands"
        )),
        _ => Err("Run with `-h` for a list of commands.".to_string()),
    };

    // Return 0 on success or 1 on error and print it.
    exit(if let Err(e) = result {
        eprintln!("{e}");
        1
    } else {
        0
    })
}
