//! Submodule handling websocket services for the portal.

use std::{
    pin::pin,
    time::{Duration, Instant},
};

use actix_web::web;
use actix_ws::AggregatedMessage;
use futures_util::{
    StreamExt as _,
    future::{Either, select},
};
use tokio::{sync::mpsc, time::interval};
use web_common_traits::{
    crud::{AsyncExecuteCrudOperation, CrudOperation},
    database::Tabular,
};
use ws_messages::{B2FMessage, F2BMessage};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping
/// messages, and monitor connection health to detect network issues and free up
/// resources.
pub(super) async fn portal_ws(
    mut session: actix_ws::Session,
    diesel_pool: web::Data<crate::DBPool>,
    redis_client: web::Data<redis::Client>,
    listen_notify_handle: crate::ListenNotifyHandle,
    msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    // let mut name = None;
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);
    let mut conn = diesel_pool.get().await.unwrap();

    let (listen_notify_sender, mut listen_notify_receiver) = mpsc::unbounded_channel();

    // unwrap: chat server is not dropped before the HTTP server
    let session_id = listen_notify_handle.connect(listen_notify_sender).await.unwrap();

    let msg_stream = msg_stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let mut msg_stream = pin!(msg_stream);

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = pin!(interval.tick());
        let listen_notify = pin!(listen_notify_receiver.recv());

        let messages = pin!(select(msg_stream.next(), listen_notify));

        match select(messages, tick).await {
            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    AggregatedMessage::Ping(_) | AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    AggregatedMessage::Text(_) => {
                        log::warn!("unexpected text message");
                    }

                    AggregatedMessage::Binary(bytes) => {
                        match F2BMessage::try_from(bytes) {
                            Ok(F2BMessage::Unsubscribe(unsubscribe)) => {
                                listen_notify_handle
                                    .unsubscribe(unsubscribe, session_id)
                                    .await
                                    .unwrap();
                            }
                            Ok(F2BMessage::Row(ops)) => {
                                if ops.requires_subscription() {
                                    listen_notify_handle
                                        .subscribe(ops.table_name().into(), session_id)
                                        .await
                                        .unwrap();
                                }
                                let crud = *ops.as_ref();

                                match ops.execute(&mut conn).await {
                                    Ok(None) => {}
                                    Ok(Some(row)) => {
                                        session
                                            .binary(B2FMessage::from((row.clone(), crud)))
                                            .await
                                            .unwrap();
                                        listen_notify_handle
                                            .notify((row, crud).into())
                                            .await
                                            .unwrap();
                                    }
                                    Err(_error) => {
                                        todo!("Handle ops error");
                                    }
                                }
                            }
                            Ok(F2BMessage::Table(ops)) => {
                                if ops.requires_subscription() {
                                    listen_notify_handle
                                        .subscribe(ops.table_name().into(), session_id)
                                        .await
                                        .unwrap();
                                }
                                let crud = *ops.as_ref();
                                match ops.execute(&mut conn).await {
                                    Ok(rows) => {
                                        if !rows.is_empty() {
                                            session
                                                .binary(B2FMessage::from((rows.clone(), crud)))
                                                .await
                                                .unwrap();
                                            listen_notify_handle
                                                .notify((rows, crud).into())
                                                .await
                                                .unwrap();
                                        }
                                    }
                                    Err(_error) => {
                                        todo!("Handle ops error");
                                    }
                                }
                            }
                            Err(err) => {
                                log::warn!("failed to parse message: {err}");
                            }
                        }
                    }

                    AggregatedMessage::Close(reason) => break reason,
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                log::error!("{err}");
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(database_message), _)), _)) => {
                session.binary(B2FMessage::from(database_message)).await.unwrap();
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => {
                unreachable!(
                    "all connection message senders were dropped; chat server may have panicked"
                )
            }

            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    let _ = listen_notify_handle.disconnect(session_id).await;

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}
