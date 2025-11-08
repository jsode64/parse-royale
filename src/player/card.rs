use std::env::Args;

use serde_json::Value;

use crate::util::{BAD_JSON_ERR_MSG, cap_to_pascal, kebab_to_cap};

use super::Player;

/// Gathers and returns info about the player's card, or returns an error if
/// any are encountered.
pub fn get_card_info(args: &mut Args, player: &Player) -> Result<String, String> {
    let mut output = String::new();

    // Get the card's name.
    let card_name = args
        .next()
        .map(|s| kebab_to_cap(&s))
        .ok_or_else(|| "Expected card name")??;
    output.push_str(&format!(
        "-#{}/\"{}\"'s {}:",
        player.id, player.username, card_name
    ));

    // Search for the card in the player's JSON data.
    let Some(card_info) = player
        .json
        .get("cards")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Got bad JSON from the Clash Royale API")?
        .into_iter()
        .find(|v| {
            v.get("name")
                .is_some_and(|s| s.as_str().is_some_and(|s| s == card_name))
        })
    else {
        // The card is not in the array if it isn't unlocked.
        output.push_str("\n\tNot unlocked");
        return Ok(output);
    };

    push_card_level(&mut output, &card_info)?;
    push_card_star_level(&mut output, &card_info)?;
    push_card_mastery_level(&mut output, &player.json, &card_name)?;

    // Get the card's star level.

    Ok(output)
}

/// Parses and pushes the card's level information to the card info string.
///
/// The given JSON root should point to the card's data.
///
/// Returns an error if the JSON is bad.
fn push_card_level(s: &mut String, json: &Value) -> Result<(), String> {
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

    s.push_str(&format!("\n\tLevel {relative_level}"));
    Ok(())
}

/// Parses and pushes the card's star level information to the card info string.
///
/// The given JSON root should point to the card's data.
///
/// Returns a `Result` for consistency, but always returns `Ok`.
fn push_card_star_level(s: &mut String, json: &Value) -> Result<(), String> {
    let star_level = json.get("starLevel").and_then(|v| v.as_i64()).unwrap_or(0);

    s.push_str(&format!("\n\tStar Level: {star_level}"));
    Ok(())
}

/// Parses and pushes the card's mastery level information to the card info string.
///
/// The given JSON root should point to the player's data.
///
/// Returns an error if the JSON is bad.
fn push_card_mastery_level(s: &mut String, json: &Value, name: &str) -> Result<(), String> {
    let badge_name = format!("Mastery{}", cap_to_pascal(name)?);
    let Some(mastery_info) = json
        .get("badges")
        .and_then(|v| v.as_array())
        .ok_or_else(|| BAD_JSON_ERR_MSG)?
        .into_iter()
        .find(|v| {
            v.get("name")
                .is_some_and(|v| v.as_str().is_some_and(|s| s == badge_name))
        })
    else {
        // The card is not in the array if its mastery level is less than one.
        s.push_str("\n\tMastery Level 0");
        return Ok(());
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

    s.push_str(&format!("\n\tMastery Level {level}/{max_level}"));
    Ok(())
}
