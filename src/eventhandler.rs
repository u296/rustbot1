use super::prelude::*;
use crate::config;

use std::time::Duration;

use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready}
}; 

pub struct Handler;

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }
}
lazy_static! {
    static ref URL_REGEX: regex::Regex = regex::Regex::new(r#"^(?:(?:https?|ftp)://)(?:\S+(?::\S*)?@|\d{0,3}(?:\.\d{1,3}){3}|(?:(?:[a-z\d\x{00a1}-\x{ffff}]+-?)*[a-z\d\x{00a1}-\x{ffff}]+)(?:\.(?:[a-z\d\x{00a1}-\x{ffff}]+-?)*[a-z\d\x{00a1}-\x{ffff}]+)*(?:\.[a-z\x{00a1}-\x{ffff}]{2,6}))(?::\d+)?(?:[^\s]*)?$"#).unwrap();
    static ref EMBED_FAIL_REGEX: regex::Regex = regex::Regex::new(r#"^https?://([A-z]+\.)+[A-z]+(/[A-z-1-9]+)*\.(png|jpg|gif|mp4|webm|mov)$"#).unwrap();
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
        if config.reactions.embed_fail && msg.embeds.is_empty() && EMBED_FAIL_REGEX.is_match(&msg.content)
        {
            tokio::time::sleep(Duration::from_millis(500)).await;
            if msg.embeds.is_empty() {
                s.push_str("\nepic embed fail");
            }
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