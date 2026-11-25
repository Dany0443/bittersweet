//! Log-filter resolution with the empty-variable rule from VIctory.
//!
//! Ported from `Marvisak/VIctory` PR #2578 ("Core: ignore empty VICTORY_LOG
//! and RUST_LOG values instead of overriding default log level"). The
//! behaviour is reimplemented here against a plain string, because the host
//! layer deliberately has no `tracing-subscriber` dependency.

/// Resolve the filter string: `BITTERSWEET_LOG`, then `RUST_LOG`, then the
/// built-in default. Empty values are skipped, not treated as a filter.
pub fn resolve_filter(default_level: &str, env: impl Fn(&str) -> Option<String>) -> String {
    let get = |k: &str| super::paths::nonempty_str(&env, k).filter(|v| !v.trim().is_empty());
    get("BITTERSWEET_LOG")
        .or_else(|| get("RUST_LOG"))
        .unwrap_or_else(|| default_level.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_from(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
        move |k| {
            pairs
                .iter()
                .find(|(name, _)| *name == k)
                .map(|(_, v)| (*v).to_owned())
        }
    }

    // Ported test from VIctory PR #2578.
    #[test]
    fn empty_values_fall_through_to_the_next_source() {
        let env = env_from(&[("BITTERSWEET_LOG", ""), ("RUST_LOG", "warn")]);
        assert_eq!(resolve_filter("info", env), "warn");

        let env = env_from(&[("BITTERSWEET_LOG", ""), ("RUST_LOG", "")]);
        assert_eq!(resolve_filter("info", env), "info");

        let env = env_from(&[("BITTERSWEET_LOG", "  "), ("RUST_LOG", "debug")]);
        assert_eq!(resolve_filter("info", env), "debug");
    }

    #[test]
    fn an_explicit_filter_wins() {
        let env = env_from(&[("BITTERSWEET_LOG", "bittersweet_host=trace")]);
        assert_eq!(resolve_filter("info", env), "bittersweet_host=trace");
    }
}
