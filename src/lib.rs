use std::{fs::File, io::Write, path};

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

#[get("/addNew/{name}/{email}/{tel}")]
async fn submit(path: web::Path<(String, String, String)>) -> impl Responder {
    // Extract the tuple from the web::Path struct using into_inner()
    let (name, email, tel) = path.into_inner();

    // create a new user
    let user = User {
        name,
        email,
        tel,
    };

    // open the file ./data.txt to write the user data
    let mut file = File::create("./data.txt").expect("Failed to create file");
    // write the user data to a file
    file.write_all(format!("{} {} {}", user.name, user.email, user.tel).as_bytes())
        .unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/submit.html"))
}
