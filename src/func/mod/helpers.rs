use serenity::{framework::standard::Args, model::prelude::*};

pub fn extract_user_id(msg: &Message, args: &mut Args) -> Result<UserId, String> {
    if let Some(user) = msg.mentions.first() {
        return Ok(user.id);
    }

    args.single::<UserId>()
        .map_err(|_| "Please mention a user or provide their ID.".to_string())
}

pub fn extract_role_id(msg: &Message, args: &mut Args) -> Result<RoleId, String> {
    if let Some(role_id) = msg.mention_roles.first() {
        return Ok(*role_id);
    }

    args.single::<RoleId>()
        .map_err(|_| "Please mention a role or provide its ID.".to_string())
}
