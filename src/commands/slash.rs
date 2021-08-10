use std::env;

use serenity::{
    builder::CreateMessage,
    framework::standard::{
        help_commands,
        macros::{check, command, group, help},
        Args, // CommandError,
        CommandGroup,
        CommandResult,
        HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[group]
// #[commands()]
#[description = "Slash commands\n"]
struct Slash;
