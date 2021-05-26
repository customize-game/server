use actix_web::{ App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;
mod controller;
mod utils;
mod server;

fn main() -> Result<(), actix_web::Error> {
    server::server::run()
}
