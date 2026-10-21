//! Server configuration (`config.toml`).

use std::path::Path;

use serde::Deserialize;

/// Hosting mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// Run by a user. No limits unless the admin sets them.
    #[default]
    SelfHosted,
    /// Official OutpostWave hosting. Requires the private modules.
    Official,
}

/// `[server]` section.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerSection {
    /// Hosting mode.
    #[serde(default)]
    pub mode: Mode,
    /// Display name of the server.
    pub name: String,
    /// Listen address, e.g. `0.0.0.0:8080`.
    pub bind: String,
}

/// `[limits]` section. Every limit is optional: `None` means unlimited.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    /// Maximum number of members.
    pub max_members: Option<u64>,
    /// Maximum size of a single uploaded file, in MiB.
    pub max_upload_size_mib: Option<u64>,
    /// Total storage quota, in GiB.
    pub storage_quota_gib: Option<u64>,
}

/// `[ads]` section. Ads are opt-in on self-hosted servers.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ads {
    /// Enables contextual ads. Disabled by default.
    #[serde(default)]
    pub enabled: bool,
}

/// Full configuration file.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// `[server]` section.
    pub server: ServerSection,
    /// `[limits]` section.
    #[serde(default)]
    pub limits: Limits,
    /// `[ads]` section.
    #[serde(default)]
    pub ads: Ads,
}

/// Configuration loading error.
#[derive(Debug)]
pub enum ConfigError {
    /// The file could not be read.
    Io(std::io::Error),
    /// The file is not valid TOML or does not match the schema.
    Parse(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "cannot read config: {e}"),
            Self::Parse(e) => write!(f, "invalid config: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Parses a configuration from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] if the content is invalid.
    pub fn from_toml(s: &str) -> Result<Self, ConfigError> {
        toml::from_str(s).map_err(ConfigError::Parse)
    }

    /// Loads a configuration file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let s = std::fs::read_to_string(path).map_err(ConfigError::Io)?;
        Self::from_toml(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Mode};

    #[test]
    fn shipped_config_is_valid() {
        let cfg = Config::from_toml(include_str!("../../../config/config.toml")).unwrap();
        assert_eq!(cfg.server.mode, Mode::SelfHosted);
        assert!(!cfg.ads.enabled, "ads must be disabled by default");
        assert!(
            cfg.limits.max_members.is_none(),
            "self-hosted has no limit by default"
        );
    }

    #[test]
    fn minimal_config_defaults() {
        let cfg = Config::from_toml("[server]\nname = \"x\"\nbind = \"127.0.0.1:1\"\n").unwrap();
        assert_eq!(cfg.server.mode, Mode::SelfHosted);
        assert!(!cfg.ads.enabled);
    }

    #[test]
    fn rejects_unknown_keys() {
        let s = "[server]\nname = \"x\"\nbind = \"a\"\ntypo = 1\n";
        assert!(Config::from_toml(s).is_err());
    }
}
