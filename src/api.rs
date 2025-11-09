use dotenv::var;
use reqwest::blocking::Client;
use serde_json::Value;

use crate::util::BAD_JSON_ERR_MSG;

/// The URL for getting player info from the Clash Royale API.
/// Appending the player's ID to the end will give the URL to get their data.
pub const API_PLAYER_URL: &str = "https://api.clashroyale.com/v1/players/%23";

/// Makes a JSON request to the given URL and returns the response or an error
/// if the call or deserialization fails.
pub fn api_call(url: &str) -> Result<Value, String> {
    let token = get_dev_token()?;
    let json: Value = Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .map_err(|_| "Got no API response".to_string())?
        .json()
        .map_err(|_| BAD_JSON_ERR_MSG)?;

    // Catch error responses.
    // All have the `reason` value, good responses don't.
    if let Some(err) = json.get("reason").and_then(|v| v.as_str()) {
        Err(format!("Clash Royale API returned an error \"{err}\""))
    } else {
        Ok(json)
    }
}

/// Retrieves and returns the developer token from `.env`, or an error if it
/// isn't found.
fn get_dev_token() -> Result<String, String> {
    var("TOKEN").map_err(|_| "No developer token found in `.env`".to_string())
}
