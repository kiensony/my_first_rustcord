use std::env;

use serenity::{
    async_trait,
    framework::standard::{Configuration, StandardFramework, macros::group},
    model::prelude::*,
    prelude::*,
};
use tracing_subscriber::EnvFilter;

mod func;
use crate::func::{
    ADDROLE_COMMAND, BAN_COMMAND, HELLO_COMMAND, HI_COMMAND, KICK_COMMAND, NICK_COMMAND,
    TIMEOUT_COMMAND, addrole, ban, hello, hi, kick, nick, timeout,
};

#[group]
#[commands(hello, hi, ban, kick, nick, addrole, timeout)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the DISCORD_TOKEN environment variable");

    let mut framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("!"));
    let framework = framework.group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await?;

    if let Err(why) = client.start().await {
        tracing::error!("Client error: {why:?}");
    }

    Ok(())
}
