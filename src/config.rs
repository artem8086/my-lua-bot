extern crate config;

use serde::Deserialize;
use config::*;
use std::io::{Error, ErrorKind};

const CONFIG_FILE: &str = "config/config.json";
const KEYS_FILE: &str = "config/config.json";

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub port: String
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let mut settings = Config::default();
    settings
        .merge(File::with_name(CONFIG_FILE))?
        .merge(File::with_name(KEYS_FILE))?;
    Ok(settings.try_into::<AppConfig>()?)
}

pub fn load() -> Result<AppConfig, Error> {
    load_config().map_err(|error| Error::new(ErrorKind::Other, error))
}