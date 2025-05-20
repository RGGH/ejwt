mod auth;
mod handlers;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || App::new().configure(handlers::configure_app))
    .bind("127.0.0.1:2424")
    .expect("Address should be free and valid")
    .run()
    .await
}

