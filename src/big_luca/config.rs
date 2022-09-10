//! # Config
//!
//! big-luca-bot configuration

use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
/// Application config
pub struct Config {
    pub database_url: String,
    pub instagram_password: String,
    pub instagram_username: String,
    pub parameters_path: PathBuf,
    pub redis_url: String,
    pub rsshub_url: String,
    pub teloxide_token: String,
}

impl Config {
    /// Try to create config from env
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}
