pub mod user;
pub mod models;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

pub fn run() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(user::greet))
            .route("/v1/api/signin", web::get().to(user::signin))
    })
        .bind("127.0.0.1:8080")
        .expect("Can't not bind to port 8080")
        .run()
        .unwrap()
}