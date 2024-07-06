// Example Rust backend using Actix-Web framework
use actix_web::{web, App, HttpServer, Responder};

mod utils;

async fn index() -> impl Responder {
    // Example handler
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start Actix-Web server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
