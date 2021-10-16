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

use crate::constantes::university as uni_constants;
use crate::modules::university;
use university::*;

#[group]
#[prefixes("uni","university")]
#[default_command(good_luck)]
#[commands(schedule_link, swap_link,good_luck, material_link, add_subject,paddel_link)]
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
    msg.reply(&ctx, uni_constants::SCHEDULE).await?;
    Ok(())
}

#[command]
#[aliases(swap)]
#[description = "Link to the swap of the university of Minho\n"]
 async fn swap_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, uni_constants::SWAP).await?;
    Ok(())
}

#[command]
#[aliases(material)]
#[description = "Link to the material of the server's discord\n"]
 async fn material_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, uni_constants::MATERIAL).await?;
    Ok(())
}

#[command]
#[aliases(paddel,mnol,MNOL)]
#[description = "Link to the material of the server's discord\n"]
async fn paddel_link(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx, uni_constants::PADDEL).await?;
    Ok(())
}

#[command]
#[min_args(1)]
#[aliases(add)]
#[sub_commands(add_test,add_note)]
#[description = "Add one subject\n"]
 async fn add_subject(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let subject_name = args.single_quoted::<String>()?;

    let name_file = String::from(msg.guild_id.unwrap().to_string().as_str());
    let mut university = university::University::json_to_university(&name_file);
    university.add_subject(&subject_name);
    university.university_to_json(&name_file);
    Ok(())
}


#[command]
#[min_args(1)]
#[max_args(3)]
#[aliases(test)]
#[description = "Add one subject\n"]
 async fn add_test(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let subject_name = args.single_quoted::<String>()?;

    let name_file = String::from(msg.guild_id.unwrap().to_string().as_str());
    let mut university = university::University::json_to_university(&name_file);

     let percentage = if args.len() >=2 {
         match args.single_quoted::<f32>() {
             Ok(value) => value,
             Err(err) => {
                 println!("Error parsing the percentage\n{}",err);
                 msg.reply(ctx, "Error parsing the percentage!\nTest not added").await;
                 return Ok(());
             }
         }
     } else {
         uni_constants::MAX_PER
     };

     let mut evaluation = Evaluation::build(percentage, None);

     let mut phrase = String::new();
     if args.len() == 3 {
         let date = args.single_quoted::<String>()?;
         match evaluation.parse_date_from_str(date) {
             true => (),
             false => phrase.push_str("Error parsing the given date\n"),
         }
     }

     match university.add_test(subject_name, evaluation) {
         true => phrase.push_str("Subject add!"),
         false => phrase.push_str("Subject not add!"),
     }

    msg.reply(ctx, phrase).await;
    university.university_to_json(&name_file);
    Ok(())
}

#[command]
#[min_args(2)]
#[aliases(notes,note)]
#[description = "Add one subject\n"]
 async fn add_note(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let subject_name = args.single_quoted::<String>()?;
    let note = args.single_quoted::<String>()?;

    let name_file = String::from(msg.guild_id.unwrap().to_string().as_str());
    let mut university = university::University::json_to_university(&name_file);
     if subject_name.is_empty() || note.is_empty() {
         msg.reply(ctx, "The given arguments are with errors").await;
         return Ok(());
     }
    university.add_notes(subject_name, note);
    university.university_to_json(&name_file);
    msg.reply(ctx, "Note added with sucess").await;
    Ok(())
}
