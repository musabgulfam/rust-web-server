use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fs;
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

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

struct Note {
    title: String,
    note: String,
    id: u64
}

struct State {
    notes: Vec<Note>,
    updated_id: u64
}

impl State {
    fn create() -> State {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("state.txt");

        for entry in fs::read_dir("./").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            println!("{:?}", path);
            if path.ends_with(".txt") {
                let file_id_str = path.to_str().trim_end_matches(".txt");
                match file_id_str.parse::<u64>() {
                    Ok(id) => {
                        let mut file_content = String::new();
                        let mut file = fs::File::open(entry.path());
                        if let Ok(file){
                            file.read_to_string(&mut file_content);
                        }

                        // let note = Note {
                        //     id,
                        //     notes: file_content,
                        // };
                        // notes.push(note);
                    }
                    Err(_) => { /* Handle invalid ID */ }
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
                    // @TODO: fetch all notes
                    State {
                        updated_id: id,
                        notes: vec![]
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
    let mut file = match File::open("state.txt"){
        Ok(mut file) => {
            // state.updated_id += 1;
            let file_name = format!("{}.txt", state.updated_id);
            let mut file = match File::create(&file_name) {
                Ok(file) => file,
                Err(err) => return Err(err)
            };
        
            match file.write_all(format!("{}\n{}", title, note).as_bytes()){
                Ok(_) => {
                    // Create note part
                    let new_note = Note{
                        title: title.to_string(), 
                        note: note.to_string(), 
                        id: state.updated_id
                    };
                    state.notes.push(new_note);
                    state.updated_id += 1;
                    println!("Updated ID: {}", state.updated_id);
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
        },
        Err(err) => return Err(err)
    };
    file
}

fn main() {
    let state = State::create();
    let title = String::from("Title");
    let note = String::from("Hello, world!1");
    create_note(
        title.to_string(),
        note.to_string(),
        state
    );
}