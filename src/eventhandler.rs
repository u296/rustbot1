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
        let mut typemap = ctx.data.write().await;

        let responses = typemap
            .get_mut::<utils::GuildDataMap>()
            .expect("no config in typemap")
            .entry(msg.guild_id.unwrap())
            .or_insert(utils::GuildData::new(msg.guild_id.unwrap()))
            .persistent
            .iter_responses()
            .map(|x| x.exec(&ctx, &msg));


        futures::future::join_all(responses).await;
        

    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} connected", ready.user.name);
    }
}
