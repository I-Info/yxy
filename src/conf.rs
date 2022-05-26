use serde::Deserialize;
use std::error::Error;

/// Configuration file
#[derive(Debug, Deserialize)]
pub struct Config {
    pub uid: String,
    pub cookie_file: Option<String>,
    pub server_chan: Option<ServerChan>,
}

#[derive(Debug, Deserialize)]
pub struct ServerChan {
    pub key: String,
    pub title: String,
    pub warning_threshold: f32,
    pub warning_title: String,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_f = std::fs::File::open(path)?;

        let config: Config = serde_yaml::from_reader(config_f)?;

        Ok(config)
    }
}
