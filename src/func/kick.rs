use serenity::{
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

use crate::func::helpers::extract_user_id;

#[command]
#[required_permissions("KICK_MEMBERS")]
pub async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
