//! Hybrid Web Worker handling the `SQLite` database and the WebSocket
//! connection.
//!
//! This worker is responsible for managing both the `SQLite` database and the
//! WebSocket connection as at the time of writing (2025-04-24) is is not known
//! how to send messages between Web Workers in the yew framework, or whether it
//! would be desirable due to the potential overhead.
//!
//! Since it is unclear whether between worker communication is possible, this
//! worker handles both the database and WebSocket connection to allow for
//! internal communication between the two.

use db_worker::listen_notify::ListenNotify;
use diesel::SqliteConnection;
use internal_message::ws_internal_message::WSInternalMessage;
use web_sys::console;
use ws_messages::{DBMessage, F2BMessage, frontend::Subscription};
use yew_agent::worker::{HandlerId, Worker};

mod c2db_message;
pub(crate) use c2db_message::C2DBMessage;
mod internal_message;
pub(crate) use internal_message::InternalMessage;
mod component_message;
mod db_worker;
mod ws_worker;
pub use component_message::ComponentMessage;
use futures::channel::mpsc::Sender;

/// Worker for handling WebSocket operations.
pub struct DBWSWorker {
    conn: Option<SqliteConnection>,
    listen_notify: ListenNotify,
    websocket: Option<Sender<F2BMessage>>,
}

impl Worker for DBWSWorker {
    type Message = InternalMessage;
    type Input = ComponentMessage;
    type Output = DBMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        console::log_1(&"Creating DBWSWorker".into());
        scope.send_future(sqlite_wasm_rs::export::install_opfs_sahpool(None, true));
        scope.send_message(WSInternalMessage::Connect);
        Self { websocket: None, conn: None, listen_notify: ListenNotify::default() }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        console::log_1(&format!("Received internal message: {internal_message:?}").into());
        match internal_message {
            InternalMessage::RetryC2DB((component_message, subscriber_id)) => {
                self.received(scope, component_message, subscriber_id);
            }
            InternalMessage::DB(db_message) => {
                self.update_database(scope, db_message);
            }
            InternalMessage::WS(ws_message) => {
                self.update_websocket(scope, ws_message);
            }
            InternalMessage::DBError(err) => {
                console::log_1(&format!("Database error: {err:?}").into());
            }
            InternalMessage::WSError(err) => {
                console::log_1(&format!("WebSocket error: {err:?}").into());
            }
        }
    }

    fn disconnected(&mut self, scope: &yew_agent::prelude::WorkerScope<Self>, id: HandlerId) {
        for orphan_table_name in self.listen_notify.remove_table_listener(id) {
            scope.send_message(Subscription::from(orphan_table_name));
        }
        for orphan_primary_key in self.listen_notify.remove_row_listener(id) {
            scope.send_message(Subscription::from(orphan_primary_key));
        }
    }

    fn connected(&mut self, _scope: &yew_agent::prelude::WorkerScope<Self>, id: HandlerId) {
        console::log_1(&format!("Connected to worker with id: {id:?}").into());
    }

    fn received(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        frontend_message: Self::Input,
        subscriber_id: HandlerId,
    ) {
        console::log_1(&format!("Received component message: {frontend_message:?}").into());
        match frontend_message {
            ComponentMessage::DB(db_message) => {
                self.received_query(scope, db_message, subscriber_id);
            }
        }
    }
}
