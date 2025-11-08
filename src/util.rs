use serde_json::Value;

/// Message for getting bad JSON from the Clash Royale API's response.
pub const BAD_JSON_ERR_MSG: &str = "Got a bad JSON response from the Clash Royale API";

/// Converts the input kebab-case string to capital case. For example:
/// `mega-minion` to `Mega Minion`.
///
/// Returns the converted string or an error if the input is the wrong format.
pub fn kebab_to_cap(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());
    let mut next_caps = true;

    for c in s.chars() {
        result.push(match c {
            'a'..='z' => {
                if next_caps {
                    next_caps = false;
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            }
            '-' | '_' | '.' => {
                next_caps = true;
                ' '
            }
            _ => return Err(format!("Invalid format for `{s}`. Should be kebab-case")),
        });
    }

    Ok(result)
}

/// Converts the input capital case string to Pascal case. For example:
/// `Mega Minion` to `MegaMinion`.
///
/// Returns the converted string or an error if the input is the wrong format.
pub fn cap_to_pascal(s: &str) -> Result<String, String> {
    let mut result = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                result.push(c);
            }
            ' ' => {}
            _ => return Err(format!("Invalid format for `{s}`. Should be kebab-case")),
        }
    }

    Ok(result)
}

/// Searches for the first item in the given JSON array path that matches the predicate.
/// Returns `None` if the array is not present or if nothing matches the predicate.
pub fn find_in_json_array<'a, F: Fn(&Value) -> bool>(
    json: &'a Value,
    name: &str,
    f: F,
) -> Option<&'a Value> {
    json.get(name)
        .and_then(|v| v.as_array())?
        .into_iter()
        .find(|&v| f(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_to_cap_test() {
        // Passing cases:
        assert_eq!(kebab_to_cap("mega-minion").unwrap(), "Mega Minion");
        assert_eq!(kebab_to_cap("royal_giant").unwrap(), "Royal Giant");
        assert_eq!(kebab_to_cap("buff.the.monk").unwrap(), "Buff The Monk");

        // Failing cases:
        assert!(kebab_to_cap("nerf ebarbs").is_err());
        assert!(kebab_to_cap("3m").is_err());
    }

    #[test]
    fn cap_to_pascal_test() {
        // Passing cases:
        assert_eq!(cap_to_pascal("Mighty Miner").unwrap(), "MightyMiner");
        assert_eq!(cap_to_pascal("Gaint Skeleton").unwrap(), "GiantSkeleton");
        assert_eq!(cap_to_pascal("Wall Breakers").unwrap(), "WallBreakers");

        // Failing cases:
        assert!(cap_to_pascal("Z4ppies").is_err());
        assert!(cap_to_pascal("Golem sucks.").is_err());
    }
}
