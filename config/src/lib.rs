use error::ConfigError;
use serde::Deserialize;

pub mod error;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub schema: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Auth {
    pub secret: String,
    pub allow_external_upload: bool,
    pub otp_lifespan: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct File {
    pub root: String,
    pub subdirs: Vec<String>,
    pub allow_mkdir: bool,
    pub size_limit: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub auth: Auth,
    pub file: File,
}

pub async fn load_config(path: Option<&str>) -> Result<Config, ConfigError> {
    let path = path.unwrap_or("config.toml");

    let config_file = match tokio::fs::read(path).await {
        Ok(file) => file,
        Err(_) => return Err(ConfigError::ReadFailed(path.to_string())),
    };

    let config_file = match String::from_utf8(config_file) {
        Ok(file) => file,
        Err(_) => return Err(ConfigError::ParseFailed),
    };

    let config: Config = match toml::from_str(&config_file) {
        Ok(config) => config,
        Err(_) => return Err(ConfigError::ParseFailed),
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn load_config_non_existent() {
        let config = load_config(Some("config.toml")).await;

        assert_eq!(
            ConfigError::ReadFailed("config.toml".to_string()),
            config.unwrap_err()
        );
    }

    #[tokio::test]
    async fn load_config_invalid() {
        let config = load_config(Some("tests/invalid.toml")).await;

        assert_eq!(ConfigError::ParseFailed, config.unwrap_err());
    }

    #[tokio::test]
    async fn load_config_missing() {
        let config = load_config(Some("tests/missing.toml")).await;

        assert_eq!(ConfigError::ParseFailed, config.unwrap_err());
    }

    #[tokio::test]
    async fn load_config_valid() {
        let config = load_config(Some("tests/valid.toml")).await;

        assert_eq!(config.unwrap().server.host, "localhost");
    }
}
