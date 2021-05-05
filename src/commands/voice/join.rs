use super::prelude::*;

#[command]
#[aliases("connect")]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    info!("join");
    let guild = msg.guild(&ctx.cache).await.unwrap();

    //FIXME
    match utils::join_user(ctx, &guild, &msg.author.id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
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
