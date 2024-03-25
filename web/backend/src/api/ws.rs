//! Submodule providing websocket services post-authentication.
use crate::DBPool;
use actix_web::web;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use sqlx::{Pool as SQLxPool, Postgres};
pub mod channels;
pub mod socket;
use actix_web_actors::ws::WsResponseBuilder;
use crate::api::oauth::refresh::refresh_access_token;
pub mod users;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(start_websocket);
}

pub fn configure_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(start_auth_websocket);
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

    WsResponseBuilder::new(
        socket::WebSocket::new(diesel_pool, sqlx_pool, redis_client),
        &req,
        stream,
    )
    .codec(actix_http::ws::Codec::new())
    // NOTE THAT HERE WE ARE NOT INCREASING THE FRAME SIZE!
    // AN UNAUTHENTICATED USER SHOULD NOT BE ABLE TO SEND LARGE MESSAGES
    .start()
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
async fn start_auth_websocket(
    req: HttpRequest,
    stream: web::Payload,
    redis_client: web::Data<redis::Client>,
    diesel_pool: web::Data<DBPool>,
    sqlx_pool: web::Data<SQLxPool<Postgres>>,
) -> Result<HttpResponse, Error> {
    log::info!("Starting websocket");

    let (user, access_code) = match refresh_access_token(&req, &diesel_pool, &redis_client).await {
        Ok(token) => token,
        Err(error) => {
            return Ok(error);
        }
    };

    let diesel_pool = diesel_pool.get_ref().clone();
    let sqlx_pool = sqlx_pool.get_ref().clone();
    let redis_client = redis_client.get_ref().clone();

    let websocket = match socket::WebSocket::authenticated(
        diesel_pool,
        sqlx_pool,
        redis_client,
        user,
        access_code
    ) {
        Ok(websocket) => websocket,
        Err(error) => {
            return Ok(error.into());
        }
    };

    WsResponseBuilder::new(
        websocket,
        &req,
        stream,
    )
    .codec(actix_http::ws::Codec::new())
    // This will overwrite the codec's max frame-size
    .frame_size(1024 * 1024 * 5) // 5MB
    .start()
}
