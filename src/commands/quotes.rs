extern crate serenity;

use serde::{Deserialize, Serialize};

use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

#[group]
#[help_available]
#[commands()]
// #[description = "\n"]
// #[default_command(diamond)]
#[prefixes("quotes")]
struct Quotes;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "quotes")]
struct Quote {
    content: &str,
    id: &str,
    name: &str,
}

impl Quote {
    build(content: &str,id: &str, name: &str) -> Quote {
        Quote {
            content: content,
            id: id,
            name: name,
        }
    }
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase", tag = "quotes")]
// enum Quotes {
//     id ,
//     prof,

// }

extern crate serde_json;
use std::fs::File;


async fn add(ctx: &Context, msg: &Message) -> CommandResult {
    let new_quote = Quote.build(msg.content,msg.author.id, msg.author.name);

    msg.reply(ctx,"Quote add\n");
    Ok(())
}

async fn load(ctx: &Context, msg: &Message) -> CommandResult {

    let x = ::serde_json::from_reader(File::open("./files/quotes.json")?)?;

    Ok(())
}
