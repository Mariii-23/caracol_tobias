extern crate serenity;

use crate::cmd_ctx_msg;

use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(bird, diamond, bird2)]
#[description = "Sends out an emoji in the chat\n"]
#[default_command(diamond)]
#[prefixes("e", "em", "emoji")]
struct Emoji;

#[command]
#[description = "Sends out an bird emoji in the chat\n"]
fn bird(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "🐦")?;
    Ok(())
}

//TODO this dont work and i dont know why
cmd_ctx_msg! { bird2,
    // msg.channel_id.say(&ctx, "🐦2")?;
    println!("RIP");
}

#[command]
#[description = "Just an amazing diamond\n"]
fn diamond(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "💎")?;
    Ok(())
}
