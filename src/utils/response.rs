use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::*;
use serenity::prelude::*;

use songbird::EventContext;
use songbird::Songbird;

use tracing::*;

use crate::utils;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub trigger: String,
    pub response: String,
}

struct Leaver {
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl songbird::EventHandler for Leaver {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<songbird::Event> {
        if let Some(call) = self.manager.get(self.guild_id) {
            debug!("acquired call");
            let mut c = call.lock().await;
            match c.leave().await {
                Ok(_) => debug!("left call"),
                Err(e) => error!("{}", e),
            }
        }

        None
    }
}

impl Response {
    pub async fn exec(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if msg.content.contains(&self.trigger) {
            utils::send_buffered_text(ctx, msg.channel_id, futures::stream::iter(self.response.trim().lines())).await?;
            
              
        } 
        Ok(())
    }
}
