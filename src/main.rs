use std::{env, fs::read_to_string, fs::File, io, io::prelude::*};

extern crate serenity;

use serenity::{
    // client::bridge::gateway::ShardManager,
    // framework::standard::{
    //     help_commands,
    //     macros::{check, group, help},
    //     Args, CheckResult, CommandGroup, CommandOptions, CommandResult,
    //     DispatchError::CheckFailed,
    //     HelpOptions, StandardFramework,
    // },
    model::{gateway::Ready, id::UserId, prelude::Message},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "§ping" {
            if let Err(error) = msg.channel_id.say(&ctx.http, "Pong§§§") {
                println!("Error sending message: {:?}", error);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

const HELP_MESSAGE: &str = "
    HIII BITCHESSS!!!

    We are working!!!
    YEIIIII
";

const HELP_COMMAND: &str = "§help";

fn read_file_and_get_token() -> String {
    // let mut file = File::open(".env").expect("Error reading file");
    let mut file = File::open(".env").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token)
        .expect("Token file not found");
    token
}

fn main() {
    let TOKEN = read_file_and_get_token();
    let mut client = Client::new(&TOKEN, Handler).expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
