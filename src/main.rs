use std::env;
use std::error::Error;
use std::path::PathBuf;

use tracing::*;

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
            tokenpath = Some(iter.peek().expect("expected argument after -t"));
            break;
        }
    }

    let filepath = PathBuf::from(tokenpath.unwrap_or("token"));
    let token = tokio::fs::read_to_string(&filepath).await?;

    Ok(token)
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let token = get_token().await?;

    let config = config::read_config(config::CONFIG_PATH).await?;

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
