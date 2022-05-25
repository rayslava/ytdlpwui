// configuration management
use config::{Config, ConfigError};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct YtDLP {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub ytdlp: YtDLP,
    pub log: Log,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./resources/ytdlpwui.conf";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        for (key, value) in std::env::vars() {
            println!("{key}: {value}");
        }

        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let builder = Config::builder()
            .set_default("env", env.clone())?
            .add_source(config::File::new(
                CONFIG_FILE_PATH,
                config::FileFormat::Toml,
            ))
            .add_source(config::File::with_name(&format!(
                "{}{}",
                CONFIG_FILE_PREFIX, env
            )))
            .add_source(config::Environment::with_prefix("YTDLPWUI"))
            .build()?;
        builder.try_deserialize()
    }
}
