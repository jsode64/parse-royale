mod api;
mod player;
mod util;

use std::env::args;

use player::proces_player_args;

fn main() -> Result<(), String> {
    let mut args = args();
    args.next();

    match args.next().as_deref() {
        // Want player info.
        Some("-p") => proces_player_args(args),

        // Errors.
        Some(s) => return Err(format!("Invalid argument `{s}`.")),
        None => return Err("Run with `--help` for a list of commands.".to_string()),
    }
}
