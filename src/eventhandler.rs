use super::prelude::*;
use crate::config;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
};

pub struct Handler;

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let typemap = ctx.data.read().await;

        let config = typemap
            .get::<config::Config>()
            .expect("no config in typemap");

        let mut s = String::new();

        

        if config.reactions.nice_69 && msg.content.contains("69") {
            s.push_str("\nnice");
        }
        if config.reactions.blazeit_420 && msg.content.contains("420") {
            s.push_str("\nblaze it");
        }
        if !s.is_empty() {
            utils::send_buffered_text(
                &ctx,
                msg.channel_id,
                futures::stream::iter(s.trim().lines()),
            )
            .await
            .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} connected", ready.user.name);
    }
}
