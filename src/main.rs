#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::{io,env};

mod models;
mod schema;
mod helpers;
mod db;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    info!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .configure(models::init_routes)
    })
    .bind(format!("{}:{}", host, port))?  
    .run()
    .await
}