//! Submodule providing websocket services post-authentication.
use actix_web::{Error, HttpRequest, HttpResponse, get, rt, web};
use actix_ws::AggregatedMessage;
use futures::StreamExt;

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
    _redis_client: web::Data<redis::Client>,
    _diesel_pool: web::Data<crate::DBPool>,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }
                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }

        let _ = session.close(None).await;
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
