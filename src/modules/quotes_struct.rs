#![crate_name = "Quotes Structs"]
extern crate serenity;
use serenity::{model::channel::Message, prelude::Context};

extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::constantes::{EXTENSION_PATH, QUOTES_PATH};
use crate::modules::function_aux::get_name_user_by_id;

use std::{
    fs::{write, File},
    io::BufReader,
    str::FromStr,
    collections::HashMap,
};

use rand::Rng;

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Hash, Clone, Copy)]
/// Quotes can have a category
pub enum CATEGORY {
    /// Represents people in a server
    MEMBERS = 0,
    /// Represents things in general
    GENERAL = 1,
    /// Represents teachers
    PROFS = 2,
}

impl FromStr for CATEGORY {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "MEMBERS" => Ok(CATEGORY::MEMBERS),
            "GENERAL" => Ok(CATEGORY::GENERAL),
            "PROFS" => Ok(CATEGORY::PROFS),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash)]
// #[derive(Eq, Serialize, Deserialize, Debug)]
/// Represents of a quote
pub struct Quote {
    /// One quote have a category
    pub category: CATEGORY,
    /// One quote must have one unique id
    pub id: String,
    /// user id of the person who said the quote
    pub user_id: String,
    /// nick of the person who said the quote
    pub nick: String,
    /// Quote :)
    pub quote: String,
}

impl Quote {
    /// Returns one quote with the data fields
    /// # Arguments
    ///
    /// * `category` - One of the possible categories
    /// * `id` - unique id
    /// * `user_id` - user id of the person who said the quote
    /// * `nick` - nick of the person who said the quote
    /// * `quote` - phrase
    ///
    /// # Examples
    ///
    /// ```
    /// use quotes_struct::Quote;
    /// let quote = Quote::build(CATEGORY::MEMBERS,String::new("1234"),String::new("5678"),String::new("Caracol Tobias"),String::new("Hello World"));
    /// ```
    pub fn build(
        category: CATEGORY,
        id: String,
        user_id: String,
        nick: String,
        quote: String,
    ) -> Quote {
        Quote {
            category,
            id,
            user_id,
            nick,
            quote,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Struct that stores all quotes by categories
pub struct AllQuotes {
    /// Stores the quotes in members's category with a HashMap, where the key is the user's id and the values is all their quotes.
    pub members: Option<HashMap<String, Vec<Quote>>>,
    /// Stores the quotes in general's category with a vector
    pub general: Option<Vec<Quote>>,
    /// Stores the quotes in profs's category with a HashMap, where the key is the teacher's name(user id) and the values is all their quotes.
    pub profs: Option<HashMap<String, Vec<Quote>>>,
}

impl AllQuotes {
    /// Returns a new AllQuotes empty
    pub fn new() -> AllQuotes {
        AllQuotes {
            members: None,
            general: None,
            profs: None,
        }
    }

    /*----------------- Add quotes --------------------------*/
    fn add_members(&mut self, quote: Quote)-> bool {
        let id = String::from(&quote.user_id);

        match &mut self.members {
            Some(map_category) => match map_category.get_mut(&id) {
                Some(vec_quotes) => {
                    // validation
                    for elem in vec_quotes.iter() {
                        if elem.id.to_lowercase().eq(&quote.id.to_lowercase()) ||
                            elem.quote.to_lowercase().eq(&quote.quote.to_lowercase()) {
                                return false;
                            }
                    }

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
        true
    }

    fn add_profs(&mut self, quote: Quote) -> bool {
        let id = String::from(&quote.user_id);

        match &mut self.profs {
            Some(map_category) => match map_category.get_mut(&id) {
                Some(vec_quotes) => {
                    // validation
                    for elem in vec_quotes.iter() {
                        if elem.id.to_lowercase().eq(&quote.id.to_lowercase()) ||
                            elem.quote.to_lowercase().eq(&quote.quote.to_lowercase()) {
                                return false;
                            }
                    }

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
        true
    }

    fn add_general(&mut self, quote: Quote) -> bool {
        match &mut self.general {
            None => self.general = Some(vec![quote]),
            Some(quotes) => {
                for elem in quotes.iter() {
                    if elem.id.to_lowercase().eq(&quote.id.to_lowercase()) ||
                        elem.quote.to_lowercase().eq(&quote.quote.to_lowercase()) {
                            return false;
                        }
                }
                quotes.push(quote)
            },
        }
        true
    }

    pub fn add(&mut self, quote: Quote) -> bool {
        match quote.category {
            CATEGORY::MEMBERS => self.add_members(quote),
            CATEGORY::PROFS => self.add_profs(quote),
            CATEGORY::GENERAL => self.add_general(quote),
            // _ => (),
        }
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

    /// Returns all quotes through a given user id and category
    pub fn get_by_user_id(&self, id: String, category: CATEGORY) -> Option<Vec<&Quote>> {
        match category {
            CATEGORY::MEMBERS => self.get_in_members_by_user_id(id),
            CATEGORY::GENERAL => self.get_in_general_by_user_id(id),
            CATEGORY::PROFS => self.get_in_profs_by_user_id(id),
        }
    }

    /*----------------- Get quotes by id --------------------------*/
    //TODO

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
    /// Returns one quote through a given user id and category
    pub async fn get_one_quote_by_user_id_to_string(&self, ctx: &Context, msg: &Message,id: String, category: CATEGORY) -> String {
        let quotes= self.get_by_user_id(id, category);
        get_one_quote_to_string(ctx, msg, quotes).await
    }

    /* get one quote by category */
    /// Returns one quote through a given category
    pub async fn get_one_quote_by_category_to_string(&self, ctx: &Context, msg: &Message,category: CATEGORY) -> String {
          let quotes = self.get_all_quote_by_category(category);
          get_one_quote_to_string(ctx, msg, quotes).await
    }

    /// Returns a quote from one of the existing categories
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

    /// Converts a struct `AllQuotes` to a json file titled with the server id contained in the given `Message`.
    pub fn quotes_to_json(&self, msg: &Message) {
        self.quotes_to_json_by_server_id(msg.guild_id.unwrap().to_string().as_str())
    }

    /// Converts a struct `AllQuotes` to a json file titled with the server id given.
    pub fn quotes_to_json_by_server_id(&self, server_id: &str) {
        let mut path = String::from(QUOTES_PATH);
        path.push_str(server_id);
        path.push_str(EXTENSION_PATH);

        let quotes = serde_json::to_string(&self).unwrap();
        write(path, &quotes).expect("Error write Movies on json file");
    }

    /// Converts a json file titled with the server id contained in the given `Message` to a struct `AllQuotes`.
    pub fn json_to_vec_movies(msg: &Message) -> AllQuotes {
        AllQuotes::json_to_vec_movies_by_server_id(msg.guild_id.unwrap().to_string().as_str())
    }

    /// Converts a json file titled with the server id given to a struct `AllQuotes`.
    pub fn json_to_vec_movies_by_server_id(server_id: &str) -> AllQuotes {
        let quotes = AllQuotes::new();
        let mut path = String::from(QUOTES_PATH);
        path.push_str(server_id);
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
/// Choose a random quote from given quote vector and convert it to string
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

