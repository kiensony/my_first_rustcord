use std::env;

use chrono::{Duration, Utc};
use serenity::{
    async_trait,
    builder::EditMember,
    framework::standard::{
        Args, CommandResult, Configuration, StandardFramework,
        macros::{command, group},
    },
    model::prelude::*,
    prelude::*,
};
use tracing_subscriber::EnvFilter;

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

fn extract_user_id(msg: &Message, args: &mut Args) -> Result<UserId, String> {
    if let Some(user) = msg.mentions.first() {
        return Ok(user.id);
    }

    args.single::<UserId>()
        .map_err(|_| "Please mention a user or provide their ID.".to_string())
}

fn extract_role_id(msg: &Message, args: &mut Args) -> Result<RoleId, String> {
    if let Some(role_id) = msg.mention_roles.first() {
        return Ok(*role_id);
    }

    args.single::<RoleId>()
        .map_err(|_| "Please mention a role or provide its ID.".to_string())
}

#[command]
async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello there!").await?;
    Ok(())
}

#[command]
async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hi! How can I help?").await?;
    Ok(())
}

#[command]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| "This command can only be used in a server.")?;
    let user_id = extract_user_id(msg, &mut args)?;
    let reason = args.rest().trim();
    let reason = if reason.is_empty() {
        "No reason provided"
    } else {
        reason
    };

    guild_id
        .ban_with_reason(ctx, user_id, 0, reason)
        .await
        .map_err(|e| format!("Failed to ban user: {e}"))?;

    msg.reply(ctx, format!("Banned <@{user_id}>: {reason}"))
        .await?;
    Ok(())
}

#[command]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| "This command can only be used in a server.")?;
    let user_id = extract_user_id(msg, &mut args)?;
    let reason = args.rest().trim();
    let reason = if reason.is_empty() {
        "No reason provided"
    } else {
        reason
    };

    guild_id
        .kick_with_reason(ctx, user_id, reason)
        .await
        .map_err(|e| format!("Failed to kick user: {e}"))?;

    msg.reply(ctx, format!("Kicked <@{user_id}>: {reason}"))
        .await?;
    Ok(())
}

#[command]
#[required_permissions("MANAGE_NICKNAMES")]
async fn nick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| "This command can only be used in a server.")?;
    let user_id = extract_user_id(msg, &mut args)?;
    let new_nick = args.rest().trim();

    if new_nick.is_empty() {
        return Err("Please provide a new nickname.".into());
    }

    let builder = EditMember::new().nickname(new_nick);

    guild_id
        .edit_member(ctx, user_id, builder)
        .await
        .map_err(|e| format!("Failed to change nickname: {e}"))?;

    msg.reply(
        ctx,
        format!("Changed nickname for <@{user_id}> to `{new_nick}`"),
    )
    .await?;
    Ok(())
}

#[command]
#[required_permissions("MANAGE_ROLES")]
async fn addrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| "This command can only be used in a server.")?;
    let user_id = extract_user_id(msg, &mut args)?;
    let role_id = extract_role_id(msg, &mut args)?;

    guild_id
        .member(ctx, user_id)
        .await
        .map_err(|e| format!("Cannot find member: {e}"))?
        .add_role(ctx, role_id)
        .await
        .map_err(|e| format!("Failed to add role: {e}"))?;

    msg.reply(ctx, format!("Added role <@&{role_id}> to <@{user_id}>."))
        .await?;
    Ok(())
}

#[command]
#[required_permissions("MODERATE_MEMBERS")]
async fn timeout(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| "This command can only be used in a server.")?;
    let user_id = extract_user_id(msg, &mut args)?;
    let minutes: i64 = args
        .single::<i64>()
        .map_err(|_| "Please provide a timeout duration in minutes.")?;

    if minutes <= 0 {
        return Err("Duration must be greater than zero minutes.".into());
    }

    let until =
        Timestamp::from_unix_timestamp((Utc::now() + Duration::minutes(minutes)).timestamp())
            .map_err(|_| "Could not build timeout timestamp.")?;
    let builder = EditMember::new().disable_communication_until_datetime(until);

    guild_id
        .edit_member(ctx, user_id, builder)
        .await
        .map_err(|e| format!("Failed to apply timeout: {e}"))?;

    msg.reply(
        ctx,
        format!("Timed out <@{user_id}> for {minutes} minute(s)."),
    )
    .await?;
    Ok(())
}
