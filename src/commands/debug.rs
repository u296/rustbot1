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

#[group]
#[commands(show_latest_log)]
struct Debug;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    utils::send_text_file(ctx, msg.channel_id, vec![LOG_FILE_LOCATION]).await
}
