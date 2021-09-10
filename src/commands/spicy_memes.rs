extern crate serenity;
use serenity::{
    http::AttachmentType,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

use std::fs;
use std::path::Path;
use rand::Rng;
use crate::constantes::SPICY_MEMES_PATH;

#[group]
#[prefixes("meme","memes","spicy_memes")]
#[commands(meme,number)]
#[default_command(meme)]
#[help_available]
struct spicy_memes;


#[command]
async fn meme(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let paths = match fs::read_dir(SPICY_MEMES_PATH) {
        Err(err) =>{
            msg.reply(ctx, "ERROR: Spicy memes is empty" ).await?;
            println!("Show meme error:\n{}",err);
            return Ok(());
        },
        Ok(paths) => paths,
    };
    let mut vec_path: Vec<String> = Vec::new();
    for path in paths {
        vec_path.push(path.unwrap().path().display().to_string());
    }
    let len = vec_path.len();
    if len <= 0 {
        msg.reply(ctx, "Spicy memes is empty" ).await?;
        return Ok(());
    }

    let number = rand::thread_rng().gen_range(0, len);

    let path = format!("./{}",vec_path.get(number).unwrap());

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        // m.embed(|e| {
        //     use serenity::utils::Colour;
        //     e.colour(Colour::BLITZ_BLUE);

        //     e.footer(|f|
        //              f.icon_url(msg.author.avatar_url().unwrap())
        //              .text(String::from(&msg.author.name))
        //     );
        //     e
        // });

        m.add_file(AttachmentType::Path(Path::new(&path)));
        m
    });
    msg.await.unwrap();
    Ok(())
}

#[command]
#[aliases("n")]
async fn number(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let paths = fs::read_dir(SPICY_MEMES_PATH).unwrap();
    let mut vec_path: Vec<String> = Vec::new();
    for path in paths {
        vec_path.push(path.unwrap().path().display().to_string());
    }
    let number = vec_path.len();
    let phrase = format!("We have {} memes.",number);
    msg.reply(ctx, phrase ).await?;
    Ok(())
}
