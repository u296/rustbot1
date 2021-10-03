use super::prelude::*;
use crate::utils::*;


#[command]
#[only_in(guilds)]
async fn add_reaction(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let trigger = args.single()?;
    let answer = args.single()?;

    let response = Response{
        trigger: trigger,
        response: answer,
    };
   

    let mut map = ctx.data.write().await;

    let guild_data = map.get_mut::<utils::GuildDataMap>()
        .expect("no GuildDataMap in typemap")
        .entry(guild_id)
        .or_insert(utils::GuildData::new(guild_id));
    
    match guild_data.persistent.add_response(response) {
        Ok(()) => msg.channel_id.say(ctx, "success").await?,
        Err(()) => msg.channel_id.say(ctx, "trigger already in use").await?,
    };
    guild_data.persistent.flush(guild_id)?;

    Ok(())
}

#[command]
async fn remove_reaction(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
