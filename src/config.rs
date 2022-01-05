use serde::Deserialize;
use std::error::Error;
use std::path::Path;

use serenity::prelude::*;
// test comment 3
pub const DEFAULT_CONFIG_PATH: &str = "./config.json";

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub log: Option<String>,
}

impl TypeMapKey for Config {
    type Value = Self;
}

pub async fn read_config(file: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
    let config = tokio::fs::read(file).await?;
    serde_json::from_slice::<Config>(&config).map_err(|e| e.into())
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
