use std::env;
use std::error::Error;
use std::path::PathBuf;

use tracing::*;

use futures::try_join;

use serenity::{
    framework::standard::{macros::hook, CommandError, StandardFramework},
    model::channel::Message,
};

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::prelude::*;
use songbird::SerenityInit;

pub mod prelude {
    pub use super::utils;
    pub use serenity::prelude::*;
    pub use std::error::Error;
    pub use tracing::*;
}

mod commands;
mod config;
mod eventhandler;
mod wolframalpha;
pub mod utils;

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
            tokenpath = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let filepath = PathBuf::from(tokenpath.unwrap_or("token"));
    let token = tokio::fs::read_to_string(&filepath).await?;

    Ok(token)
}

async fn get_config() -> Result<config::Config, Box<dyn Error>> {
    let mut configpath: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--config" || arg == "-c" {
            configpath = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let path = PathBuf::from(configpath.unwrap_or(config::DEFAULT_CONFIG_PATH));
    let config = config::read_config(path).await?;
    Ok(config)
}

async fn get_wolframalpha_apikey() -> Result<Option<wolframalpha::WolframalphaApikey>, Box<dyn Error>> {
    let mut apikey_path: Option<&str> = None;
    let mut iter = env::args().peekable();

    while let Some(arg) = iter.next() {
        if arg == "--wolframalpha_apikey" || arg == "-w" {
            apikey_path = Some(iter.peek().expect("expected argument after option"));
            break;
        }
    }

    let filepath = PathBuf::from(apikey_path.unwrap_or(wolframalpha::DEFAULT_WOLFRAMALPHA_APIKEY_PATH));
    let apikey = match tokio::fs::read_to_string(&filepath).await {
        Ok(a) => Ok(Some(wolframalpha::WolframalphaApikey::from(&a))),
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e)
    }?;

    Ok(apikey)
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let (token, config, wolframalpha_apikey) = try_join!(get_token(), get_config(), get_wolframalpha_apikey())?;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefix(&config.prefix)
                .delimiters(vec![" "])
        })
        .group(&commands::GENERAL_GROUP)
        .group(&commands::VOICE_GROUP)
        .group(&commands::DEBUG_GROUP)
        .after(after_hook);

    let gateway_intents = GatewayIntents::default()
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token)
        .event_handler(eventhandler::Handler::new())
        .intents(gateway_intents)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<config::Config>(config)
        .type_map_insert::<utils::TextChannelDataMap>(utils::TextChannelDataMap::new())
        .type_map_insert::<utils::GuildDataMap>(utils::GuildDataMap::new())
        .type_map_insert::<wolframalpha::WolframalphaApikey>(wolframalpha_apikey)
        .await?;

    match client.start().await {
        Ok(()) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    executor.block_on(async_main())
}
