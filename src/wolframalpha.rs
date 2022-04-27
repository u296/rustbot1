use super::prelude::*;
use std::env;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

pub const DEFAULT_WOLFRAMALPHA_APIKEY_PATH: &'static str = "./wolframalpha_apikey";
pub struct WolframalphaApikey(String);

impl WolframalphaApikey {
    pub fn new() -> Self {
        WolframalphaApikey(String::new())
    }

    pub fn from(s: &str) -> Self {
        WolframalphaApikey(String::from(s))
    }
}

impl Deref for WolframalphaApikey {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WolframalphaApikey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeMapKey for WolframalphaApikey {
    type Value = Option<Self>;
}

pub async fn get_wolframalpha_apikey() -> Result<Option<WolframalphaApikey>, Box<dyn Error>> {
    let mut apikey_path: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--wolframalpha_apikey" || arg == "-w" {
            apikey_path = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let filepath = PathBuf::from(apikey_path.unwrap_or(DEFAULT_WOLFRAMALPHA_APIKEY_PATH));
    let apikey = match tokio::fs::read_to_string(&filepath).await {
        Ok(a) => Ok(Some(WolframalphaApikey::from(&a))),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }?;

    Ok(apikey)
}
