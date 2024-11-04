//! # Config
//!
//! big-luca-bot configuration

use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
/// Application config
pub struct Config {
    pub database_url: String,
    pub instagram_password: Option<String>,
    pub instagram_username: Option<String>,
    pub parameters_path: PathBuf,
    pub redis_url: String,
    pub teloxide_token: String,
}

impl Config {
    /// Try to create config from env
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_should_load_from_env() {
        // set env variables
        std::env::set_var(
            "DATABASE_URL",
            "postgres://bigluca:bigluca@localhost/bigluca",
        );
        std::env::set_var("INSTAGRAM_PASSWORD", "password");
        std::env::set_var("INSTAGRAM_USERNAME", "username");
        std::env::set_var("PARAMETERS_PATH", "config/parameters.json");
        std::env::set_var("REDIS_URL", "redis://localhost");
        std::env::set_var("TELOXIDE_TOKEN", "token");

        let config = Config::try_from_env().unwrap();
        assert_eq!(
            config.database_url,
            "postgres://bigluca:bigluca@localhost/bigluca"
        );
        assert_eq!(config.instagram_password, Some("password".to_string()));
        assert_eq!(config.instagram_username, Some("username".to_string()));
        assert_eq!(
            config.parameters_path,
            PathBuf::from("config/parameters.json")
        );
        assert_eq!(config.redis_url, "redis://localhost");
        assert_eq!(config.teloxide_token, "token");
    }
}
