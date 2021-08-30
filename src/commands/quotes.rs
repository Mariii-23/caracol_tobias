extern crate serenity;

use serde::{Deserialize, Serialize};

use serenity::{
    framework::standard::{
        Args,
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use crate::modules::quotes_struct;
use quotes_struct::*;

use crate::modules::function_aux::get_name_user_by_id;

#[group]
#[help_available]
#[commands(add,show,me)]
// #[description = "\n"]
// #[default_command(diamond)]
#[prefixes("quotes")]
struct Quotes;


extern crate serde_json;
use std::fs::File;

#[command]
//TODO alguns parametros nao estao corretos
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let phrase = args.single_quoted::<String>()?;

    // let args = args;

    // let mut all_quotes = AllQuotes::json_to_vec_movies(&msg);
    let quote = Quote::build(
        CATEGORY::MEMBERS,
        msg.id.to_string(),
        msg.author.id.to_string(),
        "ola".to_string(),
        phrase );

    let all_quotes= AllQuotes::from(quote);

    all_quotes.quotes_to_json(msg);

    msg.reply(ctx,"Quote add\n").await?;
    Ok(())
}

use rand::Rng;

#[command]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(&msg);

    let my_quotes = quotes.get_by_user_id(msg.author.id.to_string());

    match my_quotes {
        None => println!("wtff"),
        Some(vetor) => {
            for quote in vetor{
                println!("{}",&quote.quote);
            }
        }
    }

        match quotes.quotes {
            None => (),
            Some(map_category) => {
                for map_id in map_category.values() {
                    for vec_quotes in map_id.values() {
                        for quote in vec_quotes {
                            println!("{:?}",quote);
                        }
                    }
                }
            }
        }

    Ok(())
}

#[command]
async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(&msg);
    println!("user_id: {}",msg.author.id.to_string());

    let my_quotes = quotes.get_by_user_id(msg.author.id.to_string());
    let mut phrase = String::from("No quotes found");

    match my_quotes {
        None => (),
        Some(vetor) => {
            let len = vetor.len();

            let quote = vetor.get(rand::thread_rng().gen_range(0,len));
            match quote {
                None => (),
                Some(quote) => {

                    // println!("{}",&quote.quote);
                    let name = get_name_user_by_id(msg,ctx,&quote.user_id).await;
                    phrase = format!("\"{}\" - {} ({})",quote.quote,quote.nick,name);
                }
            }
        }
    }

    msg.reply(ctx, phrase).await?;

    Ok(())
}
