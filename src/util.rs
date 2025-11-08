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
}
