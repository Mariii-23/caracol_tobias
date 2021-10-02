// #![crate_name = "Quotes Structs"]
extern crate serenity;
use serenity::{
    builder::CreateMessage,
    model::channel::Message,
    prelude::Context,
    utils::Colour
};
use serenity_utils::menu::Menu;

extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::constantes::{EXTENSION_PATH, QUOTES_PATH,SHOW_QUOTES_PER_PAGE};
use crate::modules::{
    function_aux::get_name_user_by_id,
    pagination
};

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
/// The fields are private to ensure that after their creation the "contract"
/// is still maintained, and none of the fields are empty.
pub struct Quote {
    /// One quote have a category
    category: CATEGORY,
    /// One quote must have one unique id
    id: String,
    /// user id of the person who said the quote
    user_id: String,
    /// nick of the person who said the quote
    nick: String,
    /// Quote :)
    quote: String,
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

        if id.is_empty() || user_id.is_empty() || quote.is_empty() {
            panic!("Some of the given values are empty! Error creating Quote")
        }

        Quote {
            category,
            id,
            user_id,
            nick,
            quote,
        }
    }

    // pub fn clone(&self) -> Quote {
    //     Quote::build(
    //         self.category,
    //                  String::from(&self.id) ,
    //                  String::from(&self.user_id) ,
    //                  String::from(&self.nick) ,
    //                  String::from(&self.quote))
    // }


    pub fn id(&self) -> String {
       String::from(&self.id)
    }
    pub fn user_id(&self) -> String {
       String::from(&self.user_id)
    }
    pub fn nick(&self) -> String {
       String::from(&self.nick)
    }
    pub fn quote(&self) -> String {
       String::from(&self.quote)
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
    fn remove_by_id_members(&mut self, id: &String) -> Option<Quote> {
        match &mut self.members {
            None => (),
            Some(map_members) => {
                for (_,vec_quotes) in map_members.iter_mut() {
                    for (index, quote) in vec_quotes.iter().enumerate() {
                        if quote.id == id.as_str() {
                            return Some(vec_quotes.remove(index));
                        }
                    }
                }
            }
        }
        None
    }

    fn remove_by_id_general(&mut self, id: &String) -> Option<Quote> {
        match &mut self.general {
            None => (),
            Some(vec_quotes) => {
                for (index, quote) in vec_quotes.iter().enumerate() {
                    if quote.id == id.as_str() {
                        return Some(vec_quotes.remove(index));
                    }
                }
            }
        }
        None
    }

    fn remove_by_id_profs(&mut self, id: &String) -> Option<Quote> {
        match &mut self.profs {
            None => (),
            Some(map_profs) => {
                for (_,vec_quotes) in map_profs.iter_mut() {
                    for (index, quote) in vec_quotes.iter().enumerate() {
                        if quote.id == id.as_str() {
                            return Some(vec_quotes.remove(index));
                        }
                    }
                }
            }
        }
        None
    }

    //TODO Fazer : remover por id de quote
    pub fn remove_by_id(&mut self, id: &String) -> Option<Quote> {
        match self.remove_by_id_general(id) {
            None => {
                match self.remove_by_id_members(id) {
                    None => self.remove_by_id_profs(id),
                    Some(quote) => Some(quote),
                }
            },
            Some(quote) => Some(quote),
        }
    }

    pub fn remove_by_id_with_category(&mut self, category: CATEGORY, id: &String) -> Option<Quote> {
        match category {
            CATEGORY::MEMBERS => self.remove_by_id_members(id),
            CATEGORY::GENERAL => self.remove_by_id_general(id),
            CATEGORY::PROFS => self.remove_by_id_profs(id),
        }
    }

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

    pub fn get_all_quotes(&self) -> Option<Vec<&Quote>> {
        let quotes_members = self.get_all_quote_by_category(CATEGORY::MEMBERS);
        let quotes_general = self.get_all_quote_by_category(CATEGORY::GENERAL);
        let quotes_profs = self.get_all_quote_by_category(CATEGORY::PROFS);

        let mut quotes: Vec<&Quote> = Vec::new();
        match quotes_members {
            None => (),
            Some(members) => quotes.extend(members),
        };

        match quotes_general {
            None => (),
            Some(general) => quotes.extend(general),
        };

        match quotes_profs {
            None => (),
            Some(profs) => quotes.extend(profs),
        };

        if quotes.is_empty() {
            None
        } else {
            Some(quotes)
        }
    }


    /*---------- get ONE quote by category ------*/
    /* get one quote by user id */
    // /// Returns one quote through a given user id and category
    // pub async fn get_one_quote_by_user_id_to_string(&self, ctx: &Context, msg: &Message,id: String, category: CATEGORY) -> String {
    //     let quotes= self.get_by_user_id(id, category);
    //     get_one_quote_to_string(ctx, msg, quotes).await
    // }

    /* get one quote by category */
    // /// Returns one quote through a given category
    // pub async fn get_one_quote_by_category_to_string(&self, ctx: &Context, msg: &Message,category: CATEGORY) -> String {
    //       let quotes = self.get_all_quote_by_category(category);
    //       get_one_quote_to_string(ctx, msg, quotes).await
    // }

    // /// Returns a quote from one of the existing categories
    // pub async fn get_one_quote_to_string(&self, ctx: &Context, msg: &Message) -> String {
    //     let number = rand::thread_rng().gen_range(0,3);
    //     let quotes = match number {
    //         0 => self.get_all_quote_by_category(CATEGORY::MEMBERS),
    //         1 => self.get_all_quote_by_category(CATEGORY::GENERAL),
    //         2 => self.get_all_quote_by_category(CATEGORY::PROFS),
    //         _ => None,
    //     };
    //     get_one_quote_to_string(ctx, msg, quotes).await
    // }

    /// Returns a quote from one of the existing categories
    pub fn get_one_random_quote(&self) -> Option<Vec<&Quote>> {
        let number = rand::thread_rng().gen_range(0,3);
        match number {
            0 => self.get_all_quote_by_category(CATEGORY::MEMBERS),
            1 => self.get_all_quote_by_category(CATEGORY::GENERAL),
            2 => self.get_all_quote_by_category(CATEGORY::PROFS),
            _ => None,
        }
    }

    /*---------- Number of quotes ------*/

    /*---------- Number of all quotes by category ------*/
    fn get_number_quotes_in_members(&self) -> usize {
        match &self.members {
            None => 0,
            Some(map_id) => {
                let mut count = 0;
                for vec in map_id.values() {
                    count += vec.len();
                };
                count
            }
        }
    }


    fn get_number_quotes_in_profs(&self) -> usize {
        match &self.profs {
            None => 0,
            Some(map_id) => {
                let mut count = 0;
                for vec in map_id.values() {
                    count += vec.len();
                };
               count
            }
        }
    }

    fn get_number_quotes_in_general(&self) -> usize {
        match &self.general {
            None => 0,
            Some(vec_quotes) => {
                vec_quotes.len()
            }
        }
    }

    pub fn get_number_quotes_in_category(&self, category: CATEGORY) -> usize {
        match category {
            CATEGORY::MEMBERS => self.get_number_quotes_in_members(),
            CATEGORY::GENERAL => self.get_number_quotes_in_general(),
            CATEGORY::PROFS => self.get_number_quotes_in_profs(),
        }
    }
    /*---------- Number of all quotes ------*/

    pub fn get_all_number_quotes(&self) -> usize {
        self.get_number_quotes_in_general() +
        self.get_number_quotes_in_members() +
        self.get_number_quotes_in_profs()
    }

    /*---------- Number of quotes by category and user id ------*/
    fn get_number_quotes_by_user_id_in_members(&self, user_id: String) -> usize {
        match &self.members {
            None => 0,
            Some(map_id) => match map_id.get(&user_id) {
                None => 0,
                Some(quotes) =>{
                    quotes.len()
                }
            },
        }
    }


    fn get_number_quotes_by_user_id_in_profs(&self, user_id: String) -> usize {
        match &self.profs {
            None => 0,
            Some(map_id) => match map_id.get(&user_id) {
                None => 0,
                Some(quotes) =>{
                    quotes.len()
                }
            },
        }
    }

    fn get_number_quotes_by_user_id_in_general(&self, user_id: String) -> usize {
        match &self.general {
            None => 0,
            Some(vec_quotes) => {
                let mut count = 0;
                for quote in vec_quotes {
                    if quote.user_id.eq(&user_id) {
                        count += 1;
                    }
                }
                count
            }
        }
    }

    pub fn get_number_quotes_by_user_in_category(&self,user_id: String, category: CATEGORY) -> usize {
        match category {
            CATEGORY::MEMBERS => self.get_number_quotes_by_user_id_in_members(user_id),
            CATEGORY::GENERAL => self.get_number_quotes_by_user_id_in_general(user_id),
            CATEGORY::PROFS => self.get_number_quotes_by_user_id_in_profs(user_id),
        }
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

        let quotes = serde_json::to_string_pretty(&self).unwrap();
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

// fn remove_by_id_in_vec_quotes(&mut vec_quotes: Vec<Quote>, id: String) -> Option<Quote> {
//     for (index, quote) in vec_quotes.iter().enumerate() {
//         if quote.id == id {
//             return Some(vec_quotes.remove(index));
//         }
//     }
//     None
// }


/* function aux */
/// Choose a random quote from given quote vector and convert it to string
// async fn get_one_quote_to_string(ctx: &Context, msg: &Message, vec: Option<Vec<&Quote>>) -> String {
//     match vec {
//         None => String::from("No quotes found"),
//         Some(vec) => {
//             let len = vec.len();
//             let quote = vec.get(rand::thread_rng().gen_range(0, len));
//             match quote {
//                 None => String::from("No quotes found"),
//                 Some(quote) => {
//                     // println!("{}",&quote.quote);
//                     let name = get_name_user_by_id(msg, ctx, &quote.user_id).await;
//                     if (name.is_empty() || quote.nick.eq(&name)){
//                         format!("> {}\n> {}", quote.quote, quote.nick)
//                     } else {
//                         format!("> {}\n> {} ({})", quote.quote, quote.nick, name)
//                     }
//                 }
//             }
//         }
//     }
// }


// async fn get_one_quote_randow(ctx: &Context, msg: &Message, vec: Option<Vec<Quote>>) -> Option<Quote> {
//     match vec {
//         None => None,
//         Some(vec) => {
//             let len = vec.len();
//             let quote = vec.get(rand::thread_rng().gen_range(0, len));

//             match quote {
//                 None => None,
//                 Some(quote) => {
//                     Some(quote.clone())
//                 }
//             }
//         }
//     }
// }

pub async fn send_one_quote_randow(ctx: &Context, msg: &Message, vec: Option<Vec<&Quote>>) {
    match vec {
        None => {
            msg.reply(ctx, "No quotes found").await.unwrap();
        },
        Some(vec) => {
            let len = vec.len();
            let quote = vec.get(rand::thread_rng().gen_range(0, len));

            match quote {
                None => {
                    msg.reply(ctx, "No quotes found").await.unwrap();
                },
                Some(quote) => {
                    let person = &msg.mentions;
                    let name = get_name_user_by_id(msg, ctx, &quote.user_id).await;
                    let msg = msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.colour(Colour::BLITZ_BLUE);

                            e.title(&quote.quote);
                            // e.description(format!("**{}**",&quote.quote));

                            let phrase: String;
                            if name.is_empty() || quote.nick.eq(&name) {
                                phrase = String::from(&quote.nick);
                            } else {
                                phrase = format!("{} ({})", quote.nick, name);
                            }

                            if person.is_empty(){
                                e.footer(|f| f.text(phrase));
                            }else{
                                e.footer(|f|
                                         f.icon_url(person[0].avatar_url().unwrap())
                                         .text(phrase));
                            }
                            e
                        });
                        m
                    });
                    msg.await.unwrap();
                }
            }
        }
    }
}

pub async fn send_quotes_menu(ctx: &Context, msg: &Message, vec: Option<Vec<&Quote>>) {
    match vec {
        None => {
            msg.reply(ctx, "No quotes found" ).await.unwrap();
        },
        Some(quotes) => {
            let n_quotes = SHOW_QUOTES_PER_PAGE;
            let len = quotes.len();
            let n_full_pages= len / n_quotes;
            let rest = len - n_full_pages*n_quotes;
            let mut pages = Vec::new();
            if n_full_pages != 0 {
                for i in 0..n_full_pages {
                    let mut page = CreateMessage::default();
                    page.content("").embed(|e| {
                        e.colour(Colour::BLITZ_BLUE);
                        for l in 0..n_quotes{
                            let quote = &quotes[i*n_quotes+l];
                            e.field(&quote.quote,&quote.nick,false);
                        }
                        e
                    });
                    pages.push(page);
                }
            }

            if rest != 0 {
                let mut page = CreateMessage::default();
                page.content("").embed(|e| {
                    e.colour(Colour::BLITZ_BLUE);
                    for l in 0..rest {
                        let quote = &quotes[l+n_full_pages*n_quotes];
                        e.field(&quote.quote,&quote.nick,false);
                    }
                    e
                });
                pages.push(page);
            }
            let menu = Menu::new(ctx, msg, &pages, pagination::simple_options());
            menu.run().await.unwrap();
        }
    }
}
