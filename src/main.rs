use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fs;

async fn index() -> impl Responder {
    match fs::read_to_string("templates/index.html") {
        Ok(contents) => HttpResponse::Ok()
            .content_type("text/html; charset=uft-8")
            .body(contents),
        Err(_) => HttpResponse::InternalServerError().body("Error al cargar el HTML"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
