use crate::config::Settings;
use config::{Config, File};
use std::env;

pub fn load_settings() -> Settings {
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());

    Config::builder()
        .add_source(File::with_name(&format!("config/{}", environment)).required(true))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
}
