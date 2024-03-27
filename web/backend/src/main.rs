// main.rs
extern crate diesel;

use actix_web::{get, web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::HttpRequest;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use redis::Client;
use sqlx::{postgres::PgPoolOptions, Pool as SQLxPool, Postgres};
use std::path::PathBuf;

mod api;
mod diesel_enums;
mod model_implementations;
mod views;
mod models;
mod schema;
mod transactions;

pub(crate) type DBPool = Pool<ConnectionManager<PgConnection>>;
pub(crate) type DieselConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Entrypoint to load the index.html file
async fn index() -> impl Responder {
    NamedFile::open("/app/frontend/dist/index.html")
}

#[get("/{filename:.*}")]
/// Entrypoint to load the *.wasm and the *.js files
///
/// # Implementative details
/// If the path happens to not exist, the server will return a 404 error.
async fn static_files(req: HttpRequest) -> impl Responder {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    NamedFile::open(format!("/app/frontend/dist/{}", path.display()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: DBPool = match r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager)
    {
        Ok(client) => {
            log::info!("✅ Diesel connection to the database is successful!");
            client
        }
        Err(e) => {
            log::error!("🔥 Error connecting to the database with Diesel: {}", e);
            std::process::exit(1);
        }
    };

    let redis_client = match Client::open(
        std::env::var("REDIS_URL")
            .expect("REDIS_URL must be set")
            .as_str(),
    ) {
        Ok(client) => {
            log::info!("✅Connection to Redis is successful!");
            client
        }
        Err(e) => {
            log::error!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let sqlx_pool: SQLxPool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ SQLx Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the SQLx Database: {:?}", err);
            std::process::exit(1);
        }
    };

    // We remove the file "backend.building" if it exists
    std::fs::remove_file("/app/backend/backend.building").unwrap_or_default();

    // We write to a "backend.ready" file to indicate that
    // the backend has finished compiling and is now starting.
    let timestamp = chrono::Utc::now().to_rfc2822();
    std::fs::write(
        "/app/backend/backend.ready",
        format!("backend is ready at {}", timestamp),
    )
    .unwrap();

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("/app/nginx/emi.local-key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("/app/nginx/emi.local.pem")
        .unwrap();

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://emi.local")
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
            // pass in the sqlx pool to all routes
            .app_data(web::Data::new(sqlx_pool.clone()))
            // We register the API handlers
            .configure(api::configure)
            // We serve the frontend as static files
            .route("/", web::get().to(index))
            .route("/login", web::get().to(index))
            .route("/profile", web::get().to(index))
            .service(static_files)
            // enable logger
            .wrap(Logger::default())
            // We add support for CORS requests
            .wrap(cors)
        // limit the maximum amount of data that server will accept
        // .app_data(web::JsonConfig::default().limit(10*1024*1024))
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .workers(4)
    .run()
    .await
}
