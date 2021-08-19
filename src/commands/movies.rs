//NOTAS 
//--- Para já para adicionar um filme é preciso meter o nome do filme entre "''" (obrigatório), as pessoas que a pessoa quer que vejam
//(opcional, e a pessoa que escreveu a mensagem fica sempre adicionada) e o link do imdb (opcional, se não meter link fica N/A)

//--- Por enquanto está implementado funções para adicionar e remover filmes e para adicionar e remover pessoas (mas precisam de ser mais testadas)

//--- O ficheiro é organizado da seguinte forma: titulo_do_filme;id1,id2,id3,...;link_imdb (o id1 é sempre da pessoa que fez add do filme)

//--- A struct para já só tem 3 campos, depois pode ser mudado
//--- Uma ideia pode ser a partir do link do imdb arranjar o título, rating, tempo do filme, etc. (Isso teria de ser pesquisado)
//--- O código está bastante mau porque ainda não sei usar rust direito, mas depois pode ser mudado 


//A fazer:
//--- Trocar o link para vazio quando não existe link de imdb
//--- Mudar os files para a diretoria nova com os ficheiros com o id do server como nome
//--- Trocar o id das pessoas para o seu nick na função do show
//--- Fazer uma função que vê as pessoas num voice channel e vê que filmes podem ser vistos com essas pessoas (§movie choose talvez?)
//--- Fazer uma função como a de cima mas com ping às pessoas em vez de ver o voice channel

extern crate serenity;

use std::{cmp::Ordering, fs::File, io::{BufRead, BufReader, Read, Write}, path::Path};
use crate::modules::pagination;

use serenity::{builder::CreateMessage, cache::Cache, framework::standard::{
        macros::{command, group},
         CommandResult,
    }, http::{CacheHttp, client::Http}, model::{channel::{ChannelType, Message}, id::UserId}, model::guild::Guild, model::id::GuildId, prelude::*};
use serenity_utils::menu::Menu;

#[group]
#[commands(add, remove, add_person, remove_person, show, choose_vc)]
#[prefixes("movie","mv")]
#[description("movie stuff")]

struct Movies;

#[derive(Eq)]
//struct para cada linha do ficheiro (provavelmente vai ter que ser muito alterada)
struct Movie {
    title: String,
    people: Vec<String>,
    link_imdb: String
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


//Passar o ficheiro para um vetor de struct (Está pouco otimizada)
fn file_to_struct(msg: &Message) -> Vec<Movie>{
    
    let mut movies = Vec::new();
    let mut path = String::from("files/movies/");
    path.push_str(msg.guild_id.unwrap().to_string().as_str());
    path.push_str(".csv");
    //Abrir o ficheiro e passar tudo para um BuffReader (é mais rapido do que passar para string)
    let f = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            File::create(path).unwrap();
            return movies;
        }
    };
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    //Agora passar o BuffReader para string
    buf_reader.read_to_string(&mut contents).unwrap();
    if contents.len() == 0 {
        return movies;
    }
    //Dividir o ficheiro em um vetor em que cada elemento é uma linha do ficheiro
    let file: Vec<&str> = contents.split("\n").collect();
    for f in &file {
        if f.len() == 0 {
            return movies;
        }
        let aux: Vec<&str> = f.split(";").collect();
        let p: Vec<&str> = aux[1].split(",").collect();
        let mut aux2 = Vec::new();
        for a in p {
            aux2.push(a.to_string());
        }
        let m = Movie {
            title: aux[0].to_string(),
            people: aux2,
            link_imdb: aux[2].to_string()
        };
        movies.push(m);
    }
    movies
}

fn struct_to_file(movies: Vec<Movie>, msg: &Message) {
    
    let mut path = String::from("files/movies/");
    path.push_str(msg.guild_id.unwrap().to_string().as_str());
    path.push_str(".csv");
    
    let mut file = match File::create(path){
        Ok(file) => file,
        Err(_) => panic!("Problema a abrir o ficheiro!"),
    };

    for i in movies {
        let mut line = String::new();
        line.push_str(i.title.as_str().trim());
        line.push_str(";");
        for (index, j) in i.people.iter().enumerate() {
            line.push_str(j.as_str().trim());
            if index != i.people.len() - 1 {
                line.push_str(",");
            }
        }
        line.push_str(";");
        line.push_str(i.link_imdb.as_str().trim());
        file.write(line.as_bytes()).expect("Erro ao ecrever no ficheiro!");
        file.write("\n".as_bytes()).expect("Erro no \n?");
    }

}


#[command]
#[description("Add a movie to the list")]
#[usage="'name'; persons; imdb's link"]
#[example="'name'"]
#[example="'name'; person1, person2 ; imdb's link"]
async fn add (ctx: &Context, msg: &Message) -> CommandResult {
    //dividir a mensagem de quem quer adicionar um filme por ";" (O divisor pode ser mudado depois)
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() > 3 || parts.len() < 1 {
        msg.channel_id.say(&ctx.http, "Error! Ivalid number of fields '§movie help' for more information").await?;
        return Ok(());
    }

    //dividir a 1a string que supostament é o titulo do filme por "'" (Isto torna obrigatório por o titulo do filme entre ')
    //assim supostamente ficamos com um vetor com a string "§movie add" e com outra string que é o titulo do filme
    let movie: Vec<&str> = parts[0].split("'").collect();

    //verifica se o titulo do filme foi escrito entre '
    if movie.len() != 3 || movie[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let mut link = String::new();
    //Verifica se foram mencionadas pessoas e guarda o id delas num vetor (também guarda a do autor)
    //Para além disso, também guarda o link
    let mut people = Vec::new();
    people.push(msg.author.id.to_string());
    let members = &msg.mentions;
    if members.len() != 0 {
        for i in members {
            if !people.contains(&i.id.to_string()) {
                people.push(i.id.to_string());
            }
        }
        //Se houver pessoas mencionadas e houver link então vão haver 3 campos, logo o link será o terceiro
        if parts.len() == 3 {
            link = parts[2].to_string();
        }
        //Se não houver link vai ficar como N/A
    }

    else {
        //Se não houver pessoas mencionadas então o link do imdb será o 2o campo (Se existir)
        if parts.len() == 2 {
            link = parts[1].to_string();
        }
    }



    let mut movies: Vec<Movie> = file_to_struct(msg);
    for f in &movies {
        if f.title.to_uppercase().eq(&movie[1].to_uppercase()) {
            msg.channel_id.say(&ctx.http, "Error! Movie already exists").await?;
            return Ok(());
        }
    }

    //Agora é adicionar um filme à struct
    let m = Movie{
        title: movie[1].to_string(),
        people,
        link_imdb: link
    };
    movies.push(m);

    //Finalmente falta passar tudo para o ficheiro outra vez (com o novo filme adicionado)
    struct_to_file(movies, msg);

    //println!("FILES: {:?}", movies);
    println!("{:?}", parts);
    println!("{:?}", movie);
    msg.channel_id.say(&ctx.http, "Movie added successfully!").await?;
    Ok(())
}




#[command]
#[description("Remove a movie to the list")]
#[usage="'name'"]
#[example="'Aladin'"]
async fn remove (ctx: &Context, msg: &Message) -> CommandResult {
    let title: Vec<&str> = msg.content.split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }
    let mut movies = file_to_struct(msg);
    if movies.len() == 0 {
        msg.channel_id.say(&ctx.http, "There are no movies to remove").await?;
        return Ok(());
    }
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            movies.remove(index);
            struct_to_file(movies, msg);
            msg.channel_id.say(&ctx.http, "Movie removed successfully!").await?;
            return Ok(());
        }
    } 
    msg.channel_id.say(&ctx.http, "Error! Movie not found").await?;
    Ok(())
}

#[command]
#[description("Add a person to a movie")]
#[usage="'name'; @person"]
#[example="'Aladin'; @23"]
async fn add_person (ctx: &Context, msg: &Message) -> CommandResult {
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() != 2 {
        msg.channel_id.say(&ctx.http, "Error! Invalid number of fields").await?;
        return Ok(());
    }

    let title: Vec<&str> = parts[0].split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Error! You can only add 1 person").await?;
        return Ok(());
    }

    let mut movies = file_to_struct(msg);
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            for i in &m.people {
                if i.eq(&person[0].id.to_string()) {
                    msg.channel_id.say(&ctx.http, "Error! Person already in the movie").await?;
                    return Ok(());
                }
            } 
            movies[index].people.push(person[0].id.to_string());
            struct_to_file(movies, msg);
            msg.channel_id.say(&ctx.http, "Person added successfully!").await?;
            return Ok(());
        }
    }
    msg.channel_id.say(&ctx.http, "Error! Movie not found").await?;
    Ok(())
}

#[command]
#[description("Remove a person from a movie")]
#[usage="'name'; @person"]
#[example="'Aladin'; @23"]
async fn remove_person (ctx: &Context, msg: &Message) -> CommandResult {
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() != 2 {
        msg.channel_id.say(&ctx.http, "Error! Invalid number of fields").await?;
        return Ok(());
    }

    let title: Vec<&str> = parts[0].split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Error! You can only remove 1 person").await?;
        return Ok(());
    }

    let mut movies = file_to_struct(msg);
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            for (index2, i) in m.people.iter().enumerate() {
                if i.eq(&person[0].id.to_string()) {
                    if index2 == 0 || m.people.len() == 1 {
                        msg.channel_id.say(&ctx.http, "Erro! Não se pode remover a pessoa que adicionou o filme").await?;
                        return Ok(());
                    }
                    movies[index].people.remove(index2);
                    struct_to_file(movies, msg);
                    msg.channel_id.say(&ctx.http, "Pessoa removida com sucesso").await?;
                    return Ok(());
                }
            } 
            msg.channel_id.say(&ctx.http, "Erro! Essa pessoa não está no filme").await?;
            return Ok(());
        }
    }
    msg.channel_id.say(&ctx.http, "Erro! O filme não foi encontrado").await?;
    Ok(())
}


#[command]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
    let movies = file_to_struct(msg);
    let mut pages = Vec::new();

    let mut all_titles = String::new();

    for movie in movies {
        all_titles.push_str(&movie.title);
        all_titles = all_titles + "\n";

        let mut persons = String::new();
        for person in &movie.people {
            //Passar o id para um int
            let person: u64 = person.parse().unwrap();
            println!("{}", person);
            //Passar agora para um membro (struct do serenety)
            let person = msg.guild_id.unwrap().member(&ctx.http, person).await?;
            println!("{:?}",person);
            let string = format!("{}\n", person.user.name);
            persons.push_str(&string);
        }

        let mut page = CreateMessage::default();
        page.content("MOVIES").embed(|e| {
            e.title(&movie.title);
            if !movie.link_imdb.eq(""){
                e.description(&movie.link_imdb);
            }
            e.field("People:",&persons,true);
         e
        });
        pages.push(page);
    }

    let msg1 = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field("All movies", all_titles, false);

                e
            });
            m
    });

    msg1.await.unwrap();
    // Creates a new menu.
    let menu = Menu::new(ctx, msg, &pages, pagination::simple_options());
    // Runs the menu and returns optional `Message` used to display the menu.
    let _ = menu.run().await?;

    Ok(())
}


/* Estava a tentar fazer uma função separada para passar o id para nick, mas n dá para ser async não sei pq (ela precisa de ser async por causa do await())
async fn id_to_nick(ctx: &Context, msg: &Message, person: &String) -> String{
    let g = msg.guild_id.unwrap();
    let person: u64 = person.parse().unwrap();
    let member = g.member(ctx, person).await.unwrap();
    member.nick.unwrap()
}
*/


#[command]
#[description("Shows a list of movies that can be seen according to the people in the voice channel")]
async fn choose_vc(ctx: &Context, msg: &Message) -> CommandResult {
    //ir ao voice channel buscar os ids
    let mut people_vc: Vec<String> = Vec::new();
    let guild = msg.guild(&ctx.cache).await.expect("something");
    match guild.voice_states.get(&msg.author.id) {
        Some(s) => {
            let vc_id = s.channel_id.unwrap();
            let guild = msg.guild_id.unwrap().channels(&ctx.http).await?;
            let guild_channel = guild.get(&vc_id).unwrap();
            let ids = guild_channel.members(&ctx.cache).await?;
            for i in ids {
                if !i.user.bot {
                    people_vc.push(i.user.id.0.to_string());
                }
            }
        }
        _ => {
            msg.channel_id.say(&ctx.http, "Erro! Não há pessoas em nenhum voice channel").await?;
            return Ok(());
        }
    };
    println!("people_vc: {:?}", people_vc);
    

    //Ir buscar os filmes ao file
    let movies = file_to_struct(msg);

    //Ver que filmes podem ser vistos em função  das pessoas na chamada
    let mut ok_movies: Vec<Movie> = Vec::new();
    for movie_aux in movies {
        let people = &movie_aux.people;
        if people.iter().all(|item| people_vc.contains(item)) {
            if msg.content.contains("exact") {
                if people.len() == people_vc.len() {
                    ok_movies.push(movie_aux);
                }
            } else {
                ok_movies.push(movie_aux);
            }
        }
    }

    //Ordenar os filmes por ordem decrescente do número de pessoas
    ok_movies.sort_by(|a, b| b.cmp(a));

    //Agora tenho um Vec<Movie> é só fazer show deles (teoricamente let mut pages = Vec::new();


    let mut pages = Vec::new();
    for movie in ok_movies {
        let mut persons = String::new();
        for person in &movie.people {
            //Passar o id para um int
            let person: u64 = person.parse().unwrap();
            println!("{}", person);
            //Passar agora para um membro (struct do serenety)
            let person = msg.guild_id.unwrap().member(&ctx.http, person).await?;
            println!("{:?}",person);
            let string = format!("{}\n", person.user.name);
            persons.push_str(&string);
        }

        let mut page = CreateMessage::default();
        page.content("MOVIES").embed(|e| {
            e.title(&movie.title);
            if !movie.link_imdb.eq(""){
                e.description(&movie.link_imdb);
            }
            e.field("People:",&persons,true);
            e
        });
        pages.push(page);
    }

    // Creates a new menu.
    let menu = Menu::new(ctx, msg, &pages, pagination::simple_options());
    // Runs the menu and returns optional `Message` used to display the menu.
    let _ = menu.run().await?;
    Ok(())
}
