extern crate serenity;

use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
//TODO: help isnt working
#[help_available]
#[commands(bird, diamond)]
#[description = "Sends out an emoji in the chat\n"]
#[default_command(diamond)]
#[prefixes("em", "emoji")]
// #[required_permissions("MANAGE_EMOJIS")]
struct Emoji;

#[command]
#[description = "Sends out an bird emoji in the chat\n"]
async fn bird(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "🐦").await?;
    Ok(())
}

#[command]
#[description = "Just an amazing diamond\n"]
async fn diamond(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "💎").await?;
    Ok(())
}
