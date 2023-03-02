use std::{fs::File, fs::OpenOptions, io::BufReader, io::Write};

use actix_web::{get, web, HttpResponse, Responder};

use csv::{ReaderBuilder, StringRecord};

#[derive(serde::Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub tel: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

#[get("/addNew")]
async fn add_new() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/add_new.html"))
}

// this function takes a user, writes the user's name, email and telephone to ./database.csv
pub fn write_to_file(user: User) {
    // open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open("./database.csv")
        .expect("Failed to open file");

    // let mut file = File::create("./database.csv").expect("Failed to create file");
    // write the user data to a file
    file.write_all(format!("{},{},{}\n", user.name, user.email, user.tel).as_bytes())
        .unwrap();
}

#[get("/addNew/{name}/{email}/{tel}")]
async fn submit(path: web::Path<(String, String, String)>) -> impl Responder {
    // Extract the tuple from the web::Path struct using into_inner()
    let (name, email, tel) = path.into_inner();

    // create a new user
    let user = User { name, email, tel };

    // call the write_to_file function to write the user data to a file
    write_to_file(user);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/submit.html"))
}

#[get("/search")]
async fn search() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/search.html"))
}

// this function takes a name, searches the ./database.csv file for the name and returns the record in this csv file
fn search_file(name: &str) -> Option<String> {
    let file = File::open("./database.csv").ok()?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    for result in csv_reader.records() {
        let record: StringRecord = result.ok()?;
        if let Some(n) = record.get(0) {
            if n == name {
                let output = format!(
                    "Name: {}, Email: {}, Telephone: {}",
                    record.get(0).unwrap_or(""),
                    record.get(1).unwrap_or(""),
                    record.get(2).unwrap_or("")
                );
                return Some(output);
            }
        }
    }

    None
}

#[get("/search/{name}")]
async fn search_name(path: web::Path<String>) -> impl Responder {
    // Extract the tuple from the web::Path struct using into_inner()
    let name = path.into_inner();

    // define a variable to hold the search result
    let mut search_result = String::new();
    if let Some(result) = search_file(&name) {
        search_result = result;
    } else {
        search_result = "Record not found".to_string();
    }

    HttpResponse::Ok().body(search_result)
}
