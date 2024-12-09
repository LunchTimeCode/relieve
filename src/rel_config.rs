use core::fmt;
use std::fs;

use anyhow::anyhow;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Files {
    files: Vec<File>,
}

impl Files {
    pub fn contains_name(&self, name: &str) -> bool {
        self.files.iter().any(|f| f.name() == name)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.files.iter().map(|f| f.name()).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct File {
    name: String,
    size: Option<u64>,
}

impl File {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn example() -> Self {
        Self {
            name: "x.csv".to_string(),
            size: Some(300),
        }
    }
}

impl FromIterator<File> for Files {
    fn from_iter<I: IntoIterator<Item = File>>(iter: I) -> Self {
        Files {
            files: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    files: Files,
}

impl Config {
    pub fn example() -> Self {
        Self {
            files: Files::from_iter(vec![File::example()]),
        }
    }

    pub fn files_names(&self) -> Vec<String> {
        self.files.get_names()
    }

    pub fn contains_file(&self, name: &str) -> bool {
        self.files.contains_name(name)
    }

    pub fn try_from_file() -> anyhow::Result<Self> {
        let raw = fs::read_to_string("rel.toml")
            .map_err(|err| anyhow!("Could not find config: {}", err.to_string()))?;
        let file: Config = toml::from_str(&raw)
            .map_err(|err| anyhow!("Could read config: {}", err.to_string()))?;
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
