//! Cache and data directories, with the empty-variable rule from VIctory.
//!
//! Ported from `Marvisak/VIctory` PR #2579 ("Core: ignore empty
//! LOCALAPPDATA, XDG_CACHE_HOME, and HOME in default directory
//! resolution"). An empty environment variable is not a directory: a
//! launcher that exports `XDG_CACHE_HOME=` must not make the runtime try to
//! write to the current directory.

use std::ffi::OsString;
use std::path::PathBuf;

/// Look up a variable, dropping empty values.
pub fn nonempty(env: impl Fn(&str) -> Option<OsString>, key: &str) -> Option<OsString> {
    env(key).filter(|v| !v.is_empty())
}

pub fn nonempty_str(env: impl Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    env(key).filter(|v| !v.is_empty())
}

/// `$XDG_CACHE_HOME/bittersweet/<name>`, else `$HOME/.cache/bittersweet/<name>`.
///
/// `Bittersweet_CACHE_DIR` (checked by the caller) wins over both; this
/// function only resolves the default.
pub fn default_cache_dir_from(
    name: &str,
    env: impl Fn(&str) -> Option<OsString>,
) -> Result<PathBuf, &'static str> {
    let base = nonempty(&env, "XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| nonempty(&env, "HOME").map(|h| PathBuf::from(h).join(".cache")))
        .ok_or("cannot determine a cache directory; set BITTERSWEET_CACHE_DIR")?;
    Ok(base.join("bittersweet").join(name))
}

/// Data directory (saves, pipeline libraries), same empty-value rule.
pub fn default_data_dir_from(
    name: &str,
    env: impl Fn(&str) -> Option<OsString>,
) -> Result<PathBuf, &'static str> {
    let base = nonempty(&env, "XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| nonempty(&env, "HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .ok_or("cannot determine a data directory; set BITTERSWEET_DATA_DIR")?;
    Ok(base.join("bittersweet").join(name))
}

/// The real environment, with the documented override.
pub fn cache_dir(name: &str) -> Result<PathBuf, &'static str> {
    let env = |k: &str| std::env::var_os(k);
    if let Some(dir) = nonempty(env, "BITTERSWEET_CACHE_DIR") {
        return Ok(PathBuf::from(dir).join(name));
    }
    default_cache_dir_from(name, env)
}

/// The root of every bittersweet cache (the parent of `dxvk-state`,
/// `vkd3d`, `mesa`, ...).
pub fn cache_root() -> Result<PathBuf, &'static str> {
    let env = |k: &str| std::env::var_os(k);
    if let Some(dir) = nonempty(env, "BITTERSWEET_CACHE_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let base = nonempty(env, "XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| nonempty(env, "HOME").map(|h| PathBuf::from(h).join(".cache")))
        .ok_or("cannot determine a cache directory; set BITTERSWEET_CACHE_DIR")?;
    Ok(base.join("bittersweet"))
}

pub fn data_dir(name: &str) -> Result<PathBuf, &'static str> {
    let env = |k: &str| std::env::var_os(k);
    if let Some(dir) = nonempty(env, "BITTERSWEET_DATA_DIR") {
        return Ok(PathBuf::from(dir).join(name));
    }
    default_data_dir_from(name, env)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env_from(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<OsString> {
        move |k| {
            pairs
                .iter()
                .find(|(name, _)| *name == k)
                .map(|(_, v)| OsString::from(*v))
        }
    }

    // Ported test from VIctory PR #2579.
    #[test]
    fn empty_variables_are_not_directories() {
        let env = env_from(&[
            ("XDG_CACHE_HOME", ""),
            ("HOME", "/tmp/bs-home"),
        ]);
        let dir = default_cache_dir_from("cache", env).unwrap();
        assert_eq!(
            dir,
            PathBuf::from("/tmp/bs-home/.cache/bittersweet/cache")
        );

        let env = env_from(&[("XDG_CACHE_HOME", ""), ("HOME", "")]);
        assert!(default_cache_dir_from("cache", env).is_err());
    }

    #[test]
    fn xdg_wins_when_it_is_set() {
        let env = env_from(&[
            ("XDG_CACHE_HOME", "/tmp/xdg-cache"),
            ("HOME", "/tmp/bs-home"),
        ]);
        assert_eq!(
            default_cache_dir_from("vkd3d", env).unwrap(),
            PathBuf::from("/tmp/xdg-cache/bittersweet/vkd3d")
        );
    }

    #[test]
    fn data_dir_uses_xdg_data_home() {
        let env = env_from(&[("XDG_DATA_HOME", "/tmp/xdg-data"), ("HOME", "/tmp/h")]);
        assert_eq!(
            default_data_dir_from("saves", env).unwrap(),
            PathBuf::from("/tmp/xdg-data/bittersweet/saves")
        );
    }
}
