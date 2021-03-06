use super::prelude::*;

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

    Ok(())
}
