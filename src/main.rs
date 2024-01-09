use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

#[get("/notes")]
async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[post("/create-note")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    title: String,
    note: String,
    id: u64
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    notes: Vec<Note>,
    updated_id: u64
}

#[get("/")]
async fn index(data: web::Data<State>) -> HttpResponse {
    let notes = &data.notes; // <- get app_name
    println!("Notes: {:?}", notes);
    let serialized = serde_json::to_string(&data.notes).unwrap();
    HttpResponse::Ok().json(serialized)
}

impl State {
    fn create() -> State {
        let mut fetched_notes: Vec<Note> = vec![];
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("state.txt");

        for entry in fs::read_dir("./").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    if let Some(extension) = Path::new(path_str).extension().and_then(OsStr::to_str) {
                        if extension == "txt" {
                            if let Some(txt_file_name) = Path::new(path_str).file_stem().unwrap().to_str() {
                                if let Ok(id) = txt_file_name.parse::<u64>() {
                                    println!("ID: {:?}", id);
                                    if let Ok(contents) = fs::read_to_string(path_str) {
                                        let mut parts = contents.split('\n');
                                        if let Some(title) = parts.next() {
                                            if let Some(note) = parts.next() {
                                                let note = Note {
                                                    title: String::from(title),
                                                    note: String::from(note),
                                                    id
                                                };
                                                fetched_notes.push(note);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        match file {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content);
                if content.is_empty() {
                    file.write_all(b"0");
                    State {
                        notes: vec![],
                        updated_id: 0
                    }
                }
                else {
                    let mut id: u64 = 0;
                    println!("Parsed content: {}", content);
                    id = match content.parse::<u64>() {
                        Ok(num) => num,
                        Err(_) => 0
                    };
                    // @TODO: fetch all fetched_notes
                    State {
                        updated_id: id,
                        notes: fetched_notes
                    }
                }
            }
            Err(e) => {
                State {
                    notes: vec![],
                    updated_id: 0
                }
            },
        }
    }
}

fn create_note (title: String, note: String, mut state: State) -> Result<State, std::io::Error> {
    
    // Create the state retrival mechanism here, if exists, else create a new state file
    if let Ok(mut file) = File::open("state.txt") {
        
        // state.updated_id += 1;
        let file_name = format!("{}.txt", state.updated_id);
        let mut file = match File::create(&file_name) {
            Ok(file) => file,
            Err(err) => return Err(err)
        };
    
        match file.write_all(format!("{}\n{}", title,  note).as_bytes()){
            Ok(_) => {
                // Create note part
                let new_note = Note{
                    title: title.to_string(), 
                    note: note.to_string(), 
                    id: state.updated_id
                };
                state.notes.push(new_note);
                state.updated_id += 1;
                // Save the state
                let mut state_file = match File::create("state.txt") {
                    Ok(mut file) => {
                        file.write_all(format!("{}", state.updated_id).as_bytes())
                    },
                    Err(err) => return Err(err)
                };
                Ok(state)
            },
            Err(err) => return Err(err)
        }
    }
    else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "State file not found"))
    }
}

// fn main() {
//     let state = State::create();
//     println!("{:?}", state);
//     let title = String::from("Title");
//     let note = String::from("Hello, world!");
//     create_note(
//         title.to_string(),
//         note.to_string(),
//         state
//     );
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let state = State::create();
        App::new()
            .app_data(web::Data::new(state))
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}