extern crate config;

use serde::Deserialize;
use config::*;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::routes::vk_bot;

const CONFIG_FILE: &str = "config/config.json";

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub port: String,
    pub vk: vk_bot::VkConfig
}

fn load_config() -> Result<AppConfig, ConfigError> {
    let mut settings = Config::default();
    settings
        .merge(File::with_name(CONFIG_FILE))?;
    Ok(settings.try_into::<AppConfig>()?)
}

pub fn load() -> Result<AppConfig, Error> {
    load_config().map_err(|error| Error::new(ErrorKind::Other, error))
}