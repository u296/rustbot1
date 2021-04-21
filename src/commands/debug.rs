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
#[commands(show_latest_log)]
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
