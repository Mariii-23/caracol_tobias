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
#[commands(meme,number,dowload)]
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

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

// extern crate image;
// use image::io::Reader as ImageReader;

#[command]
async fn dowload(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    for attachment in &msg.attachments {
            let content = match attachment.download().await {
                Ok(content) => content,
                Err(why) => {
                    println!("Error downloading attachment: {:?}", why);
                    let _ = msg.channel_id.say(&ctx, "Error downloading attachment").await;

                    return Ok(());
                },
            };

            let mut file = match File::create(&attachment.filename).await {
                Ok(file) => file,
                Err(why) => {
                    println!("Error creating file: {:?}", why);
                    let _ = msg.channel_id.say(&ctx, "Error creating file").await;

                    return Ok(());
                },
            };

            if let Err(why) = file.write(&content).await {
                println!("Error writing to file: {:?}", why);

                    return Ok(());
            }
        // use std::io::Cursor;
        // let img2 = ImageReader::new(Cursor::new(&content)).decode()?;
        // img2.write_to(&mut content, image::ImageOutputFormat::Png)?;

            let _ = msg.channel_id.say(&ctx, &format!("Saved {:?}", attachment.filename)).await;
        }
    Ok(())
}
