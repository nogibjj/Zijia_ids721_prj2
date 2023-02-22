// web microservice that manager users profiles
use actix_web::{App, HttpServer};
use prj2::{add_new, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(add_new))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
