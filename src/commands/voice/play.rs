use super::prelude::*;

use std::collections::HashMap;
use tokio::io::AsyncReadExt;

#[command]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("play");

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
            return Ok(());
        }
    };

    let maybe_vc = utils::get_user_voice_channel(&guild, &msg.author);

    let call = if let Some(vc) = maybe_vc {
        utils::join_voice_channel(ctx, &guild, &vc).await?
    } else {
        msg.channel_id
            .say(ctx, "you are not in a voice channel")
            .await?;
        return Ok(());
    };

    utils::play_from_input(call, source).await
}

#[command]
#[aliases("pl", "play local", "play saved")]
#[only_in(guilds)]
async fn play_local(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("play_local");

    let guild = msg.guild(&ctx.cache).await.unwrap();

    let text = args.message();

    let source = {
        let filename = {
            let manifest: HashMap<String, String> =
                match tokio::fs::File::open("content/manifest.json").await {
                    Ok(mut f) => {
                        let mut bytes = Vec::new();
                        f.read_to_end(&mut bytes).await?;

                        serde_json::from_slice(&bytes)?
                    }
                    _ => return Ok(()),
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
            msg.channel_id
                .say(ctx, format!("error starting source: {:?}", e))
                .await?;
            return Ok(());
        }
    };

    let maybe_vc = utils::get_user_voice_channel(&guild, &msg.author);

    let call = if let Some(vc) = maybe_vc {
        utils::join_voice_channel(ctx, &guild, &vc).await?
    } else {
        msg.channel_id
            .say(ctx, "you are not in a voice channel")
            .await?;
        return Ok(());
    };

    utils::play_from_input(call, source).await
}
