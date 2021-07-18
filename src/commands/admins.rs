extern crate serenity;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::cmd_ctx_msg_args;

#[group]
#[owners_only]
#[commands(say, say2)]
#[description = "Admin commands\n"]
struct Admins;

#[command]
#[description = "The bot will repeat what you given\n"]
#[example = ">say Hello World\nBot: Hello World\n"]
#[help_available]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    Ok(())
}

//TODO this dont work as well
// and i dont know why
cmd_ctx_msg_args! { say2,
    // msg.channel_id.say(&ctx, args.rest())?;
    println!("RIP2");
}
