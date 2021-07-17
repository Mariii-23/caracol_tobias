mod commands;
mod constantes;

use std::{env, fs::read_to_string, fs::File, io, io::prelude::*};

extern crate serenity;

use serenity::{
    framework::standard::{
        macros::{command, group},
        ComandResult, StandardFramework,
    },
    model::channel::{
        Message, Reaction,
        ReactionType::{Custom, Unicode},
    },
    model::{gateway::Ready, id::UserId},
    prelude::*,
};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§")?;

    Ok(())
}
