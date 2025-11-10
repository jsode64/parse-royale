use std::env::Args;

use crate::{cards::NUM_CARDS, player::Player, util::BAD_JSON_ERR_MSG};

pub fn get_player_info(_: &mut Args, player: &Player) -> Result<String, String> {
    let mut output = format!("- \"{}\" Info:", player.username);
    output.push_str(get_win_loss_info(player)?.as_str());
    output.push_str(get_card_collection_info(player)?.as_str());
    output.push_str(get_clan_info(player)?.as_str());

    Ok(output)
}

/// Returns a string containing the player's win/loss numbers an ratio.
fn get_win_loss_info(player: &Player) -> Result<String, String> {
    let wins = player
        .json
        .get("wins")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;
    let losses = player
        .json
        .get("losses")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;
    let ratio = 100. * wins as f64 / (wins + losses) as f64;

    Ok(format!(
        "\n\t{wins} wins, {losses} losses ({ratio:.2}% winrate)"
    ))
}

/// Returns a string containing the player's card collection info.
fn get_card_collection_info(player: &Player) -> Result<String, String> {
    let num_unlocked = player
        .json
        .get("cards")
        .and_then(|v| v.as_array())
        .map(|arr| arr.len())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;

    Ok(format!("\n\tCards Unlocked: {num_unlocked}/{NUM_CARDS}"))
}

/// Returns a string containing the player's clan's info.
fn get_clan_info(player: &Player) -> Result<String, String> {
    let info = match player.json.get("clan") {
        Some(clan_info) => {
            let role = player
                .json
                .get("role")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BAD_JSON_ERR_MSG)?;
            let id = clan_info
                .get("tag")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BAD_JSON_ERR_MSG)?;
            let name = clan_info
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BAD_JSON_ERR_MSG)?;
            format!("{role} of {id}; \"{name}\"")
        }
        None => "None".to_string(),
    };

    Ok(format!("\n\tClan: {info}"))
}
