use std::cell::*;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::sync::*;
use std::time::{Duration, Instant};

use log::*;

use futures::join;
use futures::prelude::*;

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandError, CommandGroup, CommandOptions, CommandResult, DispatchError,
        HelpOptions, Reason, StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions, MessageBuilder},
};

use songbird::SerenityInit;

use serenity::prelude::*;

mod commands;
mod config;
mod utils;

/// the md5 hash of the key must match this
const KEY_MD5_CHECKSUM_BYTES: [u8; 16] = [
    133, 23, 51, 212, 218, 233, 16, 89, 86, 135, 72, 187, 246, 150, 20, 217,
];

struct Handler;

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut s = String::new();
        if msg.content.contains("69") {
            s.push_str("\nnice");
        }
        if !msg.content.is_empty() {
            utils::send_buffered_blocking(&ctx, msg.channel_id, s.lines())
                .await
                .unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} connected", ready.user.name);
    }
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(e) = error {
        error!("{}: {}", cmd_name, e);
    }
}

async fn get_token() -> Result<String, Box<dyn Error>> {
    let mut tokenpath: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--token" || arg == "-t" {
            tokenpath = Some(iter.peek().expect("expected argument after -t"));
            break;
        }
    }

    let filepath = PathBuf::from(tokenpath.unwrap_or("token"));
    let token = tokio::fs::read_to_string(&filepath).await?;

    Ok(token)
}

fn validate_token(token: &str) -> Result<(), md5::Digest> {
    let digest = md5::compute(token);
    if md5::Digest(KEY_MD5_CHECKSUM_BYTES) == digest {
        Ok(())
    } else {
        Err(digest)
    }
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let token = get_token().await?;

    let config = config::read_config().await?;

    match validate_token(&token) {
        Ok(()) => {
            let framework = StandardFramework::new()
                .configure(|c| {
                    c.with_whitespace(true)
                        .prefix(&config.prefix)
                        .delimiters(vec![", ", ","])
                })
                .group(&commands::GENERAL_GROUP)
                .group(&commands::VOICE_GROUP)
                .group(&commands::DEBUG_GROUP)
                .after(after_hook);

            let mut client = Client::builder(&token)
                .event_handler(Handler::new())
                .framework(framework)
                .register_songbird()
                .await?;

            match client.start().await {
                Ok(()) => Ok(()),
                Err(e) => Err(e.into()),
            }
        }
        Err(checksum) => {
            println!("invalid token");
            println!("expected checksum: {:?}", KEY_MD5_CHECKSUM_BYTES);
            println!("actual checksum: {:?}", checksum);
            exit(1);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_stack_size(4 * 1024 * 1024)
        .build()?;

    executor.block_on(async_main())
}
