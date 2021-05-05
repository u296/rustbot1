use super::prelude::*;

#[command]
async fn list_server_members(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let memberlist = msg
        .guild(ctx)
        .await
        .unwrap()
        .members(ctx, None, None)
        .await?;

    let mut s = String::new();

    for member in memberlist {
        s.push_str(&format!(
            "nickname: {:?}, name: {}, discriminator: {}",
            member.nick, member.user.name, member.user.discriminator
        ));
        s.push('\n')
    }

    utils::send_buffered_text(ctx, msg.channel_id, futures::stream::iter(s.trim().lines())).await?;

    Ok(())
}
