extern crate serenity;
use serenity::{
    builder::CreateMessage,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use serenity_utils::menu::Menu;

extern crate serde_json;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer, Map, Result, Value};

use crate::constantes::{EXTENSION_PATH, QUOTES_PATH};

use std::cmp::Ordering;
use std::error::Error;
use std::fs::{write, File};
use std::io::BufReader;
use std::path::Path;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Hash, Clone, Copy)]
pub enum CATEGORY {
    MEMBERS = 0,
    GENERAL = 1,
    PROFS = 2,
}

#[derive(Serialize, Deserialize, Debug, Hash)]
// #[derive(Eq, Serialize, Deserialize, Debug)]
pub struct Quote {
    pub category: CATEGORY,
    pub id: String,
    pub user_id: String,
    pub nick: String,
    pub quote: String,
    // pub date: DateTime<Utc>,
}

// impl PartialEq for Quote {
//     fn eq(&self, other: &Self) -> bool {
//         (self.id) == (other.id) && (self.quote.eq(&other.quote))
//     }
// }

impl Quote {
    pub fn build(
        category: CATEGORY,
        id: String,
        user_id: String,
        nick: String,
        quote: String,
        // date: DateTime<Utc>,
    ) -> Quote {
        Quote {
            category,
            id,
            user_id,
            nick,
            quote,
            // date,
        }
    }
}

// impl Ord for Quote {
//     // categora < id < date
//     fn cmp(&self, other: &Self) -> Ordering {
//     }
// }
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AllQuotes {
    // pub quotes: Option<serde_json::Map<CATEGORY, serde_json::Map<String, Vec<Quote>>>>,
    pub quotes: Option<HashMap<CATEGORY, HashMap<String, Vec<Quote>>>>,
}

impl AllQuotes {
    pub fn new() -> AllQuotes {
        AllQuotes { quotes: None }
    }

    pub fn from(quote: Quote) -> AllQuotes {
        // let mut§quotes me new = AllQuotes::new();
        // new.add(quote);
        // new
        let id = String::from(&quote.user_id);
        let category = &quote.category.clone();
        let vec_quote = vec![quote];
        let mut map_id = HashMap::new();
        map_id.insert(id, vec_quote);
        let mut map_category = HashMap::new();
        map_category.insert(*category, map_id);
        AllQuotes {
            quotes: Some(map_category),
        }
    }

    //TODO tem que ser &mut self
    // mas ta a dar erros
    // pub fn add(&mut self, quote: Quote) {
    //     let id = String::from(&quote.id);
    //     let category = &quote.category.clone();

    //     match self.quotes {
    //         Some(mut quotes) => match quotes.get_mut(category) {
    //             Some(map_category) => match map_category.get_mut(&id) {
    //                 Some(vec_quotes) => {
    //                     vec_quotes.push(quote);
    //                 }
    //                 None => {
    //                     let vec_quote = vec![quote];
    //                     map_category.insert(id, vec_quote);
    //                 }
    //             },
    //             None => {
    //                 let vec_quote = vec![quote];
    //                 let mut map_id = HashMap::new();
    //                 map_id.insert(id, vec_quote);
    //                 quotes.insert(*category, map_id);
    //             }
    //         },
    //         None => {
    //             *self = AllQuotes::from(quote);
    //         }
    //     }
    // }

    /** Verifica se uma dada quote já se encontra guardada
     * Ele apenas verifica na categoria da quote dada e do seu id,
     * nao sei se o melhor seria ir ver a todas as frases de quotes
     * (isso seria ao gosto dos utilizadores)*/
    pub fn eq_quote(&self, other: &Quote) -> bool {
        match &self.quotes {
            None => (),
            Some(map_category) => match map_category.get(&other.category) {
                None => (),
                Some(map_id) => match map_id.get(&other.user_id) {
                    None => (),
                    Some(quotes) => {
                        for quote in quotes {
                            if quote.quote.to_lowercase().eq(&other.quote.to_lowercase()) {
                                return true;
                            }
                        }
                    }
                },
            },
        }
        false
    }

    //TODO remover por id de quote
    // pub fn remove_by_id(&self, id: String) -> bool {
    //     let mut fail = false;
    //     match &self.quotes {
    //         None => (),
    //         Some(map_category) => {
    //             for map_id in map_category.values() {
    //                 for vec_quotes in map_id.values() {
    //                     for quote in vec_quotes {
    //                         if quote.id == id {
    // remover quote
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     fail
    // }

    pub fn get_by_id(&self, id: String) -> Option<Vec<&Quote>> {
        let mut quotes: Vec<&Quote> = Vec::new();
        match &self.quotes {
            None => (),
            Some(map_category) => {
                for map_id in map_category.values() {
                    for vec_quotes in map_id.values() {
                        for quote in vec_quotes {
                            if quote.id == id {
                                quotes.push(quote);
                            }
                        }
                    }
                }
            }
        }

        if quotes.is_empty() {
            None
        } else {
            Some(quotes)
        }
    }

    pub fn get_by_user_id(&self, id: String) -> Option<&Vec<Quote>> {
        match &self.quotes {
            None => (),
            Some(map_category) => match map_category.get(&CATEGORY::MEMBERS) {
                None => (),
                Some(map_id) => match map_id.get(&id) {
                    None => (),
                    Some(quotes) => return Some(quotes),
                },
            },
        }
        None
    }

    pub fn quotes_to_json(&self, msg: &Message) {
        let mut path = String::from(QUOTES_PATH);
        path.push_str(msg.guild_id.unwrap().to_string().as_str());
        path.push_str(EXTENSION_PATH);

        let quotes = serde_json::to_string(&self).unwrap();
        write(path, &quotes).expect("Error write Movies on json file");
    }

    pub fn json_to_vec_movies(msg: &Message) -> AllQuotes {
        let quotes = AllQuotes::new();
        let mut path = String::from(QUOTES_PATH);
        path.push_str(msg.guild_id.unwrap().to_string().as_str());
        path.push_str(EXTENSION_PATH);
        //Abrir o ficheiro e passar tudo para um BuffReader (é mais rapido do que passar para string)
        let f = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                File::create(path).unwrap();
                return quotes;
            }
        };
        let buf_reader = BufReader::new(f);

        let quotes: AllQuotes = match serde_json::from_reader(buf_reader) {
            Ok(quotes) => quotes,
            Err(err) => {
                println!("\nError reading json file {} :\n {}", path, err);
                AllQuotes::new()
            }
        };
        quotes
    }
}
