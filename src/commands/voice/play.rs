use super::prelude::*;


use std::time::Instant;

use songbird::tracks::LoopState;
use songbird::input::{ytdl, ytdl_search, self};
use songbird::driver::Bitrate;

const COMPRESSED_BITRATE: Bitrate = Bitrate::BitsPerSecond(0x10000);

#[command]
#[only_in(guilds)]
async fn enqueue(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    let time1 = Instant::now();
    let source = if args.message().starts_with("https://") {
        ytdl(args.message()).await
    } else {
        ytdl_search(args.message()).await
    }.unwrap();
    let time2 = Instant::now();
    info!("acquiring source took {} ms", (time2 - time1).as_millis());
    let source = input::cached::Compressed::new(source, COMPRESSED_BITRATE).unwrap();
    let time3 = Instant::now();
    info!("compressing source took {} ms", (time3 - time2).as_millis());



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
async fn command_loop(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    let manager = songbird::get(ctx).await.unwrap();

    match manager.get(guild.id.0) {
        Some(call) => {
            let lock = call.lock().await;
            let queue = lock.queue();

            match queue.current() {
                Some(trackhandle) => {
                    let info = trackhandle.get_info().await.unwrap();

                    let (message, result) = if info.loops == LoopState::Finite(0) {
                        ("loop enabled", trackhandle.enable_loop())
                    } else {
                        ("loop disabled", trackhandle.disable_loop())
                    };

                    match result {
                        Ok(_) => {
                            msg.channel_id.say(ctx, message).await?;
                        },
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                },
                None => ()
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
