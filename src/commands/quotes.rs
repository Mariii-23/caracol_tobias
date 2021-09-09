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
#[help_available]
#[only_in(guilds)]
#[commands(add,show,me,build,number_quotes,show_all)]
#[description = "**Quotes are fun**\n\nWe have 3 category:\n**\
                 \"MEMBERS\"** -> quotes from people in the server\n**\
                 \"PROFS\"** -> quotes from profs\n **\
                 \"GENERAL\"** -> random phrases "]
#[default_command(show)]
#[prefixes("quotes","quote","q")]
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

            if phrase.is_empty() {
                msg.reply(ctx,"Quote not added\nQuote is empty\n").await?;
                return Ok(());
            }

            let quote = Quote::build(
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
#[aliases(members,member)]
#[description = "Add one quote to the server in category \"MEMBERS\"\n"]
#[example="\"Quote\""]
#[example="\"Quote\" @person"]
async fn add_members(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;

    if phrase.is_empty() {
        msg.reply(ctx,"Quote not added\nQuote is empty\n").await?;
        return Ok(());
    }

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
#[aliases(profs,prof)]
#[description = "Add one quote to the server in category \"PROFS\"\n"]
#[example="\"Quote\" \"profs\""]
async fn add_profs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;
    let nick = args.single_quoted::<String>()?;
    if phrase.is_empty() || nick.is_empty() {
        msg.reply(ctx,"Quote not added\nQuote is empty\n").await?;
        return Ok(());
    }

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
#[example="\"Quote\" \"description\""]
async fn add_general(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let phrase = args.single_quoted::<String>()?;
    let user_id = args.single_quoted::<String>()?;
    if phrase.is_empty() || user_id.is_empty() {
        msg.reply(ctx,"Quote not added\nQuote is empty\n").await?;
        return Ok(());
    }

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

/* show ramdom quotes */
#[command]
#[sub_commands(show_general,show_profs,show_members)]
#[description = "Show one quote\n"]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
    let person = &msg.mentions;
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if person.is_empty(){
        quotes = all_quotes.get_one_random_quote();
    } else {
        let id = person[0].id.to_string();
        quotes = all_quotes.get_by_user_id(id,CATEGORY::MEMBERS);
    }
    send_one_quote_randow(ctx, msg, quotes).await;
    Ok(())
}


#[command]
#[description = "Show one quote that is yours\n"]
async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let id = msg.author.id.to_string();
    let quotes= all_quotes.get_by_user_id(id, CATEGORY::MEMBERS);
    send_one_quote_randow(ctx, msg, quotes).await;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(general)]
#[description = "Show one quote in the category \"GENERAL\"\n"]
#[usage = "\"category\""]
async fn show_general(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if args.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::GENERAL);
    } else {
        let id = args.single_quoted::<String>()?;
        quotes = all_quotes.get_by_user_id(id,CATEGORY::GENERAL);
    }
    send_one_quote_randow(ctx,msg,quotes).await;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(profs)]
#[description = "Show one quote in the category \"PROFS\"\n"]
#[usage = "\"profs's name\""]
async fn show_profs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if args.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::PROFS);
    } else {
        let id = args.single_quoted::<String>()?;
        quotes = all_quotes.get_by_user_id(id,CATEGORY::PROFS);
    }
    send_one_quote_randow(ctx,msg,quotes).await;
    Ok(())
}

#[command]
#[max_args(1)]
#[aliases(members,member)]
#[description = "Show one quote in the category \"MEMBERS\"\n"]
#[usage = "@person"]
async fn show_members(ctx: &Context, msg: &Message) -> CommandResult {
    let person = &msg.mentions;
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if person.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::MEMBERS);
    } else {
        let id = person[0].id.to_string();
        quotes = all_quotes.get_by_user_id(id,CATEGORY::MEMBERS);
    }
    send_one_quote_randow(ctx,msg,quotes).await;
    Ok(())
}

/* show all quotes */

#[command]
#[aliases("all","ls","list")]
#[sub_commands(show_all_general,show_all_profs,show_all_members)]
#[description = "Show all quotes\n"]
async fn show_all(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    //TODO era suposto mandar 3 msg
    // show_all_members(ctx, msg, Args::new(&msg.content, &[Delimiter::Single('"')])).await?;
    // show_all_profs(ctx, msg, &Args::new(&msg.content, &[Delimiter::Single('"')])).await?;
    // show_all_general(ctx, msg, &Args::new(&msg.content, &[Delimiter::Single('"')])).await?;
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes = all_quotes.get_all_quotes();
    send_quotes_menu(ctx, msg, quotes).await;
    Ok(())
}

#[command]
#[aliases(general)]
#[max_args(1)]
#[usage = "\"category\""]
#[description = "Show all quotes in category GENERAL\n"]
async fn show_all_general(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if args.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::GENERAL);
    } else {
        let id = args.single_quoted::<String>()?;
        quotes = all_quotes.get_by_user_id(id,CATEGORY::GENERAL);
    }
    send_quotes_menu(ctx, msg, quotes).await;
    Ok(())
}

#[command]
#[aliases(members,member)]
#[max_args(1)]
#[usage = "@person"]
#[description = "Show all quotes in category MEMBERS\n"]
async fn show_all_members(ctx: &Context, msg: &Message) -> CommandResult {
    let person = &msg.mentions;
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if person.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::MEMBERS);
    } else {
        let id = person[0].id.to_string();
        quotes = all_quotes.get_by_user_id(id,CATEGORY::MEMBERS);
    }
    send_quotes_menu(ctx, msg, quotes).await;
    Ok(())
}

#[command]
#[aliases(profs,prof)]
#[max_args(1)]
#[usage = "\"profs's name\""]
#[description = "Show all quotes in category PROFS\n"]
async fn show_all_profs(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let all_quotes = AllQuotes::json_to_vec_movies(msg);
    let quotes: Option<Vec<&Quote>>;
    if args.is_empty() {
        quotes = all_quotes.get_all_quote_by_category(CATEGORY::PROFS);
    } else {
        let id = args.single_quoted::<String>()?;
        quotes = all_quotes.get_by_user_id(id,CATEGORY::PROFS);
    }
    send_quotes_menu(ctx, msg, quotes).await;
    Ok(())
}

/* quotes's number */

#[command]
#[sub_commands(number_quotes_general,number_quotes_members,number_quotes_profs)]
#[max_args(1)]
#[description = "Show the number of all quotes\nYou can mention someone and we will show ho many quotes,they have"]
#[usage = "@members"]
#[aliases("n","number")]
async fn number_quotes(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let person = &msg.mentions;
    let number: usize;
    if person.is_empty() {
        number = quotes.get_all_number_quotes();
    } else {
        let user_id = person[0].id.to_string();
        number = quotes.get_number_quotes_by_user_in_category(user_id, CATEGORY::MEMBERS);
    }
    let phrase = format!("Exists **{}** quote(s)",number);
    msg.reply(ctx, phrase).await?;
    Ok(())
}


#[command]
#[max_args(1)]
#[description = "Shows the number of all quotes in MEMBERS's Category\nYou can mention someone and see how many quotes that person have\n"]
#[usage = "@members"]
#[aliases("members,me")]
async fn number_quotes_members(ctx: &Context, msg: &Message) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let person = &msg.mentions;
    let number: usize;
    if person.is_empty() {
        number = quotes.get_number_quotes_in_category(CATEGORY::MEMBERS);
    } else {
        let user_id = person[0].id.to_string();
        number = quotes.get_number_quotes_by_user_in_category(user_id, CATEGORY::MEMBERS);
    }
    let phrase = format!("Exists **{}** quote(s)",number);
    msg.reply(ctx, phrase).await?;
    Ok(())
}


#[command]
#[max_args(1)]
#[description = "Shows the number of all quotes in Profs's Category\nYou can give one teacher's name and see how many quotes that teacher have\n"]
#[usage = "\"teacher's name\""]
#[aliases("profs")]
async fn number_quotes_profs(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let number: usize;
    if args.is_empty() {
        number = quotes.get_number_quotes_in_category(CATEGORY::PROFS);
    } else {
        let user_id = args.single_quoted::<String>()?;
        number = quotes.get_number_quotes_by_user_in_category(user_id, CATEGORY::PROFS);
    }
    let phrase = format!("Exists **{}** quote(s)",number);
    msg.reply(ctx, phrase).await?;
    Ok(())
}

#[command]
#[max_args(1)]
#[description = "Shows the number of all quotes in General's Category\nYou can give one description and see how many quotes exists\n"]
#[usage = "\"descrition\""]
#[aliases("general")]
async fn number_quotes_general(ctx: &Context, msg: &Message,mut args: Args) -> CommandResult {
    let quotes = AllQuotes::json_to_vec_movies(msg);
    let number: usize;
    if args.is_empty() {
        number = quotes.get_number_quotes_in_category(CATEGORY::GENERAL);
    } else {
        let user_id = args.single_quoted::<String>()?;
        number = quotes.get_number_quotes_by_user_in_category(user_id, CATEGORY::GENERAL);
    }
    let phrase = format!("Exists **{}** quote(s)",number);
    msg.reply(ctx, phrase).await?;
    Ok(())
}

/* developers */
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
        Ok(category) => category,
        Err(_) => {
            msg.reply(ctx, "Category invalid").await?;
            return Ok(())
        }
    };

    let id = &msg.id;

    let user_id = args.single_quoted::<String>()?;
    let name = args.single_quoted::<String>()?;
    let phrase = args.single_quoted::<String>()?;

    if phrase.is_empty() || user_id.is_empty() {
        msg.reply(ctx,"Quote not added\nQuote is empty\n").await?;
        return Ok(());
    }

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
