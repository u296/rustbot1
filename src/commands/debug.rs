use std::error::Error;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::collections::HashMap;

use log::*;

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, AsyncSeek};

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        prelude::*,
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::MessageBuilder,

    model::guild::Guild,
};

use songbird::{Songbird, Call, Event, EventContext, EventHandler, TrackEvent};
use serenity::prelude::*;
use futures::prelude::*;

use crate::utils;

#[group]
#[commands(show_latest_log)]
struct Debug;

#[command]
async fn show_latest_log(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {

    match tokio::fs::File::open("/home/discord/logs/rustbot1/current").await {
        Ok(f) => {

            let lines = tokio_stream::wrappers::LinesStream::new(tokio::io::BufReader::new(f).lines());

            utils::send_buffered(ctx, msg.channel_id, lines).await?;

            Ok(())
        },
        Err(e) => {
            Err(e.into())
        }
    }
}