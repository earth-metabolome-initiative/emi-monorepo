//! Submodule providing websocket services post-authentication.
use actix_web::{Error, HttpRequest, HttpResponse, web};

mod portal;
use portal::portal_ws;
mod listen_notify;
use actix_web_codegen::get;
pub(crate) use listen_notify::LNCommand;
pub use listen_notify::{ListenNotifyHandle, ListenNotifyServer};

use crate::api::oauth::jwt_cookies::MaybeUser;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(start_websocket);
}

#[get("/ws")]
/// Entrypoint to start the websocket
///
/// # Arguments
/// * `request` - The HTTP request
/// * `stream` - The websocket stream
/// * `diesel_pool` - The Diesel connection pool
/// * `listen_notify_handle` - The handle to the ListenNotify server
/// * `user` - The logged in user that is starting the websocket, as derived
///   from the JWT token middleware, if any.
async fn start_websocket(
    request: HttpRequest,
    stream: web::Payload,
    diesel_pool: web::Data<crate::DBPool>,
    listen_notify_handle: web::Data<ListenNotifyHandle>,
    user: MaybeUser,
) -> Result<HttpResponse, Error> {
    let (response, session, stream) = actix_ws::handle(&request, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned
    // immediately
    tokio::task::spawn_local(portal_ws(
        session,
        diesel_pool,
        (**listen_notify_handle).clone(),
        stream,
        user,
    ));

    // respond immediately with response connected to WS session
    Ok(response)
}
