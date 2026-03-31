use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

use super::helpers::{extract_role_id, extract_user_id};

#[command]
#[required_permissions("MANAGE_ROLES")]
pub async fn addrole(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
