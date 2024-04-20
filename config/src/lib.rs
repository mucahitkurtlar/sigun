use clap::Parser;
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

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long, default_value = "sigun.toml")]
    pub config: String,
}

pub async fn load_config() -> Result<Config, ConfigError> {
    let args = Args::parse();
    let path = &args.config;

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
