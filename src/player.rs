mod card;

use serde_json::Value;
use std::env::Args;

use crate::api::api_call;

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
    pub fn new(id: String) -> Result<Self, String> {
        // Get player data.
        let url = format!("{API_PLAYER_URL}{}", id.trim_start_matches('#'));
        let json = api_call(&url)?;

        // Get the player's username since it's displayed with all data output.
        let username = json
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Got bad JSON from the Clash Royale API")?
            .to_string();

        Ok(Self { username, json })
    }
}

/// The URL for getting player info from the Clash Royale API.
const API_PLAYER_URL: &str = "https://api.clashroyale.com/v1/players/%23";

/// Parses the input arguments after `-p` and returns the output, or an
/// error if any are encountered.
pub fn proces_player_args(mut args: Args) -> Result<(), String> {
    // Get the player's ID.
    let id = args.next().ok_or_else(|| "Expected player ID")?;
    let player = Player::new(id)?;

    let output = match args.next().as_deref() {
        // Display info about the player's card.
        Some("card") => get_card_info(&mut args, &player)?,

        // Errors:
        Some(s) => return Err(format!("Unexpected input: `{s}`")),
        _ => return Err("Expected input".to_string()),
    };

    println!("{output}");

    Ok(())
}
