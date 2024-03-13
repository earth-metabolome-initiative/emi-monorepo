//! Submodule providing websocket services post-authentication.
use actix_web::web;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use diesel::r2d2::{self, ConnectionManager, Pool as DieselPool};
use sqlx::{postgres::PgPoolOptions, Pool as SQLxPool, Postgres};
use crate::models::User as DBUser;
use crate::DBPool;
pub mod socket;
pub mod channels;

pub fn configure(cfg: &mut web::ServiceConfig) {
    //All these endpoints will show up under `/api/oauth/*`
    cfg.service(
        web::scope(web_common::api::auth::ws::ENDPOINT)
            .service(start_websocket)
        );
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
    user: DBUser,
    req: HttpRequest,
    stream: web::Payload,
    diesel_pool: web::Data<DBPool>,
    sqlx_pool: web::Data<SQLxPool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let diesel_conn = match diesel_pool.get() {
        Ok(diesel_conn) => diesel_conn,
        Err(e) => {
            log::error!("🔥 Error connecting to the database: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let sqlx_pool = sqlx_pool.get_ref().clone();

    actix_web_actors::ws::start(socket::WebSocket::new(user, diesel_conn, sqlx_pool), &req, stream)
}