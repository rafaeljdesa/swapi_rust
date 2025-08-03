use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub db: DatabaseSettings
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
}