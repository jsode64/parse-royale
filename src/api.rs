use dotenv::var;
use reqwest::Client;
use serde_json::Value;

/// Makes a JSON request to the given URL and returns the response or an error
/// if the call or deserialization fails.
pub async fn api_call(url: &str) -> Result<Value, String> {
    let token = get_dev_token()?;
    let json: Value = Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| "Got no API response".to_string())?
        .json()
        .await
        .map_err(|_| "Got a bad API response".to_string())?;

    Ok(json)
}

/// Retrieves and returns the developer token from `.env`, or an error if it
/// isn't found.
fn get_dev_token() -> Result<String, String> {
    var("TOKEN").map_err(|_| "No developer token found in `.env`".to_string())
}
