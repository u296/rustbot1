use super::prelude::*;
use crate::utils::*;


#[command]
#[only_in(guilds)]
async fn add_reaction(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let mut b = || -> Result<Response, Box<dyn std::error::Error + Send + Sync>>{
        let trigger = args.single()?;
        let reaction_type: String = args.single()?;
        let answer = args.single()?;

        match reaction_type.as_str() {
            "audio" => Ok(Response::AudioCue((trigger, answer))),
            "text" => Ok(Response::TextReply((trigger, answer))),
            _ => panic!()
        }
    };

    let response = match b() {
        Ok(r) => r,
        Err(e) => {
            return Ok(())
        }
    };

    let mut map = ctx.data.write().await;

    let mut guild_data = map.get_mut::<utils::GuildDataMap>()
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
