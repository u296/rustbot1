use super::prelude::*;
use crate::utils::GuildData;

use serenity::async_trait;
use songbird::{EventContext, EventHandler};
use std::time::Duration;

use std::sync::Arc;

use songbird::Songbird;
use uuid::Uuid;



#[command]
#[only_in(guilds)]
async fn enqueue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    let source = if args.message().starts_with("https://") {
        songbird::input::ytdl(args.message()).await
    } else {
        songbird::input::ytdl_search(args.message()).await
    }.unwrap();

    let user_voice_channel = match utils::get_user_voice_channel(&guild, &msg.author) {
        Some(c) => c,
        None => {
            return Ok(())
        }
    };

    let (call, joinresult) = manager.join(guild.id.0, user_voice_channel).await;

    match joinresult {
        Ok(_) => (),
        Err(e) => {
            error!("failed to join");
            return Ok(());
        }
    };

    call.lock().await.enqueue_source(source.into());

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap();

    match manager.get(guild.id.0) {
        Some(call) => {
            let lock = call.lock().await;
            lock.queue().skip().unwrap();

        },
        None => ()
    };

    Ok(())
}








use songbird::input::{self, Input};



#[command]
#[only_in(guilds)]
#[aliases("shutup", "stfu")]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap();

    match manager.get(guild.id.0) {
        Some(call) => {
            let mut lock = call.lock().await;
            if lock.current_channel().is_some() {
                lock.queue().stop();
                lock.leave().await.unwrap();
            }
        }, None => {
            ()
        }
    };


    Ok(())
}
