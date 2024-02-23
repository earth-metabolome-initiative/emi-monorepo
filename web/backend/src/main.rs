// main.rs
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use actix_web::middleware::Logger;

mod api;
mod error_handlers;
mod model_implementations;
mod models;
mod schema;
mod transactions;

use crate::error_handlers::{handle_internal_server_error, handle_unauthorized_request};

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
            // We register the API handlers
            .configure(api::config)
            // enable logger
            .wrap(Logger::default())
            // We add a 401 unauthorized error handler
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        handle_internal_server_error,
                    )
                    .handler(StatusCode::UNAUTHORIZED, handle_unauthorized_request),
            )
            // We add support for CORS requests
            .wrap(cors)
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
        // everything under '/api/' route
        // .service(
        //     web::scope("/api")
        // .service(
        //     web::resource("/invitation")
        //         .route(web::post().to(invitation_handler::post_invitation)),
        // )
        // .service(
        //     web::resource("/register/{invitation_id}")
        //         .route(web::post().to(register_handler::register_user)),
        // )
        // .service(
        //     web::resource("/auth")
        //         .route(web::post().to(auth_handler::login))
        //         .route(web::delete().to(auth_handler::logout))
        //         .route(web::get().to(auth_handler::get_me)),
        // ),
        // )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
