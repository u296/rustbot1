use std::error::Error;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::collections::HashMap;

use log::*;

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        prelude::*,
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::MessageBuilder,

    model::guild::Guild,
};

use tokio::io::AsyncReadExt;

use songbird::{Songbird, Call, Event, EventContext, EventHandler, TrackEvent};
use serenity::prelude::*;
use futures::prelude::*;


#[group]
#[commands(join, leave, play, play_local)]
struct Voice;

async fn get_user_voice_channel(
    guild: &Guild,
    user: &UserId
) -> Result<ChannelId, Box<dyn Error + Send + Sync>> {
    match guild.voice_states.get(user) {
        Some(v) => Ok(v.channel_id.unwrap()),
        None => {
            error!("user is not in a voice channel");
            Err("user is not in a voice channel".into())
        }
    }
}

async fn join_user(
    ctx: &Context,
    guild: &Guild,
    user: &UserId
) -> Result<Arc<Mutex<Call>>, Box<dyn Error + Send + Sync>> {
    
    let guild_id = guild.id;


    let connect_to = get_user_voice_channel(&guild, user).await?;
    debug!("acquired voice channel");



    let manager = songbird::get(ctx).await
        .expect("no songbird client")
        .clone();
    
    debug!("acquired manager");

    match {
        match tokio::time::timeout(Duration::from_secs(2), manager.join(guild_id, connect_to)).await {
            Ok(g) => g,
            Err(e) => {
                // for some reason it always times out when trying
                // to join the channel it is already in
                
                // For now we assume that when this happens we are
                // already in the correct channel
                warn!("joining channel timed out: {}", e);

                return Ok(manager.get(guild_id).unwrap());
            }
        }
    } {
        (handler, Ok(())) => {
            Ok(handler)
        },
        (_, Err(e)) => {
            error!("failed to join channel: {}", e);
            Err(e.into())
        }
    }
}


#[command]
#[aliases("connect")]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("join");
    let guild = msg.guild(&ctx.cache).await.unwrap();

    //FIXME
    match join_user(ctx, &guild, &msg.author.id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

async fn _leave(ctx: &Context, guild: &Guild) -> Result<(), Box<dyn Error + Send + Sync>> {
    debug!("leaving call in {}", guild.name);

    let manager = songbird::get(ctx).await
        .unwrap()
        .clone();

    let call = manager.get(guild.id);

    if call.is_some() {
        manager.remove(guild.id).await?;
    } else {
        return Err("not in a voice channel".into());
    }


    Ok(())
}

#[command]
#[aliases("dc", "disconnect")]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("leave");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    
    match _leave(ctx, &guild).await {
        Ok(()) => (),
        Err(e) => {
            msg.channel_id.say(ctx, format!("{}", e)).await?;
        }
    }

    Ok(())
}

struct SongEndLeaver {
    guild_id: GuildId,
    manager: Arc<Songbird>
}

#[async_trait]
impl EventHandler for SongEndLeaver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_tracks) = ctx {
            //debug!("tracks: {:#?}", tracks);

            let call = self.manager.get(self.guild_id);

            if call.is_some() {
                self.manager.remove(self.guild_id).await.unwrap();
            }
        }

        None
    }
}

async fn play_from_input(
    ctx: &Context,
    guild: &Guild,
    user: &UserId,
    source: songbird::input::Input
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    let call_lock = join_user(ctx, guild, user).await?;
    debug!("retrieved call lock");

    let mut call = call_lock.lock().await;
    debug!("locked call lock");

    let song = call.play_source(source);
    debug!("began playing song");


    song.add_event(
        Event::Track(TrackEvent::End),
        SongEndLeaver {
            guild_id: guild.into(),
            manager: songbird::get(ctx).await.unwrap()
        }
    )?;
    debug!("added song end leave event handler");
    

    Ok(())
}

#[command]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("play");
    debug!("streaming {} in {}", args.message(), msg.guild(&ctx.cache).await.unwrap().name);

    let guild = msg.guild(&ctx.cache).await.unwrap();


    let text = args.message();
    debug!("text is {}", text);

    let source = {
        if text.starts_with("http") {
            songbird::ytdl(text).await
        } else {
            songbird::input::ytdl_search(text).await
        }
    };

    

    let source = match source {
        Ok(src) => src,
        Err(e) => {
            msg.channel_id.say(ctx, format!("{:?}", e)).await?;
            return Ok(())
        }
    };

    debug!("acquired audio stream");
    
    match play_from_input(ctx, &guild, &msg.author.id, source).await {
        Ok(()) => (),
        Err(e) => {
            msg.channel_id.say(ctx, format!("{}", e)).await?;
            return Err(e.into());
        }
    }
    
    

    Ok(())
}

#[command]
#[aliases("pl", "play local", "play saved")]
#[only_in(guilds)]
async fn play_local(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("play_local");
    debug!("playing {} in {}", args.message(), msg.guild(&ctx.cache).await.unwrap().name);

    let guild = msg.guild(&ctx.cache).await.unwrap();


    let text = args.message();

    let source = {
        let filename = {
            let manifest: HashMap<String, String> = match tokio::fs::File::open("content/manifest.json").await {
                Ok(mut f) => {
                    let mut bytes = Vec::new();
                    f.read_to_end(&mut bytes).await?;

                    serde_json::from_slice(&bytes)?
                },
                _ => return Ok(())
            };

            match manifest.get(text) {
                Some(f) => f.clone(),
                None => {
                    msg.channel_id.say(ctx, "no such file").await?;
                    return Ok(());
                }
            }
        };

        songbird::ffmpeg(format!("content/{}", filename)).await
    };

    

    let source = match source {
        Ok(src) => src,
        Err(e) => {
            msg.channel_id.say(ctx, format!("error starting source: {:?}", e)).await?;
            return Ok(())
        }
    };

    debug!("acquired audio stream");
    
    match play_from_input(ctx, &guild, &msg.author.id, source).await {
        Ok(()) => (),
        Err(e) => {
            msg.channel_id.say(ctx, format!("{}", e)).await?;
            return Err(e.into());
        }
    }

    Ok(())
}