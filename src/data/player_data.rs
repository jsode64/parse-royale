use std::env::Args;

use crate::{
    api::{api_call, API_PLAYER_URL},
    util::write_json,
};

pub fn get_write_player_data(args: &mut Args) -> Result<String, String> {
    let id = args
        .next()
        .ok_or_else(|| "Expected player ID and output file")?;
    let path = args.next().ok_or_else(|| "Expected output file")?;
    let url = format!("{API_PLAYER_URL}{id}");
    let json = api_call(&url)?;

    write_json(&json, &path).map(|_| format!("- Wrote player #{id}'s data to `{path}`"))
}
