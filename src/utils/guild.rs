use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::prelude::*;
use songbird::tracks::TrackHandle;
use serenity::model::prelude::*;


pub struct GuildData {
    track: Option<TrackHandle>
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