extern crate serenity;

use serenity::{
 builder::CreateMessage,
    framework::standard::{
        help_commands,
        macros::{check, command, group, help},
        Args,  // CommandError,
        CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

use serenity_utils::menu::Menu;
use crate::modules::pagination;
use std::collections::HashSet;

#[help]
#[individual_command_tip = "§help [command]  Gives info about the command\n"]
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
// #[commands(ping, hi, talk_to_self, about, embed, poll)]
#[commands(ping, hi, about, embed, poll,menu)]
#[description = "Some general commands\n"]
// #[individual_command_tip = String::new("{}help [command]",constantes::PREFIX)]
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
    // let current_info = match cache_http.get_current_application_info() {
    //     Ok(c) => c,
    //     Err(err) => return Err(CommandError(err.to_string())),
    // };
    // let bot_user = match current_info.id.to_user(cache_http) {
    //     Ok(u) => u,
    //     Err(err) => return Err(CommandError(err.to_string())),
    // };
    // let bot_icon = match bot_user.avatar_url() {
    //     Some(u) => u,
    //     None => bot_user.default_avatar_url(),
    // };

    // let bot_icon = &ctx.http.get_current_application_info().id.to_user(&ctx.http).avatar_url;

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("`§23`");
            e.description("A bot with random and probably nonsensical features.\n");

            // e.thumbnail(bot_icon);

            // false = not inline
            e.fields(vec![
                ("Discord", "Mariii_01🌹#2773", false),
                ("Source Code", "[Mariii-23/discord_bot_rust](https://github.com/Mariii-23/discord_bot_rust.git)", false),
            ]);
            e
        });
        m
    });
    Ok(())
}
#[command]
#[description = "Bot will generate an embed based on input."]
#[usage = "title description <image_link>"]
#[example = "§23 can generate embeds!,https://docs.rs/rust-logo-20210302-1.52.0-nightly-35dbef235.png"]
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
    Ok(())
}

#[command]
#[description = "Create a poll,with or without options\n"]
#[usage = "title ; options"]
#[example = "Cinema tonight?"]
#[example = "Choose one options ; Funny;Great;Cool"]
#[min_args(1)]
#[max_args(27)]
 async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    let ABC: Vec<char> = vec![
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
            if let Some(emote) = ABC.get(count_options - 1) {
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
            if let Some(emote) = ABC.get(i) {
                poll.react(&ctx, *emote).await?;
            }
            i += 1;
        }
    }

    // POLL_MESSAGE_used = Some(POLLMESSAGE::build(
    //     msg.channel_id,
    //     msg.id,
    //     msg.content.clone(),
    // ));

    Ok(())
 }


#[command]
async fn menu(ctx: &Context, msg: &Message) -> CommandResult {
    // let args = msg.content[2..].split_once(" ").unwrap();
    // let mut title = String::from("Menu: ") + args.1;

    // let options = args.1.split(';');

    let mut page_one = CreateMessage::default();
    page_one.content("Page number one!").embed(|e| {
        e.description("The first page!");

        e
    });

    let mut page_two = CreateMessage::default();
    page_two.content("Page number two!").embed(|e| {
        e.description("The second page!");

        e
    });

    let pages = [page_one, page_two];

    // Creates a new menu.
    let menu = Menu::new(ctx, msg, &pages, pagination::simple_options());

    // Runs the menu and returns optional `Message` used to display the menu.
    let _ = menu.run().await?;

    Ok(())
}
