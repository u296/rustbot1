pub mod prelude {
    pub use super::utils;
    pub use serenity::prelude::*;
    pub use std::error::Error;
    pub use tracing::*;
}
use futures::try_join;
use prelude::*;
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::standard::{macros::hook, CommandError, StandardFramework},
    model::channel::Message,
};
use songbird::SerenityInit;

mod commands;
mod config;
mod eventhandler;
mod token;
pub mod utils;
mod wolframalpha;

use tracing_subscriber::layer::SubscriberExt;

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(e) = error {
        error!("{}: {}", cmd_name, e);
    }
}

async fn async_main() -> Result<(), Box<dyn Error>> {

    let (token, config, wolframalpha_apikey) = try_join!(
        token::get_token(),
        config::get_config(),
        wolframalpha::get_wolframalpha_apikey()
    )?;

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
    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(tracing_tracy::TracyLayer::new()))?;

    let executor = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    executor.block_on(async_main())
}
