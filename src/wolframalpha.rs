use super::prelude::*;
use std::ops::{Deref, DerefMut};

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
