use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::prelude::*;
use serenity::model::prelude::*;
use songbird::tracks::TrackHandle;

#[derive(Clone, Debug, Default)]
pub struct GuildData {
    // tracks that are currently playing
    pub tracks: Vec<TrackHandle>,
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
