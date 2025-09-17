//! Main entrypoint for the web server

use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, Responder, http::header, middleware::Logger, web};
use actix_web_codegen::get;
use backend::ListenNotifyServer;
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url =
        std::env::var("DOCKER_DATABASE_URL").expect("DOCKER_DATABASE_URL must be set");

    // create db connection pool
    let pool: backend::DBPool = diesel::r2d2::Pool::builder()
        .build(diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(database_url))
        .expect("Error creating database pool");

    // Run init migrations.
    {
        let mut connection = pool.get().unwrap();
        let database_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        if !init_db::database_exists(&database_name, &mut connection)
            .expect("Error checking if database exists")
        {
            init_db::init_database(&database_name, true, &mut connection)
                .await
                .expect("Error creating database");
            init_migration::init_migration(&mut connection).expect("Error running init migration");
        }
    }

    let redis_client =
        Client::open(std::env::var("REDIS_URL").expect("REDIS_URL must be set").as_str())
            .expect("Error creating redis client");

    let (listen_notify_server, listen_notify_server_handle) = ListenNotifyServer::new();

    let listen_notify_server = tokio::task::spawn(listen_notify_server.run());

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
    let http_server = HttpServer::new(move || {
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
            // Register the API handlers
            .configure(backend::api::configure)
            // We register the listen-notify server
            .app_data(web::Data::new(listen_notify_server_handle.clone()))
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
    .run();

    tokio::try_join!(http_server, async move { listen_notify_server.await.unwrap() })?;

    Ok(())
}
