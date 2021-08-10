extern crate serenity;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};

// use sqlx::Row;
// use serenity::model::prelude::*;
// use serenity::prelude::*;

// use crate::cmd_ctx_msg_args;

#[group]
#[owners_only]
#[commands(say)]
#[description = "Admin only\n"]
#[help_available]
// #[help_available(false)]
struct Admins;

#[command]
#[description = "The bot will repeat what you given\n"]
#[example = " Hello World\nBot: Hello World\n"]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.say(&ctx, args.rest()).await?;
    Ok(())
}

// #[command]
// #[description = "Gently terminates the bot process"]
// async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
//     let data = ctx.data.read().await;

//     if let Some(manager) = data.get::<ShardManagerContainer>() {
//         msg.channel_id.say(&ctx, "Goodbye!").await?;
//         manager.lock().await.shutdown_all().await;
//     } else {
//         msg.channel_id
//             .say(&ctx, "There was a problem getting the shard manager")
//             .await?;
//     }

//     Ok(())
// }

