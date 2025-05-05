//! Submodule proving the Websocket-related methods for the `DBWSWorker`
//! struct, including operations such as connecting to the WebSocket.

use api_path::api::ws::FULL_ENDPOINT;
use futures::{SinkExt, StreamExt, channel::mpsc::channel};
use gloo::net::websocket::futures::WebSocket;
use ws_messages::{B2FMessage, F2BMessage};
use yew::platform::spawn_local;

use super::internal_message::ws_internal_message::WSInternalMessage;

const DOMAIN: Option<&str> = option_env!("DOMAIN");

impl super::DBWSWorker {
    pub(super) fn update_websocket<M>(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        ws_message: M,
    ) where
        M: Into<WSInternalMessage>,
    {
        match ws_message.into() {
            WSInternalMessage::B2F(message) => {
                match message {
                    B2FMessage::DB(db_message) => {
                        scope.send_message(db_message);
                    }
                }
            }
            WSInternalMessage::F2B(message) => {
                if let Some(websocket) = self.websocket.as_mut() {
                    if let Err(err) = websocket.try_send(message) {
                        scope.send_message(err);
                    }
                } else {
                    // If the web socket is not connected, we attempt to
                    // connect again, but we loose the current message.
                    // Important messages should be stored in the DB to
                    // be sent later before submitting the internal message.
                    scope.send_message(WSInternalMessage::Connect);
                }
            }
            WSInternalMessage::Connect => {
                let domain = DOMAIN.unwrap_or("localhost");
                let websocket = match WebSocket::open(&format!("wss://{domain}{FULL_ENDPOINT}")) {
                    Ok(websocket) => websocket,
                    Err(err) => {
                        unimplemented!("Error opening WebSocket: {:?}", err);
                    }
                };

                let (mut write, mut read) = websocket.split();
                let (sender, mut receiver) = channel::<F2BMessage>(1000);

                {
                    let scope = scope.clone();
                    spawn_local(async move {
                        while let Some(message) = receiver.next().await {
                            match message.try_into() {
                                Ok(message) => {
                                    if let Err(err) = write.send(message).await {
                                        scope.send_message(err);
                                        break;
                                    }
                                }
                                Err(err) => {
                                    scope.send_message(err);
                                }
                            }
                        }
                        log::debug!("Websocket sender closed");
                    });
                }

                {
                    let scope = scope.clone();
                    spawn_local(async move {
                        while let Some(backend_message) = read.next().await {
                            match backend_message {
                                Ok(message) => {
                                    match message.try_into() {
                                        Ok(message) => {
                                            scope.send_message(WSInternalMessage::B2F(message));
                                        }
                                        Err(err) => {
                                            scope.send_message(err);
                                        }
                                    }
                                }
                                Err(err) => {
                                    scope.send_message(err);
                                    break;
                                }
                            };
                        }
                        scope.send_message(WSInternalMessage::Connect);
                    });
                }

                self.websocket = Some(sender);
            }
        }
    }
}
