use serde::Deserialize;
use std::error::Error;
use std::path::Path;

use serenity::prelude::*;

pub const CONFIG_PATH: &str = "config.json";

#[derive(PartialEq, Clone, Debug, Default, Deserialize)]
pub struct ReactionOptions {
    pub nice_69: bool,
    pub blazeit_420: bool,
    pub embed_fail: bool,
}

#[derive(PartialEq, Clone, Debug, Deserialize)]
pub struct Config {
    pub prefix: String,
    pub enable_exec: bool,
    pub reactions: ReactionOptions,
}

impl TypeMapKey for Config {
    type Value = Self;
}

pub async fn read_config(file: impl AsRef<Path>) -> Result<Config, Box<dyn Error>> {
    let config = tokio::fs::read(file).await?;
    match serde_json::from_slice::<Config>(&config) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}

#[test]
fn config_deserialize() {
    let text = r#"{
    "prefix": ".",
    "enable_exec": false,
    "reactions": {
        "nice_69": true,
        "blazeit_420": true,
        "embed_fail": false
    }
}"#;

    let wanted = Config {
        prefix: String::from("."),
        enable_exec: false,
        reactions: ReactionOptions {
            nice_69: true,
            blazeit_420: true,
            embed_fail: false,
        },
    };

    match serde_json::from_str::<Config>(text) {
        Ok(c) => assert_eq!(c, wanted),
        Err(_) => panic!("assertion failed"),
    }
}
