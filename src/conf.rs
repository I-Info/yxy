use serde::{Deserialize, Serialize};
use std::error::Error;

/// Configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub uid: String,
    pub cookie_file: Option<String>,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        let config_f = std::fs::File::open(path)?;

        let config: Config = serde_yaml::from_reader(config_f)?;

        Ok(config)
    }
}
