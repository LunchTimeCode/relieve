use core::fmt;
use std::fs;

use anyhow::anyhow;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub files: Vec<String>,
}

impl Config {
    pub fn example() -> Self {
        Self {
            files: vec!["*.csv".to_string()],
        }
    }
    
    pub fn try_from_file() -> anyhow::Result<Self> {
        let raw = fs::read_to_string("rel.toml").map_err(|err| 
            anyhow!("Could not find config: {}", err.to_string())
         )?;
        let file: Config = toml::from_str(&raw).map_err(|err| 
            anyhow!("Could read config: {}", err.to_string())
         )?;
        Ok(file)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&toml::to_string_pretty(self).expect("not possible"), f)
    }
}

pub fn example() -> anyhow::Result<String> {
    let c = Config::example();
    let as_toml = toml::to_string_pretty(&c)?;
    fs::write("rel.toml", as_toml).expect("Unable to write file");
    Ok("config file created, please adjust to your needs".to_string())
}
