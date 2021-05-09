use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use tracing::*;

use serenity::{async_trait, model::prelude::*};

use serenity::prelude::*;
use songbird::{Call, Event, EventContext, EventHandler, Songbird};

#[instrument]
pub fn get_user_voice_channel(guild: &Guild, user: &User) -> Option<ChannelId> {
    match guild.voice_states.get(&user.id) {
        Some(v) => v.channel_id,
        None => None,
    }
}

#[instrument(skip(ctx))]
pub async fn join_voice_channel(
    ctx: &Context,
    guild: &Guild,
    channel: &ChannelId,
) -> Result<Arc<Mutex<Call>>, Box<dyn Error + Send + Sync>> {
    if let Some(call) = get_guild_call(ctx, guild).await {
        if let Some(current_channel) = call.clone().lock().await.current_channel() {
            if current_channel.0 == channel.0 {
                return Ok(call);
                // we are already in the right channel
            }
        }
    }

    let man = songbird::get(ctx)
        .await
        .expect("no songbird client")
        .clone();

    let gi: u64 = guild.id.into();
    let ci: u64 = channel.0;

    let (call, join_res) = man.join(gi, ci).await;

    match join_res {
        Ok(_) => Ok(call),
        Err(e) => Err(e.into()),
    }
}

#[instrument(skip(ctx))]
pub async fn get_guild_call(ctx: &Context, guild: &Guild) -> Option<Arc<Mutex<Call>>> {
    let man = songbird::get(ctx)
        .await
        .expect("no songbird client")
        .clone();

    let gi: u64 = guild.id.into();

    man.get(gi)
}

struct SongEndLeaver {
    guild_id: GuildId,
    manager: Arc<Songbird>,
}

#[async_trait]
impl EventHandler for SongEndLeaver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_tracks) = ctx {
            let call = self.manager.get(self.guild_id);

            if call.is_some() {
                self.manager.remove(self.guild_id).await.unwrap();
            }
        }

        None
    }
}

#[instrument]
pub async fn play_from_input(
    call: Arc<Mutex<Call>>,
    source: songbird::input::Input,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _song = call.lock().await.play_source(source);

    Ok(())
}
