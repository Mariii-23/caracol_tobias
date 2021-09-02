extern crate serenity;

// use std::path::Path;
use crate::modules::pagination;

use serenity::{
    builder::CreateMessage,
    // http::AttachmentType,
    framework::standard::{
        macros::{command, group},
         CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use serenity_utils::menu::Menu;

#[group]
#[owners_only]
#[commands(ex1,menu)]
#[description = "Some examples of code\n"]
#[prefixes("ex","example")]
#[help_available]
struct Examples;

#[command]
#[description = "Example of a embed msg with several fields\n"]
async fn ex1( ctx: &Context, msg: &Message) -> CommandResult{
   // The create message builder allows you to easily create embeds and messages
   // using a builder syntax.
   // This example will create a message that says "Hello, World!", with an embed that has
   // a title, description, three fields, and a footer.
   let msg = msg
       .channel_id
       .send_message(&ctx.http, |m| {
           m.content("Hello, World!");
           m.embed(|e| {
               e.title("This is a title");
               e.description("This is a description");
               e.image("attachment://ferris_eyes.png");
               e.fields(vec![
                   ("This is the first field", "This is a field body", true),
                   ("This is the second field", "Both of these fields are inline", true),
               ]);
               e.field("This is the third field", "This is not an inline field", false);
               e.field("This is the third field", "This is not an inline field", false);
               e.field("This is the third field", "This is not an inline field", true);
               e.field("This is the third field", "This is not an inline field", true);
               e.field("This is the third field", "This is not an inline field", true);
               e.footer(|f| {
                   f.text("This is a footer");

                   f
               });

               e
           });
           // m.add_file(AttachmentType::Path(Path::new("./images/ferris_eyes.png")));
           m
   });

   msg.await.unwrap();

  Ok(())
}

#[command]
#[description = "Example of a menu\n"]
async fn menu(ctx: &Context, msg: &Message) -> CommandResult {

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
