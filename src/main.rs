#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::io;

mod models;
mod schema;
mod helpers;
mod db;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server...");
    HttpServer::new(|| {
        App::new()
            .configure(models::init_routes)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}