// main.rs
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenvy::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // enable logger
            .wrap(middleware::Logger::default())
            // limit the maximum amount of data that server will accept
            .app_data(web::JsonConfig::default().limit(4096))
            // everything under '/api/' route
            // .service(
            //     web::scope("/api")
            //         .service(
            //             web::resource("/invitation")
            //                 .route(web::post().to(invitation_handler::post_invitation)),
            //         )
            //         // .service(
            //         //     web::resource("/register/{invitation_id}")
            //         //         .route(web::post().to(register_handler::register_user)),
            //         // )
            //         // .service(
            //         //     web::resource("/auth")
            //         //         .route(web::post().to(auth_handler::login))
            //         //         .route(web::delete().to(auth_handler::logout))
            //         //         .route(web::get().to(auth_handler::get_me)),
            //         // ),
            // )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}