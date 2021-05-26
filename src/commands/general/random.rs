use super::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[command]
#[aliases("pick", "random")]
async fn select_random(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let names: Vec<String> = args.iter::<String>().filter_map(|m| m.ok()).collect();

    let s: &str = match names.choose(&mut thread_rng()) {
        Some(b) => &b,
        None => "",
    };

    if !s.trim().is_empty() {
        msg.channel_id.say(ctx, s).await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn split(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild = msg.guild(ctx).await.unwrap();
    
    match msg.guild(ctx).await.unwrap().voice_states.get(&msg.author.id) {
        None => {
            msg.channel_id.say(ctx, "you are not in a voice channel").await?;
        },
        Some(c) => {
            let mut users = vec![msg.author.id];
            for (user, voicestate) in guild.voice_states.iter() {
                if voicestate.channel_id == c.channel_id {
                    users.push(*user);
                }
            }

            users.shuffle(&mut thread_rng());

            let move_users = &users[0..(users.len()/2)];



            use serenity::model::channel::ChannelType;

            let voicechannels: Vec<_> = guild.channels
                .iter()
                .filter(|(_, channel)| channel.kind == ChannelType::Voice)
                .map(|(_, channel)| channel)
                .collect();
            
            let move_channel = *match voicechannels.iter()
                .find(|c| c.name == args.message()) {
                    Some(channel) => channel,
                    None => {
                        msg.channel_id.say(ctx, "you need to specify a voice channel to split to").await?;
                        return Ok(())
                    }
                };
            

            for user in move_users.iter() {
                guild.move_member(
                    ctx,
                    user,
                    move_channel
                ).await?;
            }
        }
    }
    Ok(())
}