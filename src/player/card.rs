use serde_json::Value;
use std::env::Args;

use crate::{
    cards::Card,
    util::{find_in_json_array, BAD_JSON_ERR_MSG},
};

use super::Player;

/// Gathers and returns info about the player's card, or returns an error if
/// any are encountered.
pub fn get_card_info(args: &mut Args, player: &Player) -> Result<String, String> {
    let card = Card::from_name(args.next().ok_or_else(|| "Expected card name")?.as_str())?;
    let mut output = format!("- \"{}\" {}:", player.username, card.name);

    // Find the card's info.
    let predicate = |v: &Value| {
        v.get("id")
            .is_some_and(|v| v.as_i64().is_some_and(|id| id == card.id))
    };
    let Some(card_info) = find_in_json_array(&player.json, "cards", predicate) else {
        // The card is not in the array if it isn't unlocked.
        // Assume this is the case.
        output.push_str("\n\tNot unlocked");
        return Ok(output);
    };

    output.push_str(get_card_level(&card_info)?.as_str());
    output.push_str(get_card_evo_level(&card_info)?.as_str());
    output.push_str(get_card_star_level(&card_info)?.as_str());
    output.push_str(get_card_mastery_level(&player.json, &card)?.as_str());

    Ok(output)
}

/// Parses the card's level info and returns it in a string.
/// The string is formatted to be pushed onto `.get_card_info`'s `output`.
///
/// The given JSON root must be the card's info.
fn get_card_level(json: &Value) -> Result<String, String> {
    // We need the level and max level to get it relative to level 15.
    // Level just return the number of times the card has been leveled up (minus one).
    let level = json
        .get("level")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;
    let max_level = json
        .get("maxLevel")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;
    let relative_level = (level - max_level) + 14;

    Ok(format!("\n\tLevel: {relative_level}"))
}

/// Parses the card's evolution level and returns its info in a string.
/// If the card does not have an evolution, returns an empty string.
///
/// The given JSON root must be the card's info.
fn get_card_evo_level(json: &Value) -> Result<String, String> {
    // If this field doesn't exist the card doesn't have an evolution.
    if json.get("maxEvolutionLevel").is_none() {
        return Ok(String::new());
    }

    // If this field exists the player has the evolution; it will only ever be 1.
    let has_evolution = json.get("evolutionLevel").is_some();
    Ok(format!("\n\tEvo unlocked: {has_evolution}"))
}

/// Parses the card's star level info and returns it in a string.
/// The string is formatted to be pushed onto `.get_card_info`'s `output`.
///
/// The given JSON root must be the card's info.
fn get_card_star_level(json: &Value) -> Result<String, String> {
    let star_level = json.get("starLevel").and_then(|v| v.as_i64()).unwrap_or(0);

    Ok(format!("\n\tStar Level: {star_level}"))
}

/// Parses the card's mastery level info and returns it in a string.
/// The string is formatted to be pushed onto `.get_card_info`'s `output`.
///
/// The given JSON root must be the player's info.
fn get_card_mastery_level(json: &Value, card: &Card) -> Result<String, String> {
    let predicate = |v: &Value| {
        v.get("name")
            .is_some_and(|v| v.as_str().is_some_and(|s| s == card.badge_name))
    };
    let Some(mastery_info) = find_in_json_array(json, "badges", predicate) else {
        return Ok("\n\tMastery Level 0".to_string());
    };

    // Get level and max level.
    let level = mastery_info
        .get("level")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;
    let max_level = mastery_info
        .get("maxLevel")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?;

    Ok(format!("\n\tMastery Level: {level}/{max_level}"))
}
