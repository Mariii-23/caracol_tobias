extern crate serenity;
use serenity::{model::channel::Message, prelude::Context};

extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::constantes::{EXTENSION_PATH, QUOTES_PATH};
use crate::modules::function_aux::get_name_user_by_id;

use std::fs::{write, File};
use std::io::BufReader;

use rand::Rng;

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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AllQuotes {
//     // maybe alterar o hahMap<Categoru,...
//     // para um array visto cada category ter um numero distinto
//     pub quotes: Option<HashMap<CATEGORY, HashMap<String, Vec<Quote>>>>,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct AllQuotes {
    pub members: Option<HashMap<String, Vec<Quote>>>,
    pub general: Option<Vec<Quote>>,
    pub profs: Option<HashMap<String, Vec<Quote>>>,
}

impl AllQuotes {
    pub fn new() -> AllQuotes {
        AllQuotes {
            members: None,
            general: None,
            profs: None,
        }
    }

    /*----------------- Add quotes --------------------------*/
    fn add_members(&mut self, quote: Quote) {
        let id = String::from(&quote.user_id);

        match &mut self.members {
            Some(map_category) => match map_category.get_mut(&id) {
                Some(vec_quotes) => {
                    vec_quotes.push(quote);
                }
                None => {
                    let vec_quote = vec![quote];
                    map_category.insert(id, vec_quote);
                }
            },
            None => {
                let vec_quote = vec![quote];
                let mut map_id = HashMap::new();
                map_id.insert(id, vec_quote);
                self.members = Some(map_id);
            }
        };
    }

    fn add_profs(&mut self, quote: Quote) {
        let id = String::from(&quote.user_id);

        match &mut self.profs {
            Some(map_category) => match map_category.get_mut(&id) {
                Some(vec_quotes) => {
                    vec_quotes.push(quote);
                }
                None => {
                    let vec_quote = vec![quote];
                    map_category.insert(id, vec_quote);
                }
            },
            None => {
                let vec_quote = vec![quote];
                let mut map_id = HashMap::new();
                map_id.insert(id, vec_quote);
                self.profs = Some(map_id);
            }
        };
    }

    fn add_general(&mut self, quote: Quote) {
        match &mut self.general {
            None => self.general = Some(vec![quote]),
            Some(quotes) => quotes.push(quote),
        }
    }

    pub fn add(&mut self, quote: Quote) {
        match quote.category {
            CATEGORY::MEMBERS => self.add_members(quote),
            CATEGORY::PROFS => self.add_profs(quote),
            CATEGORY::GENERAL => self.add_general(quote),
            // _ => (),
        };
    }

    /* ----------- remove ------------- */
    //TODO Fazer : remover por id de quote
    // pub fn remove_by_id(&mut self, id: String) -> bool {
    //     let mut fail = false;
    //     match &mut self.quotes {
    //         None => (),
    //         Some(map_category) => {
    //             for (_key, map_id) in map_category.iter_mut() {
    //                 for (_key_id, vec_quotes) in map_id.iter_mut() {
    //                     for (index, quote) in vec_quotes.iter().enumerate() {
    //                         if quote.id == id {
    //                             fail = true;
    //                             vec_quotes.remove(index);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     fail
    // }

    /*----------------- Get quotes by user id --------------------------*/
    fn get_in_members_by_user_id(&self, id: String) -> Option<Vec<&Quote>> {
        match &self.members {
            None => (),
            Some(map_id) => match map_id.get(&id) {
                None => (),
                Some(quotes) =>{
                    let mut  vec = Vec::new();
                    for quote in quotes {
                      vec.push(quote)  ;
                    };
                    return Some(vec)
                }
            },
        }
        None
    }

    fn get_in_profs_by_user_id(&self, id: String) -> Option<Vec<&Quote>> {
        match &self.profs {
            None => (),
            Some(map_id) => match map_id.get(&id) {
                None => (),
                Some(quotes) =>{
                    let mut  vec = Vec::new();
                    for quote in quotes {
                      vec.push(quote)  ;
                    };
                    return Some(vec)
                }
            },
        }
        None
    }

    fn get_in_general_by_user_id(&self, id: String) -> Option<Vec<&Quote>> {
        match &self.general {
            None => (),
            Some(vec_quotes) => {
                let mut vec = Vec::new();
                for quote in vec_quotes {
                    if quote.user_id.eq(&id) {
                        vec.push(quote);
                    }
                }
                if !vec.is_empty() {
                    return Some(vec);
                }
            }
        }
        None
    }

    pub fn get_by_user_id(&self, id: String, category: CATEGORY) -> Option<Vec<&Quote>> {
        match category {
            CATEGORY::MEMBERS => self.get_in_members_by_user_id(id),
            CATEGORY::GENERAL => self.get_in_general_by_user_id(id),
            CATEGORY::PROFS => self.get_in_profs_by_user_id(id),
        }
    }

    /*----------------- Get quotes by user id --------------------------*/

    /*----------------- Get all quotes --------------------------*/
    fn get_all_members_quotes(&self) -> Option<Vec<&Quote>> {
        match &self.members {
            None => None,
            Some(map_id) => {
                let mut vec = Vec::new();
                for all_quotes in map_id.values() {
                    vec.extend(all_quotes);
                }
                Some(vec)
            }
        }
    }


    fn get_all_profs_quotes(&self) -> Option<Vec<&Quote>> {
        match &self.profs {
            None => None,
            Some(map_id) => {
                let mut vec = Vec::new();
                for all_quotes in map_id.values() {
                    vec.extend(all_quotes);
                }
                Some(vec)
            }
        }
    }

    fn get_all_general_quotes(&self) -> Option<Vec<&Quote>> {
        match &self.general {
            None => None,
            Some(quotes) => {
                let mut vec = Vec::new();
                for quote in quotes{
                    vec.push(quote);
                }
                Some(vec)
            }
        }
    }

    pub fn get_all_quote_by_category(&self,category: CATEGORY) -> Option<Vec<&Quote>> {
        match category {
            CATEGORY::MEMBERS => self.get_all_members_quotes(),
            CATEGORY::GENERAL => self.get_all_general_quotes(),
            CATEGORY::PROFS => self.get_all_profs_quotes(),
        }
    }

    /*---------- get ONE quote by category ------*/
    /* get one quote by user id */
    pub async fn get_one_quote_by_user_id_to_string(&self, ctx: &Context, msg: &Message,id: String, category: CATEGORY) -> String {
        let quotes= self.get_by_user_id(id, category);
        get_one_quote_to_string(ctx, msg, quotes).await
    }

    /* get one quote by category */
    pub async fn get_one_quote_by_category_to_string(&self, ctx: &Context, msg: &Message,category: CATEGORY) -> String {
          let quotes = self.get_all_quote_by_category(category);
          get_one_quote_to_string(ctx, msg, quotes).await
    }

    pub async fn get_one_quote_to_string(&self, ctx: &Context, msg: &Message) -> String {
        let number = rand::thread_rng().gen_range(0,3);
        let quotes = match number {
            0 => self.get_all_quote_by_category(CATEGORY::MEMBERS),
            1 => self.get_all_quote_by_category(CATEGORY::GENERAL),
            2 => self.get_all_quote_by_category(CATEGORY::PROFS),
            _ => None,
        };
        get_one_quote_to_string(ctx, msg, quotes).await
    }

    /*----------------- Struct «--» json --------------------------*/

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

/* function aux */
async fn get_one_quote_to_string(ctx: &Context, msg: &Message, vec: Option<Vec<&Quote>>) -> String {
    match vec {
        None => String::from("No quotes found"),
        Some(vec) => {
            let len = vec.len();
            let quote = vec.get(rand::thread_rng().gen_range(0, len));
            match quote {
                None => String::from("No quotes found"),
                Some(quote) => {
                    // println!("{}",&quote.quote);
                    let name = get_name_user_by_id(msg, ctx, &quote.user_id).await;
                    format!("\"{}\" - {} ({})", quote.quote, quote.nick, name)
                }
            }
        }
    }
}

