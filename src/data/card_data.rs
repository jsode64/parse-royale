use std::{env::Args, fs::File, io::Write};

use crate::api::api_call;

/// The URL to get the Clash Royale API's card data.
const API_CARDS_URL: &str = "https://api.clashroyale.com/v1/cards";

/// Gets the Clash Royale API's card data and writes it to a file.
pub fn get_write_card_data(args: &mut Args) -> Result<String, String> {
    let path = args.next().ok_or_else(|| "Expected output file")?;
    let json = api_call(API_CARDS_URL)?;

    // Write the JSON data to the given file.
    let mut f = File::create(path.as_str())
        .map_err(|_| format!("Failed to open file `{path}` for writing"))?;
    f.write_all(serde_json::to_string_pretty(&json).unwrap().as_bytes())
        .map_err(|_| format!("Failed to write data to `{path}`"))?;

    Ok(format!("- Wrote card data to `{path}`"))
}
