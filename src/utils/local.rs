use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub const CONTENT_MANIFEST_PATH: &str = "content/manifest.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentManifest {
    pub uploads: HashMap<String, String>,
}

impl ContentManifest {
    pub async fn read_from_file(
        file: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let bytes = tokio::fs::read(file).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }
}
