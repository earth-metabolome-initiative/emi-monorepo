// main.rs
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use redis::Client;

mod api;
mod model_implementations;
mod models;
mod schema;
mod diesel_enums;
mod transactions;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool<ConnectionManager<PgConnection>> = r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.");

    let redis_client = match Client::open(
        std::env::var("REDIS_URL").expect("REDIS_URL must be set").as_str(),
    ) {
        Ok(client) => {
            println!("✅Connection to the redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    // let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            // pass in the database pool to all routes
            .app_data(web::Data::new(pool.clone()))
            // pass in the redis client to all routes
            .app_data(web::Data::new(redis_client.clone()))
            // We register the API handlers
            .configure(api::configure)
            // enable logger
            .wrap(Logger::default())
            // We add support for CORS requests
            .wrap(cors)
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
