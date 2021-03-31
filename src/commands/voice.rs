use std::collections::HashMap;

use log::*;

use serenity::{
    framework::standard::{
        Args, CommandResult, 
        macros::{command, group, help, check, hook},
    },
    model::prelude::*,
};

use tokio::io::AsyncReadExt;

use serenity::prelude::*;

use crate::utils;

#[group]
#[commands(join, leave, play, play_local)]
struct Voice;


#[command]
#[aliases("connect")]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    info!("join");
    let guild = msg.guild(&ctx.cache).await.unwrap();

    //FIXME
    match utils::join_user(ctx, &guild, &msg.author.id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}



#[command]
#[aliases("dc", "disconnect")]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    info!("leave");
    let guild = msg.guild(&ctx.cache).await.unwrap();
    
    match utils::leave(ctx, &guild).await {
        Ok(()) => (),
        Err(e) => {
            msg.channel_id.say(ctx, format!("{}", e)).await?;
        }
    }

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
    
    match utils::play_from_input(ctx, &guild, &msg.author.id, source).await {
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
    
    match utils::play_from_input(ctx, &guild, &msg.author.id, source).await {
        Ok(()) => (),
        Err(e) => {
            msg.channel_id.say(ctx, format!("{}", e)).await?;
            return Err(e.into());
        }
    }

    Ok(())
}