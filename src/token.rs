use super::prelude::*;
use std::env;
use std::path::PathBuf;

const DEFAULT_TOKEN_FILE_PATH: &str = "./token";

pub async fn get_token() -> Result<String, Box<dyn Error>> {
    let mut tokenpath: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--token" || arg == "-t" {
            tokenpath = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let filepath = PathBuf::from(tokenpath.unwrap_or(DEFAULT_TOKEN_FILE_PATH));
    let token = tokio::fs::read_to_string(&filepath).await?;

    Ok(token)
}
