//! Submodule providing websocket services post-authentication.
use actix_web::{Error, HttpRequest, HttpResponse, web};

mod portal;
use portal::portal_ws;
mod listen_notify;
use actix_web_codegen::get;
pub(crate) use listen_notify::LNCommand;
pub use listen_notify::{ListenNotifyHandle, ListenNotifyServer};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(start_websocket);
}

#[get("/ws")]
/// Entrypoint to start the websocket
///
/// # Arguments
/// * `user` - The logged in user that is starting the websocket, as derived
///   from the JWT token middleware
/// * `req` - The HTTP request
/// * `stream` - The websocket stream
/// * `diesel_pool` - The Diesel connection pool
async fn start_websocket(
    req: HttpRequest,
    stream: web::Payload,
    redis_client: web::Data<redis::Client>,
    diesel_pool: web::Data<crate::DBPool>,
    listen_notify_handle: web::Data<ListenNotifyHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned
    // immediately
    tokio::task::spawn_local(portal_ws(
        session,
        diesel_pool,
        redis_client,
        (**listen_notify_handle).clone(),
        stream,
    ));

    // respond immediately with response connected to WS session
    Ok(res)
}
