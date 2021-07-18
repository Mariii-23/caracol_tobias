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
#[commands(bird, diamond)]
#[description = "Sends out an emoji in the chat\n"]
#[default_command(diamond)]
#[prefixes("e", "em", "emoji")]
struct Emoji;

#[command]
fn bird(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "🐦")?;
    Ok(())
}

#[command]
#[description = "Just an amazing diamond\n"]
fn diamond(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "💎")?;
    Ok(())
}
