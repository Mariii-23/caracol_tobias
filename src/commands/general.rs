extern crate serenity;

use serenity::{
    framework::standard::{
        help_commands,
        macros::{ command, group, help},
        Args,
        CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

// use serenity::model::application::CurrentApplicationInfo;
use std::collections::HashSet;

#[help]
#[individual_command_tip = "§help [command]  Gives info about the command\n"]
#[command_not_found_text = "This command is not valid\n"]
// #[strikethrough_commands_tip_in_guild(None)]
// If a user lacks permissions for a command, we can hide the command
// #[lacking_permissions = "Hide"]
// #[lacking_role = "Nothing"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[commands(ping, hi, about, embed, poll,which)]
#[description = "Some general commands\n"]
struct General;

#[command]
#[description = "Says pong on \"§ping\"\n"]
 async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "Pong§§§").await?;
    Ok(())
}

#[command]
#[description = "Just react to your hi\n"]
#[aliases(hello, Hello, Hi)]
 async fn hi(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "HII").await?;
    msg.react(ctx, '🔥').await?;
    Ok(())
}

// //TODO mehhhh
// #[command]
// #[checks(Bot)]
// #[description = "Talk with your self\n"]
// #[aliases(talk)]
// async fn talk_to_self(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(&ctx, "Hello, myself!").await?;
//     Ok(())
// }

// #[check]
// #[name = "Bot"]
// async fn bot_check(ctx: &Context, msg: &Message) -> CheckResult {
//     if let Some(member) = msg.member(&ctx.cache) {
//         let user = member.user.read();
//         user.bot.into()
//     } else {
//         false.into()
//     }
// }

#[command]
#[description = "Bot will reply with pretty embed containing title and description of bot"]
async fn about(ctx: & Context, msg: &Message) -> CommandResult {
    // Obtain Bot's profile pic: cache -> current info -> bot user -> bot icon
    // let cache_http = &ctx.http;
    // let current_info = cache_http.get_current_application_info();
    // let current_info = match cache_http.get_current_application_info().await {
    //     Ok(c) => c,
    //     Err(err) => return Err(err.to_string()),
    // };

    // // let bot_user = current_info.id.to_user(cache_http);
    // let bot_user = match current_info.id.to_user(cache_http).await {
    //     Ok(u) => u,
    //     // Err(err) => return Err(CommandError(err.to_string())),
    //     Err(err) => return Err(err.to_string()),
    // };
    // let bot_icon = match bot_user.avatar_url(){
    //     Some(u) => u,
    //     None => bot_user.default_avatar_url(),
    // };

    // // let bot_icon = &ctx.http.get_current_application_info().await.id.to_user(&ctx.http).avatar_url;
    //  let bot_icon = match &ctx.http.get_current_application_info().await {
    //     Ok(u) => u.id// .to_user(&ctx.http).avatar_url
    //          ,
    //     Err(err) => return Err(err.to_string()),
    //  };

    //  let bot_icon = match &bot_icon.to_user(&ctx.http).await {
    //     Ok(u) => u// .avatar_url()
    //          ,
    //     Err(err) => return Err(err.to_string()),
    //  };

    // let bot_icon = match bot_icon.avatar_url() {
    //     Some(u) => u,
    //     None => bot_user.default_avatar_url(),
    // };

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`§23`");
            e.description("Hellooooo!!!\nMy name is Caracol Tobias, and I'm a \"carangueijo\"(crab)\n");

            //TODO: This dont work
            // e.thumbnail(bot_icon);

            // false = not inline;
            e.fields(vec![
                ("Discord", "Mariii_01🌹#2773", false),
                ("Source Code", "[Mariii-23/discord_bot_rust](https://github.com/Mariii-23/discord_bot_rust.git)", false),
            ]);
            e
        });
        m
    });
     msg.await.unwrap();
    Ok(())
}

#[command]
#[description = "Bot will generate an embed based on input."]
#[usage = "title description <image_link>"]
#[example = "rust hihih https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png"]
 async fn embed(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let title = args.single::<String>()?;
    let description = args.single::<String>()?;
    let image = args.single::<String>().unwrap_or("false".to_string());

    let link = if image == "false" {
        "https://i.imgur.com/pMBcpoq.png".to_string()
    } else {
        image.replace("<", "").replace(">", "")
    };

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(title);
            e.description(description);
            e.image(link)
        });
        m
    });

     msg.await.unwrap();
    Ok(())
 }

// fn create_poll(ctx: &Context, msg: &Message,title :&String, description:&String, counter:usize, other_options: bool) -> Message{
//     let embed = msg.channel_id.send_message(&ctx, |m| {
//         m.embed(|e| {
//             e.title(&title).description(&description).footer(|f| {
//                 f.icon_url("https://www.clipartkey.com/mpngs/m/203-2037526_diamonds-clipart-blue-diamond-logo-png-hd.png")
//                     .text("React with one emoji")
//             })
//         })
//     });

//     embed
//     // let poll = embed.await.unwrap();
//     // poll
// }

#[command]
#[description = "Create a poll,with or without options\n"]
#[usage = "title ; options"]
#[example = "Cinema tonight?"]
#[example = "Choose one options ; Funny;Great;Cool"]
#[min_args(1)]
#[max_args(27)]
 async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    let abc: Vec<char> = vec![
        '🇦', '🇧', '🇨', '🇩', '🇪', '🇫', '🇬', '🇭', '🇮', '🇯', '🇰', '🇱', '🇲', '🇳', '🇴', '🇵', '🇶', '🇷',
        '🇸', '🇹', '🇺', '🇻', '🇼', '🇽', '🇾', '🇿',
    ];
    let args = msg.content[2..].split_once(" ").unwrap();
    let mut title = String::from("Poll: ") + args.1;

    let options = args.1.split(';');
    let mut description = String::new();
    let mut count_options: usize = 0;

    for s in options {
        if count_options > 0 && count_options < 27 {
            if let Some(emote) = abc.get(count_options - 1) {
                let string = format!("{} -> {}\n", emote, s);
                description.push_str(&string);
            }
        } else {
            title = String::from("Poll: ") + s;
        }
        count_options += 1;
    }

    let embed = msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title(&title).description(&description).footer(|f| {
                f.icon_url("https://www.clipartkey.com/mpngs/m/203-2037526_diamonds-clipart-blue-diamond-logo-png-hd.png")
                    .text("React with one emoji")
            })
        })
    });

    let poll = embed.await.unwrap();

    if count_options - 1 == 0 {
        poll.react(&ctx, '✅').await?;
        poll.react(&ctx, '❌').await?;
    } else {
        let mut i: usize = 0;
        while i < count_options - 1 {
            if let Some(emote) = abc.get(i) {
                poll.react(&ctx, *emote).await?;
            }
            i += 1;
        }
    }

     // msg.reactions;

    // POLL_MESSAGE_used = Some(POLLMESSAGE::build(
    //     msg.channel_id,
    //     msg.id,
    //     msg.content.clone(),
    // ));

    Ok(())
 }


// use std::fs::File;
// use std::io::{self, prelude::*, BufReader};

#[command]
#[description("I will choose one of your given lines\nBetween the given lines it is necessary to have a enter\n")]
#[usage = "\noption 1\noption 2\n..."]
#[example = "\nFunny\nGreat\nCool"]
#[min_args(1)]
//TODO add feature to give a file and choose one random line of that file.
//TODO you can give a number and the bot will given x random lines
async fn which(ctx: &Context, msg: &Message) -> CommandResult {
    // let file_name = msg.content[2..].split_once(" ").unwrap();
    // if std::path::Path::new(&file_name.1).exists() {
    //     let file = File::open(&file_name.1)?;
    //     let reader = BufReader::new(file);

    //     for line in reader.lines() {
    //         // println!("{}", line?);
    //         msg.channel_id.say(&ctx,line?);
    //     }

    // } else {
    //     msg.reply(&ctx, "The path given dont exist.").await?;
    // }

    let args = msg.content[2..].split_once("\n").unwrap();
    let args = args.1.split("\n");

    let mut count_options: usize = 0;
    let mut v: Vec<String> = Vec::new();
    for s in args {
        count_options+=1;
        v.push(s.to_string());
    }

    extern crate rand;
    use rand::Rng;
    let random_number = rand::thread_rng().gen_range(1,&count_options);

    match v.get(random_number) {
        Some(elem) => {
            let string = format!("I choose -> {}\n", elem);
            msg.reply(&ctx, string).await?;
        },
        None => { msg.reply(&ctx, "Something happen\nError\n").await?;},
    }
    Ok(())
}
