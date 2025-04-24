//! Hybrid Web Worker handling the SQLite database and the WebSocket connection.
//!
//! This worker is responsible for managing both the SQLite database and the
//! WebSocket connection as at the time of writing (2025-04-24) is is not known
//! how to send messages between Web Workers in the yew framework, or whether it
//! would be desirable due to the potential overhead.
//!
//! Since it is unclear whether between worker communication is possible, this
//! worker handles both the database and WebSocket connection to allow for
//! internal comunication between the two.
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use api_path::api::ws::FULL_ENDPOINT;
use core_structures::User;
use db_worker::listen_notify::ListenNotify;
use diesel::SqliteConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use internal_message::db_internal_message::DBInternalMessage;
use rosetta_uuid::Uuid;
use ws_messages::F2BMessage;
use yew_agent::worker::{HandlerId, Worker};

mod c2db_message;
pub(crate) use c2db_message::C2DBMessage;
mod db2c_message;
pub(crate) use db2c_message::DB2CMessage;
mod internal_message;
pub(crate) use internal_message::InternalMessage;
mod db_worker;
mod ws_worker;

const NOMINAL_CLOSURE_CODE: u16 = 1000;

/// Worker for handling WebSocket operations.
pub struct DBWSWorker {
    subscribers: HashSet<HandlerId>,
    tasks: HashMap<Uuid, HandlerId>,
    user: Option<Rc<User>>,
    conn: Option<SyncConnectionWrapper<SqliteConnection>>,
    listen_notify: ListenNotify,
    websocket_sender: Option<futures::channel::mpsc::Sender<F2BMessage>>,
}

impl DBWSWorker {
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

impl Worker for DBWSWorker {
    type Message = InternalMessage;
    type Input = C2DBMessage;
    type Output = DB2CMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        scope.send_message(DBInternalMessage::InstallSAHPool);
        Self {
            subscribers: HashSet::new(),
            tasks: HashMap::new(),
            user: None,
            websocket_sender: None,
            listen_notify: ListenNotify::default(),
            conn: None,
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::DB(db_message) => {
                self.update_database(scope, db_message);
            }
            InternalMessage::WS(ws_message) => {
                self.update_websocket(scope, ws_message);
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
