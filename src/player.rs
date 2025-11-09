mod card;

use serde_json::Value;
use std::env::Args;

use crate::{
    api::{api_call, API_PLAYER_URL},
    util::BAD_JSON_ERR_MSG,
};

use card::get_card_info;

/// A player's basic info.
pub struct Player {
    /// Clash Royale username.
    username: String,

    /// JSON data.
    json: Value,
}

impl Player {
    /// Player data from the Clash Royale API from the given account ID.
    pub fn new(id: &str) -> Result<Self, String> {
        // Get player data.
        let url = format!("{API_PLAYER_URL}{}", id.trim_start_matches('#'));
        let json = api_call(&url)?;

        // Get the player's username since it's displayed with all data output.
        let username = json
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BAD_JSON_ERR_MSG)?
            .to_string();

        Ok(Self { username, json })
    }
}

/// Parses the input arguments after `-p` and returns the output, or an
/// error if any are encountered.
pub fn process_player_commands(mut args: Args) -> Result<(), String> {
    // Get the player's ID.
    let id = args.next().ok_or_else(|| "Expected player ID")?;
    let player = Player::new(&id)?;

    println!(
        "Got player data from ID #{}; username \"{}\"",
        id, player.username
    );

    while let Some(arg) = args.next() {
        let output = match arg.as_str() {
            // Display info about the player's card.
            "--card" => get_card_info(&mut args, &player)?,

            // Errors:
            _ => return Err(format!("Unexpected input: `{arg}`")),
        };

        println!("{output}");
    }

    Ok(())
}
