mod card_data;

use std::env::Args;

use card_data::get_write_card_data;

pub fn process_data_commands(mut args: Args) -> Result<(), String> {
    while let Some(arg) = args.next() {
        let output = match arg.as_str() {
            "-c" => get_write_card_data(&mut args)?,

            _ => return Err("Expected argument (use \"--help\" for a list of commands".to_string()),
        };

        println!("{output}");
    }

    Ok(())
}
