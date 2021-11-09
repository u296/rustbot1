use crate::utils;
use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub trigger: String,
    pub response: String,
}

impl Response {
    pub async fn exec(
        &self,
        ctx: &Context,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if msg.content.contains(&self.trigger) {
            utils::send_buffered_text(
                ctx,
                msg.channel_id,
                futures::stream::iter(self.response.trim().lines()),
            )
            .await?;
        }
        Ok(())
    }
}
