extern crate serenity;
mod constantes;
mod modules;
mod commands;

use modules::events;
use commands::{general::HELP, EXAMPLES_GROUP, ADMINS_GROUP, EMOJI_GROUP, GENERAL_GROUP, MOVIES_GROUP};

use std::{env,collections::HashSet};
use serenity::{
    framework::standard::StandardFramework,
    prelude::*,
    http::Http,
};

use tracing::{error, instrument};

// fn read_file_and_get_token() -> String {
//     let mut file = File::open(".env").unwrap();
//     let mut token = String::new();
//     file.read_to_string(&mut token)
//         .expect("Token file not found");
//     token
// }

#[tokio::main]
#[instrument]
 async fn main() {
    // Configure the client with your Discord bot token in the environment.
     kankyo::load(false).expect("Failed to load .env file");
     tracing_subscriber::fmt::init();

     let token = env::var("DISCORD_TOKEN").expect("No token found!");
     // let prefix = env::var("PREFIX").expect("No prefix found!");
    // let database_credentials = env::var("DATABASE_URL").expect("No database credentials found!");
    // let lastfm_api_key = env::var("LASTFM_TOKEN").expect("No lastfm api key found!");
    // let lastfm_secret = env::var("LASTFM_SECRET").expect("No lastfm secret found!");

    let http = Http::new_with_token(&token);

    //    let token = read_file_and_get_token();
    // let http = Http::new_with_token(&token);

    // let mut client = Client::new(&token)
    //     .event_handler(Handler)
    //     .await
    //     .expect("Err creating client");

    // if let Err(why) = client.start().await {
    //     println!("Client error: {:?}", why);
    // }

     // We will fetch your bot's owners and id
     let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

     // Init commands
     let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix(constantes::PREFIX)
                .on_mention(Some(bot_id))
                .case_insensitivity(true)
        })
        .after(events::after_hook)
        .before(events::before_hook)
        .on_dispatch_error(events::dispatch_error)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&EMOJI_GROUP)
        .group(&ADMINS_GROUP)
        .group(&EXAMPLES_GROUP)
        .group(&MOVIES_GROUP)
         ;

     let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(events::Handler)
        // .intents(GatewayIntents::all())
        .await
        .expect("Error creating client!");

     // Start
     if let Err(why) = client.start_autosharded().await {
        error!("An error occurred while running the client: {:?}", why);
     }
// pub struct ShardMessenger { /* fields omitted */ }
// use serenity::model::user::OnlineStatus;

//      client.set_status(OnlineStatus::DoNotDisturb);
//      use serenity::model::gateway::Activity;
     // client.set_activity(Some(Activity::playing("Heroes of the Storm")));
}
