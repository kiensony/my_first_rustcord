use serenity::{
    builder::EditMember,
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

use super::helpers::extract_user_id;

#[command]
#[required_permissions("MANAGE_NICKNAMES")]
pub async fn nick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
