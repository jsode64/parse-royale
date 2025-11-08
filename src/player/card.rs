use std::env::Args;

use serde_json::Value;

use crate::util::{cap_to_pascal, find_in_json_array, kebab_to_cap, BAD_JSON_ERR_MSG};

use super::Player;

/// Gathers and returns info about the player's card, or returns an error if
/// any are encountered.
pub fn get_card_info(args: &mut Args, player: &Player) -> Result<String, String> {
    // Get the card's name.
    let card_name = args
        .next()
        .map(|s| kebab_to_cap(&s))
        .ok_or_else(|| "Expected card name")??;
    let mut output = format!("- \"{}\"'s {}:", player.username, card_name);

    // Find the card's info.
    let f = |v: &Value| {
        v.get("name")
            .is_some_and(|v| v.as_str().is_some_and(|s| s == card_name))
    };
    let Some(card_info) = find_in_json_array(&player.json, "cards", f) else {
        // The card is not in the array if it isn't unlocked.
        // Assume this is the case.
        return Ok("\n\tNot unlocked".to_string());
    };

    output.push_str(get_card_level(&card_info)?.as_str());
    output.push_str(get_card_star_level(&card_info)?.as_str());
    output.push_str(get_card_mastery_level(&player.json, &card_name)?.as_str());

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
///
/// FIXME: Some cards don't have mastery badges with the same name. For example,
/// Mother With's badge name is "MasteryWitchMother", Rune Giant's is
/// "MasteryGiantBuffer", etc. which will say they have a mastery level of zero,
/// which may not be true.
fn get_card_mastery_level(json: &Value, name: &str) -> Result<String, String> {
    let badge_name = format!("Mastery{}", cap_to_pascal(name)?);
    let f = |v: &Value| {
        v.get("name")
            .is_some_and(|v| v.as_str().is_some_and(|s| s == badge_name))
    };
    let Some(mastery_info) = find_in_json_array(json, "badges", f) else {
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
