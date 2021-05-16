use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::prelude::*;
use serenity::model::prelude::*;
use songbird::tracks::TrackHandle;

#[derive(Clone, Debug, Default)]
pub struct GuildData {
    // tracks that have been played during this session on this guild
    pub tracks: Vec<TrackHandle>,
    // the uuid of the last played track, used for leaving when idle
    pub last_played_track: Option<uuid::Uuid>,
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
