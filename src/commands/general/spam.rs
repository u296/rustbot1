use super::prelude::*;

#[command]
#[aliases("ping")]
#[only_in(guilds)]
async fn spam(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.message();

    let guild = msg.guild(&ctx.cache).await.unwrap();


    match guild.role_by_name(name) {
        Some(role) => {
            utils::repeat_mention(ctx, msg.channel_id, role, 10, Duration::from_secs(1)).await?;
        }
        None => {
            let members = guild.members(ctx, None, None).await?;

            match members.iter().find(|m| {
                let nickeq = if let Some(s) = &m.nick {
                    s == name
                } else {
                    false
                };
                nickeq || m.user.name == name
            }) {
                Some(member) => {
                    utils::repeat_mention(ctx, msg.channel_id, member, 10, Duration::from_secs(1))
                        .await?;
                }
                None => {
                    let mentions: Vec<_> = msg
                        .mentions
                        .iter()
                        .map(|u| u as &(dyn Mentionable + Sync))
                        .chain(
                            msg.mention_roles
                                .iter()
                                .map(|r| r as &(dyn Mentionable + Sync)),
                        )
                        .collect();

                    if !mentions.is_empty() {
                        utils::repeat_mention_multiple(
                            ctx,
                            msg.channel_id,
                            &mentions,
                            10,
                            Duration::from_secs(1),
                        )
                        .await?;
                    } else {
                        msg.channel_id
                            .say(ctx, format!("no such role or member \"{}\"", name))
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}
