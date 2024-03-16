//! Submodule providing websocket services post-authentication.
use crate::DBPool;
use actix_web::web;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use sqlx::{Pool as SQLxPool, Postgres};
pub mod channels;
pub mod socket;
pub mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(start_websocket);
}

#[get("/ws")]
/// Entrypoint to start the websocket
///
/// # Arguments
/// * `user` - The logged in user that is starting the websocket, as derived from the JWT token middleware
/// * `req` - The HTTP request
/// * `stream` - The websocket stream
/// * `diesel_pool` - The Diesel connection pool
/// * `sqlx_pool` - The SQLx connection pool
async fn start_websocket(
    req: HttpRequest,
    stream: web::Payload,
    redis_client: web::Data<redis::Client>,
    diesel_pool: web::Data<DBPool>,
    sqlx_pool: web::Data<SQLxPool<Postgres>>,
) -> Result<HttpResponse, Error> {

    log::info!("Starting websocket");

    let diesel_pool = diesel_pool.get_ref().clone();
    let sqlx_pool = sqlx_pool.get_ref().clone();
    let redis_client = redis_client.get_ref().clone();

    actix_web_actors::ws::start(
        socket::WebSocket::new(diesel_pool, sqlx_pool, redis_client),
        &req,
        stream,
    )
}
