# My First Rustcord

Simple Discord moderation bot built with [`serenity`](https://github.com/serenity-rs/serenity).

## Prerequisites

- Rust toolchain
- Discord bot token with privileged intents (`MESSAGE CONTENT`, `GUILD MEMBERS`) enabled

## Running

1. Set your bot token:
   ```bash
   export DISCORD_TOKEN=your_bot_token_here
   ```
2. Start the bot:
   ```bash
   cargo run
   ```

Default prefix: `!`

## Commands

- `!hello` / `!hi` — friendly replies.
- `!ban <user> [reason]` — ban a member (requires `Ban Members`).
- `!kick <user> [reason]` — kick a member (requires `Kick Members`).
- `!nick <user> <new nickname>` — change nickname (requires `Manage Nicknames`).
- `!addrole <user> <role>` — add a role to a member (requires `Manage Roles`).
- `!timeout <user> <minutes>` — apply a communication timeout (requires `Moderate Members`).

IDs or mentions are accepted for users/roles. Use commands in guild channels where the bot has the necessary permissions.
