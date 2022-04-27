use super::prelude::*;
use serde::Deserialize;
use std::convert::TryFrom;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub const DEFAULT_CONFIG_PATH: &str = "./config.json";

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub log: Option<String>,
}

impl TypeMapKey for Config {
    type Value = Self;
}

impl FromStr for Config {
    type Err = Box<dyn Error + Send + Sync>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| e.into())
    }
}

impl TryFrom<&str> for Config {
    type Error = Box<dyn Error + Send + Sync>;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        FromStr::from_str(s)
    }
}

#[test]
fn config_deserialize() {
    let text = r#"{
    "prefix": ".",
    "log": "/home/discord/logs/current"
}"#;

    let wanted = Config {
        prefix: String::from("."),
        log: Some(String::from("/home/discord/logs/current")),
    };

    match serde_json::from_str::<Config>(text) {
        Ok(c) => assert_eq!(c, wanted),
        Err(_) => panic!("assertion failed"),
    }
}

pub async fn get_config() -> Result<Config, Box<dyn Error + Send + Sync>> {
    let mut configpath: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--config" || arg == "-c" {
            configpath = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let filepath = PathBuf::from(configpath.unwrap_or(DEFAULT_CONFIG_PATH));
    Config::from_str(&tokio::fs::read_to_string(&filepath).await?)
}
