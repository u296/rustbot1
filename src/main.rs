use std::env;
use std::fs;
use std::error::Error;
use std::sync::*;
use std::cell::*;
use std::process::exit;
use std::time::{Instant, Duration};

use log::*;

use futures::prelude::*;
use futures::join;

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup, CommandError,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
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

/// the md5 hash of the key must match this
const KEY_MD5_CHECKSUM: [u8; 16] = [
    133,
    23,
    51,
    212,
    218,
    233,
    16,
    89,
    86,
    135,
    72,
    187,
    246,
    150,
    20,
    217
];



struct Handler;

impl Handler {
    pub fn new() -> Handler {
        Handler {
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("69") {
            msg.channel_id.say(&ctx, "nice").await.unwrap();
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


async fn async_main() {
    env_logger::init();
    let token = {
         let mut tokenpath: Option<&str> = None;
         let mut iter = env::args().peekable();
         while let Some(arg) = iter.next() {
            if arg == "-token" || arg == "-t" {
                tokenpath = Some(iter.peek().expect("expected token after -t argument"));
                 break;
             }
        }
        use std::path::PathBuf;
                
        let filepath: PathBuf = PathBuf::from(tokenpath.unwrap_or("token"));
        let token = fs::read_to_string(&filepath).unwrap();



        token
    };

    let checksum = md5::compute(&token);

    let checksum: [u8; 16] = checksum.into();

    if checksum == KEY_MD5_CHECKSUM {

        let framework = StandardFramework::new()
            .configure(|c| c
                .with_whitespace(true)
                .prefix("%")
                .delimiters(vec![", ", ","])
            )
            .group(&commands::GENERAL_GROUP)
            .group(&commands::VOICE_GROUP)
            .after(after_hook);

        let mut client = Client::builder(&token)
        .event_handler(Handler::new())
        .framework(framework)
        .register_songbird()
        .await
        .expect("error creating client");

        if let Err(why) = client.start().await {
            println!("client error: {:?}", why);
        }
    } else {
        println!("invalid token");
        println!("expected checksum: {:?}", KEY_MD5_CHECKSUM);
        println!("actual checksum: {:?}", checksum);
        exit(1);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .thread_stack_size(1024*1024)
        .build()?;

    executor.block_on(async_main());
    
    Ok(())
}