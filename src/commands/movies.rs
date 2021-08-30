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
use serenity::{builder::CreateMessage,framework::standard::{
        macros::{command, group},
         CommandResult,
}, model::channel::Message, prelude::*};
use serenity_utils::menu::Menu;


use crate::modules::pagination;
use crate::modules::movies_aux;
use movies_aux::*;
use movies_aux::Movie as Movie;

use crate::modules::function_aux::init_hashmap;


#[group]
#[commands(add, rm, add_person, rm_person, show, choose_vc)]
#[prefixes("movie","mv")]
#[description("movie stuff")]
struct Movies;


#[command]
#[description("Add a movie to the list with either name or IMDB id")]
#[usage="§movie add title"]
#[example="§movie add Joker"]
#[example="§movie add tt7286456 @person1 @person2"]
async fn add (ctx: &Context, msg: &Message) -> CommandResult {

    //dividir a 1a string que supostament é o titulo do filme por "'" (Isto torna obrigatório por o titulo do filme entre ')
    //assim supostamente ficamos com um vetor com a string "§movie add" e com outra string que é o titulo do filme
    let movie = msg.content.replace("§movie add ", "");
    let movie = movie.replace("§mv add ", "");
    let movie: Vec<&str> = movie.split(" <@").collect();
    let movie = movie[0];

    //verifica se o titulo do filme foi escrito entre '
    if movie.len() < 3 {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let info = match movie_with_name(movie.to_string()).await {
        Ok(info) => info,
        Err(_) => match movie_with_id(movie.to_string()).await {
            Ok(info) => info,
            Err(_) => {
                println!("{}", movie);
                msg.channel_id.say(&ctx.http, "Error! Movie not found ('§movie help to see examples)").await?;
                return Ok(());
            }
        }

    };

    let title = &info.title;
    let title = title.to_string();
    let imdb_id = info.imdb_id;
    let mut link_imdb = String::from("https://www.imdb.com/title/");
    link_imdb.push_str(imdb_id.as_str());

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
        //Se não houver link vai ficar como N/A
    }


    let mut movies: Vec<Movie> = json_to_vec_movies(msg);

    let title = match Movie::search_title(&mut movies, title) {
        Ok(filme) => {
            msg.channel_id.say(&ctx.http, format!("Error! Movie already exists: {}", filme.link_imdb )).await?;
            return Ok(());
        }
        Err(title) => title,
    };

    let aux = format!("Movie added susscessfully: {}", &link_imdb);


    //Agora é adicionar um filme à struct
    let m = Movie::create_movie(title, people, imdb_id);
    movies.push(m);

    //Finalmente falta passar tudo para o ficheiro outra vez (com o novo filme adicionado)
    vec_movie_to_json(movies, msg);

    //println!("FILES: {:?}", movies);
    println!("{:?}", movie);
    msg.channel_id.say(&ctx.http, aux).await?;
    Ok(())
}

#[command]
#[description("Remove a movie to the list")]
#[usage="§movie rm"]
#[example="§movie remove Joker"]
async fn rm (ctx: &Context, msg: &Message) -> CommandResult {
    let movie = msg.content.replace("§movie rm ", "");
    let movie = movie.replace("§mv rm ", "");
    let movie = movie.trim();
    println!("{}", movie);


    //verifica se o titulo do filme foi escrito entre '
    if movie.len() < 3 {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let mut movies = json_to_vec_movies(msg);
    if movies.len() == 0 {
        msg.channel_id.say(&ctx.http, "There are no movies to remove").await?;
        return Ok(());
    }
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&movie.to_uppercase()) {
            movies.remove(index);
            vec_movie_to_json(movies, msg);
            msg.channel_id.say(&ctx.http, "Movie removed successfully!").await?;
            return Ok(());
        }
    } 
    msg.channel_id.say(&ctx.http, "Error! Movie not found").await?;
    Ok(())
}

#[command]
#[description("Add a person to a movie")]
#[usage="§movie add_person title @person"]
#[example="§movie add_person Joker @person"]
async fn add_person (ctx: &Context, msg: &Message) -> CommandResult {
    let movie = msg.content.replace("§movie add_person ", "");
    let movie = movie.replace("§mv add_person ", "");
    let movie: Vec<&str> = movie.split(" <@").collect();
    let movie = movie[0].trim();

    //verifica se o titulo do filme foi escrito entre '
    if movie.len() < 3 {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Error! You can only add 1 person").await?;
        return Ok(());
    }

    let mut movies = json_to_vec_movies(msg);
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&movie.to_uppercase()) {
            for i in &m.people {
                if i.eq(&person[0].id.to_string()) {
                    msg.channel_id.say(&ctx.http, "Error! Person already in the movie").await?;
                    return Ok(());
                }
            } 
            movies[index].people.push(person[0].id.to_string());
            vec_movie_to_json(movies, msg);
            msg.channel_id.say(&ctx.http, "Person added successfully!").await?;
            return Ok(());
        }
    }
    msg.channel_id.say(&ctx.http, "Error! Movie not found").await?;
    Ok(())
}

#[command]
#[description("Remove a person from a movie")]
#[usage="§movie remove title @person"]
#[example="§movie remove Joker @person"]
async fn rm_person (ctx: &Context, msg: &Message) -> CommandResult {
    let movie = msg.content.replace("§movie rm_person ", "");
    let movie = movie.replace("§mv rm_person ", "");
    let movie: Vec<&str> = movie.split(" <@").collect();
    let movie = movie[0].trim();
    println!("{}", movie);

    //verifica se o titulo do filme foi escrito entre '
    if movie.len() < 3 {
        msg.channel_id.say(&ctx.http, "Error! Make sure you put the movie name correctly ('§movie help to see examples)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Error! You can only remove 1 person").await?;
        return Ok(());
    }

    let mut movies = json_to_vec_movies(msg);
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&movie.to_uppercase()) {
            for (index2, i) in m.people.iter().enumerate() {
                if i.eq(&person[0].id.to_string()) {
                    if index2 == 0 || m.people.len() == 1 {
                        msg.channel_id.say(&ctx.http, "Erro! Não se pode remover a pessoa que adicionou o filme").await?;
                        return Ok(());
                    }
                    movies[index].people.remove(index2);
                    vec_movie_to_json(movies, msg);
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
    let mut movies = json_to_vec_movies(msg);
    let mut names = init_hashmap(msg, ctx).await;
    let title = msg.content.replace("§movie show", "");
    let title = title.replace("§mv show", "");
    println!("{}", title);
    if !title.eq("") {
        let title = title.trim().to_string();
        let movie = match Movie::search_title(&mut movies, title) {
            Ok(movie) => movie,
            Err(title) => {
                msg.channel_id.say(&ctx.http, format!("Error, movie not found: {}", title)).await?;
                return Ok(());
            }
        };
        let mut people = String::new();
        for person in &movie.people {
            let name = names.get(person).unwrap();
            let string = format!("{}\n", name);
            people.push_str(&string);
        }
    
        msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(&movie.title);
                    e.description(&movie.link_imdb);
                    e.field("People", people, true);

                    e
                });
                m
        }).await.unwrap();

        return Ok(());
        

    }


    let guild = msg.guild_id.unwrap();//.members(&ctx.http, None, None).await.expect("Falha aqui não sei pq");
    let members = guild.members(&ctx.http, Some(100), None).await?;
    for member in members {
        names.insert(member.user.id.to_string(), member.user.name.to_string());
    }

    let mut pages = Vec::new();

    let mut all_titles = String::new();

    for movie in movies {
        all_titles.push_str(&movie.title);
        all_titles = all_titles + "\n";

        let mut persons = String::new();
        for person in &movie.people {
            let name = names.get(person).unwrap();
            let string = format!("{}\n", name);
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
    let names = init_hashmap(msg, ctx).await;
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
    let movies = json_to_vec_movies(msg);

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

    if ok_movies.is_empty() {
        msg.channel_id.say(&ctx.http, "Error! There are no movies").await?;
        return Ok(());
    }


    //Ordenar os filmes por ordem decrescente do número de pessoas
    ok_movies.sort_by(|a, b| b.cmp(a));

    //Agora tenho um Vec<Movie> é só fazer show deles (teoricamente let mut pages = Vec::new();

    let mut all_titles = String::new();
    let mut pages = Vec::new();
    for movie in ok_movies {
        all_titles.push_str(&movie.title);
        all_titles = all_titles + "\n";
        let mut persons = String::new();
        for person in &movie.people {
            let name = names.get(person).unwrap();
            let string = format!("{}\n", name);
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
