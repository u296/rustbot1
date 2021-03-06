mod prelude {
    pub use crate::utils;
    pub use serenity::{
        framework::standard::{macros::command, Args, CommandResult},
        model::channel::Message,
        model::id::{ChannelId, GuildId, UserId},
        prelude::*,
    };
    pub use tracing::*;
}

pub mod debug;
pub mod general;
pub mod voice;

pub use debug::DEBUG_GROUP;
pub use general::GENERAL_GROUP;
pub use voice::VOICE_GROUP;
