use std::{fs, io, path::Path};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub pulseaudio_sink: String,
    pub monitors: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    // using `IndexMap` to preserve definition order
    #[serde(rename = "profile")]
    pub profiles: indexmap::IndexMap<String, Profile>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Read error:\n  {0}")]
    Read(#[from] io::Error),
    #[error("Parse error:\n  {0}")]
    Parse(#[from] toml::de::Error),
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        fn from_file(path: &Path) -> Result<Config, ConfigError> {
            let bytes = fs::read(path)?;
            Config::from_slice(&bytes)
        }
        from_file(path.as_ref())
    }
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ConfigError> {
        Ok(toml::from_slice(bytes)?)
    }
}
