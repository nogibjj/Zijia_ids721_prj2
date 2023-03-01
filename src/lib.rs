use std::{fs::File, fs::OpenOptions, io::Write};

use actix_web::{get, post, web, HttpResponse, Responder};

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
