mod commands;
mod constantes;

use std::{fs::File, io::prelude::*};

extern crate serenity;
use serenity::{
    model::channel::{
        Reaction,
        ReactionType::{Custom, Unicode},
    },
    model::gateway::Ready,
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!(
            "{} left a reaction {}",
            reaction.user(&ctx.http).unwrap().name,
            match reaction.emoji {
                Custom {
                    animated: _,
                    id: _,
                    name,
                } => name.unwrap(),
                Unicode(uni) => uni,
                _ => String::new(),
            }
        );
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
    let token = read_file_and_get_token();
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    // init commands
    commands::init_commands(&mut client);

    // start
    if let Err(msg) = client.start() {
        println!("Error: {:?}", msg);
    }
}
