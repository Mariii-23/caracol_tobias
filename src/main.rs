mod constantes;
// use commands::send_msg::*;

use std::{env, fs::read_to_string, fs::File, io, io::prelude::*};

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
    model::{channel::Message, channel::Reaction, gateway::Ready, id::UserId},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // // send a message to channel
        // if let Err(why) = reaction.channel_id.say(
        //     &ctx.http,
        //     format!("{} left a reaction", reaction.user(&ctx.http).unwrap().name),
        // ) {
        //     println!("Error reaction to a reaction: {:?}", why);
        // }
        println!("{} left a reaction", reaction.user(&ctx.http).unwrap().name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        // let show(string) = {
        //     if let Err(error) = msg.channel_id.say(&ctx.http, string) {
        //         println!("Error sending message: {:?}", error);
        //     }
        // };

        if msg.content ==
            // "§ping" => show_msg("Pong§§§", msg, ctx),
            // "§help" => show_msg(HELP_MESSAGE, msg, ctx),
            "§ping"
        {
            // send_msg::ping_pong(&self, ctx, msg);
            if let Err(error) = msg.channel_id.say(&ctx.http, "Pong§§§") {
                println!("Error sending message: {:?}", error);
            }
        } else if msg.content == constantes::HELP_COMMAND {
            if let Err(error) = msg.channel_id.say(&ctx.http, constantes::HELP_MESSAGE) {
                println!("Error sending message: {:?}", error);
            }
        } else if msg.content == constantes::HI_COMMAND {
            if let Err(why) = msg.react(ctx, '🔥') {
                println!("Error reacting to message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

fn read_file_and_get_token() -> String {
    // let mut file = File::open(".env").expect("Error reading file");
    let mut file = File::open(".env").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token)
        .expect("Token file not found");
    token
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let TOKEN = read_file_and_get_token();
    let mut client = Client::new(&TOKEN, Handler).expect("Error creating client");

    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
