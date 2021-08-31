extern crate serenity;
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

#[group]
#[help_available(false)]
// #[help_available]
#[commands(add,show,me,build)]
#[description = "**Quotes are fun**\n\nWe have 3 category:\n**\"MEMBERS\"** -> quotes from people in the server\n**\"PROFS\"** -> quotes from profs\n **\"GENERAL\"** -> random phrases "]
#[default_command(show)]
#[prefixes("quotes","quote")]
struct Quotes;

#[command]
#[sub_commands(add_members,add_profs,add_general)]
#[description = "Add one quote to the server in category \"MEMBERS\"\nYou can reply one msg with §quotes add"]
#[example="\"Quote\""]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match &msg.referenced_message {
        None => add_members(ctx, msg, args).await,
        Some(referenced_message) => {
            let mut all_quotes = AllQuotes::json_to_vec_movies(msg);
            let phrase = String::from(&referenced_message.content);
            let nick = match &referenced_message.author.nick_in(&ctx, msg.guild_id.unwrap()).await {
                None => String::from(&referenced_message.author.name),
                Some(name) => name.to_string(),
            };

            let quote =Quote::build(
                CATEGORY::MEMBERS,
                referenced_message.id.to_string(),
                referenced_message.author.id.to_string(),
                nick,
                phrase
            );

            match all_quotes.add(quote) {
                true => {
                    all_quotes.quotes_to_json(msg);
                    msg.reply(ctx,"Quote added\n").await?;
                },
                false => {
                    msg.reply(ctx,"Quote not added\nAlready exists\n").await?;
                }
            }
            Ok(())
        },
    }
}

#[command]
#[max_args(2)]
#[min_args(1)]
#[aliases(members)]
#[description = "Add one quote to the server in category \"MEMBERS\"\n"]
#[example="\"Quote\""]
#[example="\"Quote\" @person"]
async fn add_members(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;
    let mut all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quote = if msg.mentions.is_empty() {
        let nick = match msg.author.nick_in(&ctx, msg.guild_id.unwrap()).await {
                None => String::from(&msg.author.name),
                Some(name) => name,
        };

        Quote::build(
            CATEGORY::MEMBERS,
            msg.id.to_string(),
            msg.author.id.to_string(),
            nick,
            phrase
        )
    } else {
        let person = &msg.mentions[0];
        let nick = match &person.nick_in(&ctx, msg.guild_id.unwrap()).await {
            None => String::from(&person.name),
            Some(name) => String::from(name),
        };

        Quote::build(
            CATEGORY::MEMBERS,
            msg.id.to_string(),
            person.id.to_string(),
            nick,
            phrase
        )
    };

    match all_quotes.add(quote) {
        true => {
            all_quotes.quotes_to_json(msg);
            msg.reply(ctx,"Quote added\n").await?;
        },
        false => {
            msg.reply(ctx,"Quote not added\nAlready exists\n").await?;
        }
    }
    Ok(())
}


#[command]
#[max_args(2)]
#[min_args(2)]
#[aliases(profs)]
#[description = "Add one quote to the server in category \"PROFS\"\n"]
#[example="\"Quote\" \"profs\""]
async fn add_profs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;
    let nick = args.single_quoted::<String>()?;

    let mut all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quote = Quote::build(
        CATEGORY::PROFS,
        msg.id.to_string(),
        nick.to_lowercase(),
        nick,
        phrase );

    match all_quotes.add(quote) {
        true => {
            all_quotes.quotes_to_json(msg);
            msg.reply(ctx,"Quote added\n").await?;
        },
        false => {
            msg.reply(ctx,"Quote not added\nAlready exists\n").await?;
        }
    }
    Ok(())
}

#[command]
#[max_args(2)]
#[min_args(2)]
#[aliases(general)]
#[description = "Add one quote to the server in category \"GENERAL\"\n"]
#[example="\"Quote\" \"category\""]
async fn add_general(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;
    let user_id = args.single_quoted::<String>()?;

    let mut all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quote = Quote::build(
        CATEGORY::GENERAL,
        msg.id.to_string(),
        String::from(&user_id),
        user_id,
        phrase );

    match all_quotes.add(quote) {
        true => {
            all_quotes.quotes_to_json(msg);
            msg.reply(ctx,"Quote added\n").await?;
        },
        false => {
            msg.reply(ctx,"Quote not added\nAlready exists\n").await?;
        }
    }
    Ok(())
}


/* show quotes */

#[command]
#[sub_commands(show_general,show_profs,show_members)]
#[description = "Show one quote\n"]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let phrase = quotes.get_one_quote_to_string(ctx, msg).await;
    msg.reply(ctx, phrase).await?;
    Ok(())
}


#[command]
#[description = "Show one quote that is yours\n"]
async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let phrase = quotes.get_one_quote_by_user_id_to_string(ctx, msg,msg.author.id.to_string(), CATEGORY::MEMBERS).await;
    msg.reply(ctx, phrase).await?;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(general)]
#[description = "Show one quote in the category \"GENERAL\"\n"]
#[usage = ""]
#[usage = "\"category\""]
async fn show_general(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let mut phrase = String::from("No quotes found");
    if args.is_empty() {
        phrase = quotes.get_one_quote_by_category_to_string(ctx, msg,CATEGORY::GENERAL).await;
    } else {
        //TODO pode se mudar para ser sem ""
        let id = args.single_quoted::<String>()?;
        phrase = quotes.get_one_quote_by_user_id_to_string(ctx, msg,id, CATEGORY::GENERAL).await;
    }
    msg.reply(ctx, phrase).await?;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(profs)]
#[description = "Show one quote in the category \"PROFS\"\n"]
#[usage = ""]
#[usage = "\"profs's name\""]
async fn show_profs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let mut phrase = String::from("No quotes found");
    if args.is_empty() {
        phrase = quotes.get_one_quote_by_category_to_string(ctx, msg,CATEGORY::PROFS).await;
    } else {
        //TODO pode se mudar para ser sem ""
        let id = args.single_quoted::<String>()?;
        phrase = quotes.get_one_quote_by_user_id_to_string(ctx, msg,id, CATEGORY::PROFS).await;
    }
    msg.reply(ctx, phrase).await?;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(members)]
#[description = "Show one quote in the category \"MEMBERS\"\n"]
#[usage = ""]
#[usage = "@person"]
async fn show_members(ctx: &Context, msg: &Message) -> CommandResult {
    let person = &msg.mentions;
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let mut phrase = String::from("No quotes found");
    if person.is_empty() {
        phrase = quotes.get_one_quote_by_category_to_string(ctx, msg,CATEGORY::MEMBERS).await;
    } else {
        let id = person[0].id.to_string();
        phrase = quotes.get_one_quote_by_user_id_to_string(ctx, msg,id, CATEGORY::MEMBERS).await;
    }
    msg.reply(ctx, phrase).await?;
   Ok(())
}

#[command]
#[max_args(5)]
#[min_args(4)]
#[help_available(false)]
#[description = "Add one quote with all information\n"]
#[usage = "\"server_id\" \"CATEGORY\" \"user_id\" \"name\" \"quote\""]
// serverid (op) - category - user_id - name -  phrase
async fn build(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let server_id = if args.len() >= 5 {
        args.single_quoted::<String>()?
    } else {
        String::from(msg.guild_id.unwrap().to_string().as_str())
    };

    let category = match args.single_quoted::<CATEGORY>() {
    // let category = match args.single_quoted::<CATEGORY>() {
        Ok(category) => category,
        Err(_) => {
            msg.reply(ctx, "Category invalid").await?;
            return Ok(())
        }
    };
    // let category = args.single_quoted::<String>()?;
    // let category = CATEGORY::MEMBERS;

    let id = &msg.id;

    let user_id = args.single_quoted::<String>()?;
    let name = args.single_quoted::<String>()?;
    let phrase = args.single_quoted::<String>()?;

    let quote = Quote::build(
        category,
        id.to_string(),
        user_id,
        name,
        phrase
    );

    let mut all_quotes = AllQuotes::json_to_vec_movies_by_server_id(&server_id);
    match all_quotes.add(quote) {
        true => {
            all_quotes.quotes_to_json_by_server_id(&server_id);
            msg.reply(ctx,"Quote added\n").await?;
        },
        false => {
            msg.reply(ctx,"Quote not added\nAlready exists\n").await?;
        }
    }
    Ok(())
}
