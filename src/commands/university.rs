extern crate serenity;

use serenity::{
    framework::standard::{
        macros::{command, group },
        Args, CommandGroup, CommandResult,
    },
    model::{
        channel::{Message, ReactionType},
        id::UserId,
    },
    prelude::*,
};

use crate::constantes::university;

#[group]
#[prefixes("uni","university")]
#[default_command(good_luck)]
#[commands(schedule_link, swap_link,good_luck, material_link)]
#[description = "Some university commands\nGood luck!!!"]
struct University;


#[command]
#[description = "Just a good luck\n"]
async fn good_luck(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, "GOOOOOD LUCK!!!!!\nWe can do it!!!\nUniversity of MINHOOOOOO").await?;
    Ok(())
}

#[command]
#[aliases(schedule,horario)]
#[description = "Link to the schedules of the university of Minho\n"]
async fn schedule_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, university::SCHEDULE).await?;
    Ok(())
}

#[command]
#[aliases(swap)]
#[description = "Link to the swap of the university of Minho\n"]
 async fn swap_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, university::SWAP).await?;
    Ok(())
}

#[command]
#[aliases(material)]
#[description = "Link to the material of the server's discord\n"]
 async fn material_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, university::MATERIAL).await?;
    Ok(())
}
