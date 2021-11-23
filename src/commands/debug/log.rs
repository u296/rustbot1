use std::path::Path;

use super::prelude::*;
use crate::config::Config;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let log_file_location = ctx
        .data
        .read()
        .await
        .get::<Config>()
        .expect("no config in typemap")
        .log
        .clone();

    match log_file_location {
        Some(file) => {
            utils::send_text_file(ctx, msg.channel_id, vec![Path::new(&file)].into_iter()).await?;
        }
        None => {
            msg.channel_id.say(ctx, "log file not configured").await?;
        }
    }

    Ok(())
}
