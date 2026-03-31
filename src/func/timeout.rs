use chrono::{Duration, Utc};
use serenity::{
    builder::EditMember,
    framework::standard::{Args, CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

use crate::func::helpers::extract_user_id;

#[command]
#[required_permissions("MODERATE_MEMBERS")]
pub async fn timeout(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
