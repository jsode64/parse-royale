mod card_data;
mod player_data;

use std::env::Args;

use card_data::get_write_card_data;
use player_data::get_write_player_data;

pub fn process_data_commands(mut args: Args) -> Result<(), String> {
    while let Some(arg) = args.next() {
        let output = match arg.as_str() {
            // Player data.
            "--player" => get_write_player_data(&mut args)?,

            // Data of all cards.
            "--cards" => get_write_card_data(&mut args)?,

            _ => return Err("Expected argument (use \"--help\" for a list of commands".to_string()),
        };

        println!("{output}");
    }

    Ok(())
}
