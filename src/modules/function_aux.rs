
use std::collections::HashMap;


extern crate serenity;
use serenity::{builder::CreateMessage,framework::standard::{
        macros::{command, group},
         CommandResult,
}, model::channel::Message, prelude::*};


pub async fn init_hashmap (msg: &Message, ctx: &Context) -> HashMap<String, String> {
    let mut hash = HashMap::new();
    let members = msg.guild_id.unwrap().members(&ctx.http, None, None).await.expect("Falha aqui não sei pq");
    for member in members {
        hash.insert(member.user.id.to_string(), member.user.name.to_string());
    }
    hash
}

pub async fn get_name_user_by_id(msg: &Message, ctx: &Context,id: &String) -> String {

    let mut names = init_hashmap(msg, ctx).await;
    let guild = msg.guild_id.unwrap();//.members(&ctx.http, None, None).await.expect("Falha aqui não sei pq");
    let members = guild.members(&ctx.http, Some(100), None).await.expect("Error") ;
    for member in members {
        names.insert(member.user.id.to_string(), member.user.name.to_string());
    }

    match names.get(id) {
        None => String::new(),
        Some(value) => value.to_string(),
    }
}
