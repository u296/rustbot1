use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use serenity::prelude::*;

use crate::utils;

const LOG_FILE_LOCATION: &'static str = "/home/discord/logs/rustbot1/current";
const LOG_FILE_TMP_STORE_LOCATION: &'static str = "/tmp/rustbot/log.txt";
const LOG_FILE_TMP_STORE_DIR: &'static str = "/tmp/rustbot";

#[group]
#[commands(show_latest_log, list_server_members)]
struct Debug;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    tokio::fs::create_dir_all(LOG_FILE_TMP_STORE_DIR).await?;
    tokio::fs::copy(LOG_FILE_LOCATION, LOG_FILE_TMP_STORE_LOCATION).await?;
    utils::send_text_file(
        ctx,
        msg.channel_id,
        vec![LOG_FILE_TMP_STORE_LOCATION].into_iter(),
    )
    .await
}

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
        s.push_str(format!(
            "nickname: {:?}, name: {}, discriminator: {}",
            member.nick, member.user.name, member.user.discriminator
        ));
        s.push('\n')
    }

    utils::send_buffered_text(ctx, msg.channel_id, futures::stream::iter(s.trim().lines()));

    Ok(())
}
