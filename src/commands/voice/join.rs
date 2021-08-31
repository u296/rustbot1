use super::prelude::*;
use crate::utils::GuildData;

#[command]
#[aliases("connect")]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let maybe_vc = utils::get_user_voice_channel(&guild, &msg.author);

    if let Some(vc) = maybe_vc {
        let _call = utils::join_voice_channel(ctx, &guild, &vc).await?;
    } else {
        msg.channel_id
            .say(ctx, "you are not in a voice channel")
            .await?;
    }

    Ok(())
}

#[command]
#[aliases("dc", "disconnect")]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let maybe_call = utils::get_guild_call(ctx, &guild).await;

    if let Some(call) = maybe_call {
        call.lock().await.leave().await?;
    } else {
        msg.channel_id.say(ctx, "not in a call").await?;
    }

    for track in ctx
        .data
        .write()
        .await
        .get_mut::<utils::GuildDataMap>()
        .expect("no GuildDataMap in typemap")
        .entry(msg.guild_id.unwrap())
        .or_insert(GuildData::new(msg.guild_id.unwrap()))
        .tracks
        .drain(0..)
    {
        match track.stop() {
            Ok(_) => (),
            Err(e) => {
                error!("error stopping track: {}", e);
            }
        }
    }

    Ok(())
}
