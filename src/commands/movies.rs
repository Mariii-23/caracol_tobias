//NOTAS 
//--- Para já para adicionar um filme é preciso meter o nome do filme entre "''" (obrigatório), as pessoas que a pessoa quer que vejam
//(opcional, e a pessoa que escreveu a mensagem fica sempre adicionada) e o link do imdb (opcional, se não meter link fica N/A)

//--- Por enquanto está implementado funções para adicionar e remover filmes e para adicionar e remover pessoas (mas precisam de ser mais testadas)

//--- O ficheiro é organizado da seguinte forma: titulo_do_filme;id1,id2,id3,...;link_imdb (o id1 é sempre da pessoa que fez add do filme)

//--- A struct para já só tem 3 campos, depois pode ser mudado
//--- Uma ideia pode ser a partir do link do imdb arranjar o título, rating, tempo do filme, etc. (Isso teria de ser pesquisado)
//--- O código está bastante mau porque ainda não sei usar rust direito, mas depois pode ser mudado 


extern crate serenity;

use std::{fs::File, io::{BufRead, BufReader, Read, Write}, path::Path};
use crate::modules::pagination;

use serenity::{
    builder::CreateMessage,
    http::AttachmentType,
    framework::standard::{
        macros::{command, group},
         CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use serenity_utils::menu::Menu;

#[group]
#[commands(add, remove, add_person, remove_person)]
#[prefixes("movie")]
#[description("movie stuff")]

struct Movies;

//struct para cada linha do ficheiro (provavelmente vai ter que ser muito alterada)
struct Movie {
    title: String,
    people: Vec<String>,
    link_imdb: String
}

#[command]
async fn add (ctx: &Context, msg: &Message) -> CommandResult {
    //dividir a mensagem de quem quer adicionar um filme por ";" (O divisor pode ser mudado depois)
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() > 3 || parts.len() < 1 {
        msg.channel_id.say(&ctx.http, "Número de campos inválido! Faz '§movie help' para mais informações").await?;
        return Ok(());
    }

    //dividir a 1a string que supostament é o titulo do filme por "'" (Isto torna obrigatório por o titulo do filme entre ')
    //assim supostamente ficamos com um vetor com a string "§movie add" e com outra string que é o titulo do filme
    let movie: Vec<&str> = parts[0].split("'").collect();

    //verifica se o titulo do filme foi escrito entre '
    if movie.len() != 3 || movie[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Erro! Certifica-te que colocaste bem o filme ('§movie help para mais ajuda)").await?;
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
        else {
            link = String::from("N/A");
        }
    }

    else {
        //Se não houver pessoas mencionadas então o link do imdb será o 2o campo (Se existir)
        if parts.len() == 2 {
            link = parts[1].to_string();
        }
        else {
            link = String::from("N/A");
        }
    }



    let mut movies: Vec<Movie> = file_to_struct();
    for f in &movies {
        if f.title.to_uppercase().eq(&movie[1].to_uppercase()) {
            msg.channel_id.say(&ctx.http, "Esse filme já existe na lista").await?;
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
    struct_to_file(movies);

    //println!("FILES: {:?}", movies);
    println!("{:?}", parts);
    println!("{:?}", movie);
    msg.channel_id.say(&ctx.http, "Filme adicionado com sucesso!").await?;
    Ok(())
}

//Passar o ficheiro para um vetor de struct (Está pouco otimizada)
fn file_to_struct() -> Vec<Movie>{
    //Abrir o ficheiro e passar tudo para um BuffReader (é mais rapido do que passar para string)
    let f = File::open("files/movies.csv").expect("Erro ao abrir o ficheiro");
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    let mut movies = Vec::new();
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

fn struct_to_file(movies: Vec<Movie>) {
    let mut file = match File::create("files/movies.csv"){
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
async fn remove (ctx: &Context, msg: &Message) -> CommandResult {
    let title: Vec<&str> = msg.content.split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Erro! Certifica-te que colocaste bem o filme ('§movie help para mais ajuda)").await?;
        return Ok(());
    }
    let mut movies = file_to_struct();
    if movies.len() == 0 {
        msg.channel_id.say(&ctx.http, "Não há filmes para remover").await?;
        return Ok(());
    }
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            movies.remove(index);
            struct_to_file(movies);
            msg.channel_id.say(&ctx.http, "Filme removido com sucesso!").await?;
            return Ok(());
        }
    } 
    msg.channel_id.say(&ctx.http, "Esse filme não foi encontrado").await?;
    Ok(())
}

#[command]
async fn add_person (ctx: &Context, msg: &Message) -> CommandResult {
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() != 2 {
        msg.channel_id.say(&ctx.http, "Erro! Número de campos inválido").await?;
        return Ok(());
    }

    let title: Vec<&str> = parts[0].split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Erro! Certifica-te que colocaste bem o filme ('§movie help para mais ajuda)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Erro! Só dá para adicionar 1 pessoa").await?;
        return Ok(());
    }

    let mut movies = file_to_struct();
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            for i in &m.people {
                if i.eq(&person[0].id.to_string()) {
                    msg.channel_id.say(&ctx.http, "Erro! Essa pessoa já está adicionada ao filme").await?;
                    return Ok(());
                }
            } 
            movies[index].people.push(person[0].id.to_string());
            struct_to_file(movies);
            msg.channel_id.say(&ctx.http, "Pessoa adicionada com sucesso").await?;
            return Ok(());
        }
    }
    msg.channel_id.say(&ctx.http, "Erro! O filme não foi encontrado").await?;
    Ok(())
}

#[command]
async fn remove_person (ctx: &Context, msg: &Message) -> CommandResult {
    let parts: Vec<&str> = msg.content.split(";").collect();
    if parts.len() != 2 {
        msg.channel_id.say(&ctx.http, "Erro! Número de campos inválido").await?;
        return Ok(());
    }

    let title: Vec<&str> = parts[0].split("'").collect();
    if title.len() != 3 || title[1].is_empty() {
        msg.channel_id.say(&ctx.http, "Erro! Certifica-te que colocaste bem o filme ('§movie help para mais ajuda)").await?;
        return Ok(());
    }

    let person = &msg.mentions;
    if person.len() != 1 {
        msg.channel_id.say(&ctx.http, "Erro! Só dá para remover 1 pessoa").await?;
        return Ok(());
    }

    let mut movies = file_to_struct();
    for (index, m) in movies.iter().enumerate() {
        if m.title.to_uppercase().eq(&title[1].to_uppercase()) {
            for (index2, i) in m.people.iter().enumerate() {
                if i.eq(&person[0].id.to_string()) {
                    if index2 == 0 || m.people.len() == 1 {
                        msg.channel_id.say(&ctx.http, "Erro! Não se pode remover a pessoa que adicionou o filme").await?;
                        return Ok(());
                    }
                    movies[index].people.remove(index2);
                    struct_to_file(movies);
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