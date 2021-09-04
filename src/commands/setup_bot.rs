extern crate serenity;

use serenity::{framework::standard::{
        macros::{ command, group},
        Args,
        CommandResult,
}, model::channel::Message, prelude::*};


#[group]
#[commands(listen,play,stream,watch,compete)]
#[help_available(false)]
#[prefixes("bot","setup")]
#[description = "Setup bot\n"]
struct Setup_bot;

use serenity::model::gateway::Activity;

// Setup the Bot's 'status
#[command]
async fn play(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::playing(&name)).await;

    Ok(())
}

#[command]
async fn listen(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::listening(&name)).await;

    Ok(())
}

#[command]
async fn stream(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    const STREAM_URL: &str = "...";

    let name = args.message();
    ctx.set_activity(Activity::streaming(&name, STREAM_URL)).await;

    Ok(())
}

#[command]
async fn watch(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::watching(&name)).await;

    Ok(())
}

#[command]
async fn compete(ctx: &Context, _msg: &Message, args: Args) -> CommandResult {
    let name = args.message();
    ctx.set_activity(Activity::competing(&name)).await;

    Ok(())
}
