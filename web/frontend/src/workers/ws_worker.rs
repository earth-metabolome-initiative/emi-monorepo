use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use api_path::api::ws::FULL_ENDPOINT;
use core_structures::User;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use rosetta_uuid::Uuid;
use serde::{Deserialize, Serialize};
use ws_messages::{B2FMessage, F2BMessage};
use yew::platform::spawn_local;
use yew_agent::worker::{HandlerId, Worker};

const NOMINAL_CLOSURE_CODE: u16 = 1000;

type UserId = i32;

pub struct WebsocketWorker {
    subscribers: HashSet<HandlerId>,
    tasks: HashMap<Uuid, HandlerId>,
    user: Option<Rc<User>>,
    websocket_sender: Option<futures::channel::mpsc::Sender<F2BMessage>>,
    reconnection_attempt: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the frontend to the web-worker.
pub enum ComponentMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messages from the websocket web-worker to the frontend.
pub enum WebsocketMessage {}

pub enum InternalMessage {
    Backend(B2FMessage),
}

impl WebsocketWorker {
    fn connect_to_websocket(
        hostname: &str,
        scope: &yew_agent::prelude::WorkerScope<Self>,
    ) -> Result<futures::channel::mpsc::Sender<F2BMessage>, String> {
        let websocket = WebSocket::open(&format!("wss://{hostname}{FULL_ENDPOINT}"))
            .map_err(|err| format!("Error opening websocket connection: {:?}", err))?;

        match websocket.state() {
            gloo_net::websocket::State::Open => {}
            gloo_net::websocket::State::Connecting => {}
            _ => {
                return Err("Websocket connection is not open".to_string());
            }
        }

        let (mut write, mut read) = websocket.split();

        let (sender, mut receiver) = futures::channel::mpsc::channel::<F2BMessage>(1000);

        // spawn_local(async move {
        //     while let Some(frontend_message) = receiver.next().await {
        //         if write.send(frontend_message.into()).await.is_err() {
        //             log::error!("Error sending to websocket");
        //             break;
        //         }
        //     }
        //     log::debug!("Websocket sender closed");
        // });

        let hostname = hostname.to_owned();

        // {
        //     let scope = scope.clone();
        //     spawn_local(async move {
        //         while let Some(backend_message) = read.next().await {
        //             match backend_message {
        //                 Ok(message) => {
        //                     match message.try_into() {
        //                         Ok(message) => {
        //
        // scope.send_message(InternalMessage::Backend(message));
        // }                         Err(err) => {
        //                             log::error!("Error deserializing message: {:?}",
        // err);                         }
        //                     }
        //                 }
        //                 Err(err) => {
        //                     log::error!("Error reading from websocket: {:?}", err);
        //                     break;
        //                 }
        //             }
        //         }
        //         scope.send_message(InternalMessage::Reconnect(hostname));
        //     });
        // }

        Ok(sender)
    }
}

impl Worker for WebsocketWorker {
    type Message = InternalMessage;
    type Input = ComponentMessage;
    type Output = WebsocketMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        Self {
            subscribers: HashSet::new(),
            tasks: HashMap::new(),
            user: None,
            websocket_sender: None,
            reconnection_attempt: 0,
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::Backend(backend_message) => {
                match backend_message {
                    B2FMessage::Pong => {
                        log::debug!("Received pong from backend");
                    }
                };
            }
        }
    }

    fn connected(
        &mut self,
        _scope: &yew_agent::prelude::WorkerScope<Self>,
        id: yew_agent::worker::HandlerId,
    ) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, _scope: &yew_agent::prelude::WorkerScope<Self>, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn received(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        frontend_message: Self::Input,
        subscriber_id: HandlerId,
    ) {
        todo!();
    }
}
