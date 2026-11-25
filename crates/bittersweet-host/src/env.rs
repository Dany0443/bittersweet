//! Small helpers for reading the process environment without letting an
//! empty value masquerade as a setting.

use std::ffi::OsString;

/// The first non-empty value among `keys`, in order.
pub fn first_nonempty(keys: &[&str]) -> Option<(String, OsString)> {
    for key in keys {
        if let Some(value) = std::env::var_os(key).filter(|v| !v.is_empty()) {
            return Some(((*key).to_owned(), value));
        }
    }
    None
}

/// Parse a boolean switch: `1/true/yes/on` are true, `0/false/no/off` are
/// false, anything else is `None` (so a typo does not silently enable a
/// risky path).
pub fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

/// Parse a size with an optional `K`/`M`/`G` suffix into MiB.
pub fn parse_mib(value: &str) -> Option<u64> {
    let v = value.trim();
    let (number, mult) = match v.chars().last()?.to_ascii_uppercase() {
        'K' => (&v[..v.len() - 1], 1u64),
        'M' => (&v[..v.len() - 1], 1024),
        'G' => (&v[..v.len() - 1], 1024 * 1024),
        _ => (v, 1024),
    };
    let n: u64 = number.trim().parse().ok()?;
    Some(n.saturating_mul(mult) / 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booleans_are_strict() {
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("OFF"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn sizes_accept_suffixes() {
        // A bare number is MiB, the unit the runtime reports.
        assert_eq!(parse_mib("8192"), Some(8192));
        assert_eq!(parse_mib("512M"), Some(512));
        assert_eq!(parse_mib("12G"), Some(12 * 1024));
        assert_eq!(parse_mib("nope"), None);
    }
}
