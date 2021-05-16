use serde::Deserialize;
use std::error::Error;

use serenity::prelude::*;

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub enable_exec: bool,
}

impl TypeMapKey for Config {
    type Value = Self;
}

pub async fn read_config() -> Result<Config, Box<dyn Error>> {
    let config = tokio::fs::read("config.json").await?;
    match serde_json::from_slice::<Config>(&config) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

#[test]
fn config_deserialize() {
    let text = 
r#"{
    "prefix": ".",
    "enable_exec": false
}"#;

    let wanted = Config {
        prefix: String::from("."),
        enable_exec: false
    };

    match serde_json::from_str::<Config>(text) {
        Ok(c) => assert_eq!(c, wanted),
        Err(_) => panic!("assertion failed")
    }
}