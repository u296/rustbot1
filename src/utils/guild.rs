use std::collections::HashMap;
use std::fs::*;
use std::ops::{Deref, DerefMut};
use std::path::*;

use super::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use songbird::tracks::TrackHandle;
use songbird::tracks::TrackQueue;

use super::response::*;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersistentData {
    responses: Vec<Response>,
}

impl PersistentData {
    pub fn iter_responses(&self) -> impl std::iter::Iterator<Item = &Response> {
        self.responses.iter()
    }

    pub fn add_response(&mut self, response: Response) -> Result<(), ()> {
        match self
            .responses
            .iter()
            .find(|r| r.get_trigger() == response.get_trigger())
        {
            None => {
                self.responses.push(response);
                Ok(())
            }
            Some(_) => Err(()),
        }
    }

    pub fn remove_response(&mut self, trigger: &str) -> Result<(), ()> {
        let mut removed = false;
        self.responses.retain(|x| {
            if x.get_trigger() != trigger {
                true
            } else {
                removed = true;
                false
            }
        });

        Ok(())
    }

    pub fn flush(&self, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let path_to_dir = PathBuf::from("./guilds");
        let mut path_to_file = path_to_dir.clone();
        path_to_file.push(guild_id.0.to_string());
        path_to_file.push(".json");

        let file = match File::create(&path_to_file) {
            Ok(f) => f,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                create_dir_all(&path_to_dir)?;
                File::create(&path_to_file)?
            }
            Err(e) => return Err(Box::new(e)),
        };

        Ok(serde_json::to_writer_pretty(file, &self)?)
    }
}

#[derive(Clone, Debug)]
pub struct GuildData {
    id: GuildId,
    pub persistent: PersistentData,
}

impl GuildData {
    pub fn new(id: GuildId) -> Self {
        let mut path_to_persistent = PathBuf::from(".");
        path_to_persistent.push("guilds");
        path_to_persistent.push(format!("{}.json", id.0.to_string()));

        let persistent: PersistentData = match File::open(&path_to_persistent) {
            Ok(f) => match serde_json::from_reader(f) {
                Ok(d) => d,
                Err(e) => {
                    error!("could not parse guild persistent data: {}", e);
                    Default::default()
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                let mut dir_path = path_to_persistent.clone();
                dir_path.pop();
                std::fs::create_dir_all(dir_path).unwrap();
                let newfile = File::create(path_to_persistent).unwrap();

                let persistent = Default::default();
                serde_json::to_writer_pretty(newfile, &persistent).unwrap();
                persistent
            }
            Err(e) => {
                error!("could not read guild persistent data: {}", e);
                Default::default()
            }
        };

        GuildData {
            id: id,
            persistent: persistent,
        }
    }
}

pub struct GuildDataMap(pub HashMap<GuildId, GuildData>);

impl GuildDataMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for GuildDataMap {
    type Target = HashMap<GuildId, GuildData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildDataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeMapKey for GuildDataMap {
    type Value = Self;
}
