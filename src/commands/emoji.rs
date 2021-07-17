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
#[commands(bird)]
struct Emoji;

#[command]
fn bird(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "🐦")?;
    Ok(())
}
