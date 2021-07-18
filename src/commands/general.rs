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
#[individual_command_tip = "§help [command]  Gives info about the command\n"]
// #[strikethrough_commands_tip_in_guild(None)]
// If a user lacks permissions for a command, we can hide the command
#[lacking_permissions = "Hide"]
// #[lacking_role = "Nothing"]
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
#[commands(ping, hi)]
#[description = "Some general commands\n"]
// #[individual_command_tip = String::new("{}help [command]",constantes::PREFIX)]
struct General;

#[command]
#[description = "Says pong on \"§ping\"\n"]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§")?;
    Ok(())
}

#[command]
#[description = "Just react to your hi\n"]
#[aliases(hello, Hello, Hi)]
fn hi(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "HII")?;
    msg.react(ctx, '🔥')?;
    Ok(())
}
