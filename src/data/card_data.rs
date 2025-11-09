use std::env::Args;

use crate::{api::api_call, util::write_json};

/// The URL to get the Clash Royale API's card data.
const API_CARDS_URL: &str = "https://api.clashroyale.com/v1/cards";

/// Gets the Clash Royale API's card data and writes it to a file.
pub fn get_write_card_data(args: &mut Args) -> Result<String, String> {
    let path = args.next().ok_or_else(|| "Expected output file")?;
    let json = api_call(API_CARDS_URL)?;

    write_json(&json, &path).map(|_| format!("- Wrote card data to `{path}`"))
}
