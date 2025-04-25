extern crate diesel;

use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{
    App, HttpRequest, HttpServer, Responder, get, http::header, middleware::Logger, web,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use redis::Client;

/// Entrypoint to load the index.html file
async fn index() -> impl Responder {
    NamedFile::open("/home/appuser/app/web/frontend/dist/index.html")
}

#[get("/{filename:.*}")]
/// Entrypoint to load the *.wasm and the *.js files
///
/// # Implementation details
/// If the path happens to not exist, the server will return a 404 error.
async fn frontend_static_files(req: HttpRequest) -> impl Responder {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    match NamedFile::open(format!("/home/appuser/app/web/frontend/dist/{}", path.display())) {
        Ok(file) => file,
        Err(_) => NamedFile::open("/home/appuser/app/web/frontend/dist/index.html").unwrap(),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url =
        std::env::var("DOCKER_DATABASE_URL").expect("DOCKER_DATABASE_URL must be set");

    // create db connection pool
    let pool: backend::DBPool = diesel_async::pooled_connection::bb8::Pool::builder()
        .build(diesel_async::pooled_connection::AsyncDieselConnectionManager::<
            diesel_async::AsyncPgConnection,
        >::new(database_url))
        .await
        .expect("Error creating database pool");

    let redis_client =
        Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set").as_str())
            .expect("Error creating redis client");

    let domain = std::env::var("DOMAIN").expect("DOMAIN is not available.");

    // load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            format!("/home/appuser/app/web/nginx/{domain}-key.pem"),
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file(format!("/home/appuser/app/web/nginx/{domain}.pem"))
        .unwrap();

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&format!("https://{domain}"))
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .supports_credentials();
        App::new()
            // pass in the database pool to all routes
            .app_data(web::Data::new(pool.clone()))
            // pass in the redis client to all routes
            .app_data(web::Data::new(redis_client.clone()))
            // We register the API handlers
            .configure(backend::api::configure)
            // We serve the frontend as static files
            .route("/", web::get().to(index))
            .route("/login", web::get().to(index))
            .route("/profile", web::get().to(index))
            .service(frontend_static_files)
            // enable logger
            .wrap(Logger::default())
            // We add support for CORS requests
            .wrap(cors)
        // limit the maximum amount of data that server will accept
        // .app_data(web::JsonConfig::default().limit(10*1024*1024))
    })
    .bind_openssl(
        format!("0.0.0.0:{}", std::env::var("ACTIX_PORT").expect("ACTIX_PORT is not available.")),
        builder,
    )?
    .workers(4)
    .run()
    .await
}
