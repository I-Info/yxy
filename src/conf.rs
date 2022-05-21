use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigInfo {
    id: String,
}

/// Configuration file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    info: ConfigInfo,
}

impl Config {
    pub fn parse(path: String) -> Result<Self, Box<dyn Error>> {
        let config_f = std::fs::File::open(path)?;

        let config: Config = serde_yaml::from_reader(config_f)?;

        Ok(config)
    }
}
