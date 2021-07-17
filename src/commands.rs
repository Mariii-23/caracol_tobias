use crate::constantes;

extern crate serenity;

use serenity::{
    // async_trait,
    // client::bridge::gateway::ShardManager,
    // framework::standard::{
    //     help_commands,
    //     macros::{check, group, help},
    //     Args, CheckResult, CommandGroup, CommandOptions, CommandResult,
    //     DispatchError::CheckFailed,
    //     HelpOptions, StandardFramework,
    // },
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(ping, help, hi, say)]

struct General;

// says pong on "§ping"
#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§")?;
    Ok(())
}

//TODO:  change the help message
#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, constantes::HELP_MESSAGE)?;
    Ok(())
}

// Just react to you hi message
#[command]
fn hi(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "HII")?;
    msg.react(ctx, '🔥')?;
    Ok(())
}

// The bot will repeat what you given
// Ex
// > say Hello
// bot: Hello
#[command]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest())?;
    Ok(())
}

pub fn init_commands(client: &mut Client) {
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix(constantes::PREFIX)) // set the bot's prefix
            .group(&GENERAL_GROUP),
    );
}
