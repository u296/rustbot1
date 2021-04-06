use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use log::*;

use tokio::io::{
    AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncSeek, AsyncWrite, AsyncWriteExt,
};

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    http::Http,
    model::guild::Guild,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
        prelude::*,
    },
    utils::MessageBuilder,
};

use futures::prelude::*;
use serenity::prelude::*;
use songbird::{Call, Event, EventContext, EventHandler, Songbird, TrackEvent};

use crate::utils;

#[group]
#[commands(show_latest_log)]
struct Debug;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    match tokio::fs::File::open("/home/discord/logs/rustbot1/current").await {
        Ok(f) => {
            let lines =
                tokio_stream::wrappers::LinesStream::new(tokio::io::BufReader::new(f).lines());

            utils::send_buffered(ctx, msg.channel_id, lines).await?;

            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}
