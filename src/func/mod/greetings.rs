use serenity::{
    framework::standard::{CommandResult, macros::command},
    model::prelude::*,
    prelude::*,
};

#[command]
pub async fn hello(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hello there!").await?;
    Ok(())
}

#[command]
pub async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Hi! How can I help?").await?;
    Ok(())
}
