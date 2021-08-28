use std::{cmp::Ordering, fs::{File,write}, io::BufReader};
use omdb::*;

use serenity::model::channel::Message;
use crate::constantes::{APIKEY,FILES_PATH,EXTENSION_PATH};

extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Eq, Serialize, Deserialize, Debug)]
//struct para cada linha do ficheiro (provavelmente vai ter que ser muito alterada)
pub struct Movie {
    pub title: String,
    pub people: Vec<String>,
    pub link_imdb: String,
    pub imdb_id: String
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
    let mut path = String::from(FILES_PATH);
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

    let mut path = String::from(FILES_PATH);
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
