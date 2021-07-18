// use crate::constantes;

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
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::collections::HashSet;

#[help]
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    group: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, group, owners)
}

#[group]
#[description = "Some general commands\n"]
#[commands(ping, hi)]
struct General;

#[command]
#[description = "Says pong on \"§ping\"\n"]
#[help_available]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§")?;
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
