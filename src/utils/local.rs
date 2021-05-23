use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentManifest {
    pub uploads: HashMap<String, String>
}