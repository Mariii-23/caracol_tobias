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
#[commands(say)]
struct Owners;

#[command]
// TODO: dont work if this
// #[owners_only]
#[description = "The bot will repeat what you given\n"]
#[example = ">say Hello World\nBot: Hello World\n"]
#[help_available]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    Ok(())
}
