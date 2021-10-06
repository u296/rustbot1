use super::prelude::*;

use futures::join;

use songbird::tracks::LoopState;
use songbird::input::{Input, ytdl, ytdl_search, self};
use songbird::driver::Bitrate;

const COMPRESSED_BITRATE: Bitrate = Bitrate::BitsPerSecond(0x10000);


async fn get_local_source(name: &str) -> Result<Option<Input>, Box<dyn std::error::Error + Send + Sync>> {
    let manifest = utils::ContentManifest::read_from_file(&utils::CONTENT_MANIFEST_PATH).await?;

    let filename = match manifest.uploads.get(name) {
        Some(f) => f.clone(),
        None => return Ok(None),
    };

    let filepath: &str = format!("content/{}", filename);

    let source = songbird::ffmpeg(filepath).await?;

    Ok(Some(input::cached::Compressed::new(source, COMPRESSED_BITRATE)?.into()))
}

#[command]
#[only_in(guilds)]
#[aliases("play_local", "pl")]
async fn enqueue_local(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_fut = msg.guild(ctx);
    let manager_fut = songbird::get(ctx);
    let source_fut = get_local_source(args.message());

    let (guild, manager, source) = join!(guild_fut, manager_fut, source_fut);
    let guild = guild.unwrap();
    let manager = manager.unwrap();
    let source = match source? {
        Some(s) => s,
        None => {
            msg.channel_id.say(ctx, "no such file").await?;
            return Ok(());
        }
    };



    let user_voice_channel = match utils::get_user_voice_channel(&guild, &msg.author) {
        Some(c) => c,
        None => {
            return Ok(())
        }
    };

    let (call, joinresult) = manager.join(guild.id.0, user_voice_channel).await;

    joinresult?;

    call.lock().await.enqueue_source(source.into());

    Ok(())
}

async fn get_yt_source(text: &str) -> Result<Input, Box<dyn std::error::Error + Send + Sync>> {
    let source = if text.starts_with("https://") {
        ytdl(text).await?
    } else {
        ytdl_search(text).await?
    };

    Ok(input::cached::Compressed::new(source, COMPRESSED_BITRATE)?.into())
}

#[command]
#[only_in(guilds)]
#[aliases("play", "p")]
async fn enqueue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_fut = msg.guild(ctx);
    let manager_fut = songbird::get(ctx);
    let source_fut = get_yt_source(args.message());

    let (guild, manager, source) = join!(guild_fut, manager_fut, source_fut);
    let guild = guild.unwrap();
    let manager = manager.unwrap();
    let source = source?;



    let user_voice_channel = match utils::get_user_voice_channel(&guild, &msg.author) {
        Some(c) => c,
        None => {
            return Ok(())
        }
    };

    let (call, joinresult) = manager.join(guild.id.0, user_voice_channel).await;

    joinresult?;

    call.lock().await.enqueue_source(source.into());

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("s")]
async fn skip(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (guild, manager) = join!(msg.guild(ctx), songbird::get(ctx));
    let guild = guild.unwrap();
    let manager = manager.unwrap();

    let num_skips: usize = match args.message().parse() {
        Ok(n) => n,
        _ => 1,
    };

    match manager.get(guild.id.0) {
        Some(call) => {
            let lock = call.lock().await;
            for _ in 0..num_skips {
                lock.queue().skip().unwrap();
            }

        },
        None => ()
    };

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("loop")]
async fn command_loop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let (guild, manager) = join!(msg.guild(ctx), songbird::get(ctx));
    let guild = guild.unwrap();
    let manager = manager.unwrap();

    match manager.get(guild.id.0) {
        Some(call) => {
            let lock = call.lock().await;
            let queue = lock.queue();

            match queue.current() {
                Some(trackhandle) => {
                    let info = trackhandle.get_info().await.unwrap();

                    let message = if info.loops == LoopState::Finite(0) {
                        trackhandle.enable_loop()?;
                        "loop enabled"
                    } else {
                        trackhandle.disable_loop()?;
                        "loop disabled"
                    };
                    msg.channel_id.say(ctx, message).await?;
                        
                    
                },
                None => {
                    msg.channel_id.say(ctx, "nothing playing").await?;
                }
            };

        },
        None => ()
    };

    Ok(())
}


#[command]
#[only_in(guilds)]
#[aliases("shutup", "stfu")]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let (guild, manager) = join!(msg.guild(ctx), songbird::get(ctx));
    let guild = guild.unwrap();
    let manager = manager.unwrap();

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
