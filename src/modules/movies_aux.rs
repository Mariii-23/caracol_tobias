use std::{cmp::Ordering, collections::HashMap, fs::{File,write}, io::BufReader};
use omdb::*;

use serenity::{builder::CreateMessage, client::Context, model::{channel::{Message, ReactionType}, guild::Emoji}};
use crate::constantes::{APIKEY,MOVIES_PATH,EXTENSION_PATH};

extern crate serde_json;
use serde::{Deserialize, Serialize};

use super::function_aux::init_hashmap;

#[derive(Eq, Serialize, Deserialize, Debug)]
//struct para cada linha do ficheiro (provavelmente vai ter que ser muito alterada)
pub struct Movie {
    pub title: String,
    pub people: Vec<String>,
    pub link_imdb: String,
    pub imdb_id: String
}

impl Movie {
    pub fn create_movie(title: String, people: Vec<String>, imdb_id: String) -> Movie {
        let mut link_imdb = String::from("https://www.imdb.com/title/");
        link_imdb.push_str(imdb_id.as_str());
        Movie {
            title,
            people,
            link_imdb,
            imdb_id
        }
    }

    pub fn search_title(movies: &mut Vec<Movie>, title: String) -> Result<&Movie, String> {
        for movie in movies{
            if movie.title.to_uppercase().eq(&title.to_uppercase()) {

                return Ok(movie);
            }
        }
        Err(title)
    }

    pub fn search_person(&self, id: &String) -> Result<String, String> {
        for person in &self.people {
            if id.eq(person) {
                return Err(id.to_owned());
            }
        }
        Ok(id.to_owned())
    }

    fn get_people(&self, names: &HashMap<String, String>) -> String {
        let mut people = String::new();
        for person in &self.people {
            let name = names.get(person).unwrap();
            let string = format!("{}\n", name);
            people.push_str(&string);
        }
        people
    }

}

impl Ord for Movie {
    fn cmp(&self, other: &Self) -> Ordering {
        self.people.len().cmp(&other.people.len())
    }
}

impl PartialOrd for Movie {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        (self.people.len()) == (other.people.len())
    }
}


// pub async fn search_by_name(name: String) -> Result<SearchResults, Error> {
//     omdb::search(name).apikey(APIKEY).get().await
// }

pub async fn movie_with_name(name: String) -> Result<omdb::Movie, Error> {
    println!("{}{:?}", name, omdb::title(&name).get().await);
    omdb::title(name).apikey(APIKEY).get().await
}

pub async fn movie_with_id(id: String) -> Result<omdb::Movie, Error> {
    println!("{}{:?}", id, omdb::title(&id).get().await);
    omdb::imdb_id(id).apikey(APIKEY).get().await
}

//Passar o ficheiro para um vetor de struct (Está pouco otimizada)
pub fn json_to_vec_movies(msg: &Message) -> Vec<Movie>{

    let movies = Vec::new();
    let mut path = String::from(MOVIES_PATH);
    path.push_str(msg.guild_id.unwrap().to_string().as_str());
    // path.push_str(".csv");
    path.push_str(EXTENSION_PATH);
    //Abrir o ficheiro e passar tudo para um BuffReader (é mais rapido do que passar para string)
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            File::create(path).unwrap();
            return movies;
        }
    };
    let buf_reader = BufReader::new(f);

    let movies: Vec<Movie> = match  serde_json::from_reader(buf_reader){
        Ok(movies) => movies,
        Err(err) => {
            println!("\nError reading json file {} :\n {}",path,err);
            Vec::new()
        }
    };
    // let mut contents = String::new();
    // //Agora passar o BuffReader para string
    // buf_reader.read_to_string(&mut contents).unwrap();
    // if contents.len() == 0 {
    //     return movies;
    // }
    // //Dividir o ficheiro em um vetor em que cada elemento é uma linha do ficheiro
    // let file: Vec<&str> = contents.split("\n").collect();
    // for f in &file {
    //     if f.len() == 0 {
    //         return movies;
    //     }
    //     let aux: Vec<&str> = f.split(";").collect();
    //     let p: Vec<&str> = aux[1].split(",").collect();
    //     let mut aux2 = Vec::new();
    // //     for a in p {
    //         aux2.push(a.to_string());
    //     }
    //     let imdb_id  = aux[2].to_string();
    //     let mut link_imdb = String::from("https://www.imdb.com/title/");
    //     link_imdb.push_str(imdb_id.as_str());
    //     let m = Movie {
    //         title: aux[0].to_string(),
    //         people: aux2,
    //         link_imdb,
    //         imdb_id
    //     };
    //     movies.push(m);
    // }
    movies
}

pub fn vec_movie_to_json(movies: Vec<Movie>, msg: &Message) {

    let mut path = String::from(MOVIES_PATH);
    path.push_str(msg.guild_id.unwrap().to_string().as_str());
    path.push_str(EXTENSION_PATH);

    // let mut file = match File::create(path){
    //     Ok(file) => file,
    //     Err(_) => panic!("Problema a abrir o ficheiro!"),
    // };

    //for i in movies {
    //    let mut line = String::new();
    //    line.push_str(i.title.as_str().trim());
    //    line.push_str(";");
    //    for (index, j) in i.people.iter().enumerate() {
    //        line.push_str(j.as_str().trim());
    //        if index != i.people.len() - 1 {
    //            line.push_str(",");
    //        }
    //    }
    //    line.push_str(";");
    //    line.push_str(i.imdb_id.as_str().trim());
    //    file.write(line.as_bytes()).expect("Erro ao ecrever no ficheiro!");
    //    file.write("\n".as_bytes()).expect("Erro no \n?");
    //}

    let movies = serde_json::to_string(&movies).unwrap();
    write(path,&movies).expect("Error write Movies on json file");
}

fn get_all_mv_titles(movies: &Vec<Movie>) -> String {
    let mut all_titles = String::new();
    for movie in movies {
        all_titles.push_str(&movie.title);
        all_titles = all_titles + "\n";
    }
    all_titles
}

pub async fn show_one_mv(msg: &Message, ctx: &Context, movie: &Movie, names: &HashMap<String, String>) {
    let id = &movie.imdb_id;
    println!("{}", id);
    let info = movie_with_id(id.to_owned()).await.unwrap();
    msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&movie.title);
                e.description(&movie.link_imdb);
                e.field("People", movie.get_people(names), true);
                e.image(info.poster);
                e.description(info.plot);

                e
            });
            m
    }).await.unwrap();
}

pub async fn show_all_mvs(msg: &Message, ctx: &Context, movies: &Vec<Movie>) {
    msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field("All movies", get_all_mv_titles(&movies), false);

                e
            });
            m
    }).await.unwrap();
}  

pub async fn show_mv_menu<'a>(movies: &Vec<Movie>, names: &HashMap<String, String>) -> Vec<CreateMessage<'a>> {
    let mut pages = Vec::new();

    for movie in movies {
        let id = &movie.imdb_id;
        println!("{}", id);
        let info = movie_with_id(id.to_owned()).await.unwrap();
        let people = movie.get_people(names);
        let mut page = CreateMessage::default();
        page.content("MOVIES").embed(|e| {
            e.title(&movie.title);
            if !movie.link_imdb.eq(""){
                e.description(&movie.link_imdb);
            }
            e.field("People:",&people,true);
            e.image(info.poster);
            e.description(info.plot);

         e
        });
        pages.push(page);

    }
    pages
}


pub async fn get_vc_people(ctx: &Context, msg: &Message) -> Result<Vec<String>, String> {
    let mut people_vc: Vec<String> = Vec::new();
    let guild = msg.guild(&ctx.cache).await.expect("something");
    match guild.voice_states.get(&msg.author.id) {
        Some(s) => {
            let vc_id = s.channel_id.unwrap();
            let guild = msg.guild_id.unwrap().channels(&ctx.http).await.unwrap();
            let guild_channel = guild.get(&vc_id).unwrap();
            let ids = guild_channel.members(&ctx.cache).await.unwrap();
            for i in ids {
                if !i.user.bot {
                    people_vc.push(i.user.id.0.to_string());
                }
            }
        }
        _ => {
            return Err("Erro! Não há pessoas no voice channel".to_string());
        }
    };
    Ok(people_vc)
}



pub async fn create_review_poll(ctx: &Context, msg: &Message, movie: &Movie, names: &HashMap<String, String>) {
    let emoji: Vec<ReactionType> = vec![ReactionType::Unicode("1️⃣".to_string()), ReactionType::Unicode("2️⃣".to_string()),
                                        ReactionType::Unicode("3️⃣".to_string()), ReactionType::Unicode("4️⃣".to_string()),
                                        ReactionType::Unicode("5️⃣".to_string()), ReactionType::Unicode("6️⃣".to_string()),
                                        ReactionType::Unicode("7️⃣".to_string()), ReactionType::Unicode("8️⃣".to_string()),                                            ReactionType::Unicode("1️⃣".to_string()), ReactionType::Unicode("2️⃣".to_string()),
                                        ReactionType::Unicode("9️⃣".to_string()), ReactionType::Unicode("🔟".to_string()),
                                        ];
    let id = &movie.imdb_id;
    println!("{}", id);
    let info = movie_with_id(id.to_owned()).await.unwrap();
    msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&movie.title);
                e.description(&movie.link_imdb);
                e.field("People", movie.get_people(names), true);
                e.image(info.poster);
                e.description(info.plot);

                e
            });
            m.reactions(emoji)
    }).await.unwrap();   
}
