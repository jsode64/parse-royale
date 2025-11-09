use serde_json::{to_string_pretty, Value};
use std::{fs::File, io::Write};

/// Message for getting bad JSON from the Clash Royale API's response.
pub const BAD_JSON_ERR_MSG: &str = "Got a bad JSON response from the Clash Royale API";

/// Returns `true` if the string is kebab-case, `false` if not.
pub fn is_kebab_case(s: &str) -> bool {
    // Can't start or end with a hyphen.
    if s.starts_with('-') || s.ends_with('-') {
        return false;
    }

    // Make sure all are lowercase and hyphens are separated.
    let mut last_was_hyphen = false;
    s.chars().all(|c| {
        let is_hyphen = c == '-';

        let valid = if is_hyphen {
            !last_was_hyphen
        } else {
            matches!(c, 'a'..='z' | '0'..='9')
        };

        last_was_hyphen = is_hyphen;
        valid
    })
}

/// Searches for the first item in the given JSON array path that matches the predicate.
/// Returns `None` if the array is not present or if nothing matches the predicate.
pub fn find_in_json_array<'a, P: FnMut(&Value) -> bool>(
    json: &'a Value,
    name: &str,
    mut predicate: P,
) -> Option<&'a Value> {
    json.get(name)
        .and_then(|v| v.as_array())?
        .into_iter()
        .find(|&v| predicate(v))
}

/// Write the JSON to the given file, prettily.
pub fn write_json(json: &Value, path: &str) -> Result<(), String> {
    let mut f = File::create(path)
        .map_err(|_| format!("Failed to create/open file `{path}` for writing"))?;
    let s = to_string_pretty(json).map_err(|_| BAD_JSON_ERR_MSG)?;

    f.write_all(s.as_bytes())
        .map_err(|_| format!("Failed to write to file `{path}`"))
}
