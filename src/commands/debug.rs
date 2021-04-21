use futures::prelude::*;
use tokio::io::AsyncBufReadExt;

use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};

use serenity::prelude::*;

use crate::utils;

#[group]
#[commands(show_latest_log)]
struct Debug;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    match tokio::fs::File::open("/home/discord/logs/rustbot1/current").await {
        Ok(f) => {
            let lines =
                tokio_stream::wrappers::LinesStream::new(tokio::io::BufReader::new(f).lines())
                    .filter_map(|r| future::ready(r.ok()));

            utils::send_buffered(ctx, msg.channel_id, lines).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
