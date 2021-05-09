use super::prelude::*;

use std::collections::HashMap;
use tokio::io::AsyncReadExt;

#[command]
#[aliases("p")]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    play_impl(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn play_impl(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.message();
    let guild = msg.guild(ctx).await.unwrap();

    debug!(text, "{}", &guild.name);
    
    let source = {
        if text.starts_with("http") {
            debug!("source is a link");
            songbird::ytdl(text).await
        } else {
            debug!("source is search");
            songbird::input::ytdl_search(text).await
        }
    };

    let source = match source {
        Ok(src) => src,
        Err(e) => {
            return Err(format!("{:?}", e).into());
        }
    };

    let maybe_vc = utils::get_user_voice_channel(&guild, &msg.author);

    let call = if let Some(vc) = maybe_vc {
        utils::join_voice_channel(ctx, &guild, &vc).await?
    } else {
        debug!("user is not in voice channel");
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
    play_local_impl(ctx, msg, args).await
}

#[instrument(skip(ctx))]
async fn play_local_impl(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.message();
    let guild = msg.guild(ctx).await.unwrap();

    debug!(text, "{}", &guild.name);
    

    let source = {
        let filename = {
            let manifest: HashMap<String, String> =
                match tokio::fs::File::open("content/manifest.json").await {
                    Ok(mut f) => {
                        let mut bytes = Vec::new();
                        f.read_to_end(&mut bytes).await?;

                        serde_json::from_slice(&bytes)?
                    }
                    Err(e) => return Err(e.into()),
                };

            match manifest.get(text) {
                Some(f) => f.clone(),
                None => {
                    msg.channel_id.say(ctx, "no such file").await?;
                    return Ok(());
                }
            }
        };

        let file: &str = &format!("content/{}", filename);
        debug!(file);

        songbird::ffmpeg(file).await
    };

    let source = match source {
        Ok(src) => src,
        Err(e) => {
            msg.channel_id
                .say(ctx, format!("error starting source: {:?}", e))
                .await?;
            return Err(format!("{:?}", e).into());
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
