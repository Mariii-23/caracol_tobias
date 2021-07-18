extern crate serenity;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[owners_only]
#[commands(say)]
struct Admins;

#[command]
#[description = "The bot will repeat what you given\n"]
#[example = ">say Hello World\nBot: Hello World\n"]
#[help_available]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    Ok(())
}
