//! Submodule providing websocket services post-authentication.
use crate::DBPool;
use actix_web::web;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use sqlx::{Pool as SQLxPool, Postgres};
pub mod socket;
use crate::api::oauth::refresh::refresh_access_token;
use actix_web_actors::ws::WsResponseBuilder;

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
    let mut frame_size = None;

    let websocket = match refresh_access_token(&req, &diesel_pool, &redis_client).await {
        Ok((user, access_code)) => {
            log::info!("Starting authenticated websocket for user {}", user.id);
            frame_size = Some(5 * 1024 * 1024);
            socket::WebSocket::authenticated(
                diesel_pool.get_ref().clone(),
                sqlx_pool.get_ref().clone(),
                redis_client.get_ref().clone(),
                user,
                access_code,
            )
        }
        Err(_) => {
            log::info!("Starting unauthenticated websocket");
            socket::WebSocket::new(
                diesel_pool.get_ref().clone(),
                sqlx_pool.get_ref().clone(),
                redis_client.get_ref().clone(),
            )
        }
    };

    let mut builder =
        WsResponseBuilder::new(websocket, &req, stream).codec(actix_http::ws::Codec::new());

    if let Some(frame_size) = frame_size {
        builder = builder.frame_size(frame_size);
    }

    builder.start()
}
