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
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[commands(ping, help, hi)]

struct General;

#[command]
#[description = "says pong on \"§ping\"\n"]
#[help_available]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§")?;
    Ok(())
}

//TODO:  change the help message
#[command]
#[description = "Help command\n"]
#[aliases(Help)]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, constantes::HELP_MESSAGE)?;
    Ok(())
}

#[command]
#[description = "Just react to your hi\n"]
#[help_available(false)]
#[aliases(hello, Hello, Hi)]
fn hi(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "HII")?;
    msg.react(ctx, '🔥')?;
    Ok(())
}