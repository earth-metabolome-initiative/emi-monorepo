//! Submodule proving the Database-related methods for the `DBWSWorker`
//! struct, including operations such as connecting to the database and
//! installing the SAHPool.

use diesel::SqliteConnection;
use diesel_async::{AsyncConnection, sync_connection_wrapper::SyncConnectionWrapper};
use yew_agent::worker::HandlerId;

use super::{C2DBMessage, DBWSWorker, internal_message::db_internal_message::DBInternalMessage};

pub mod listen_notify;

impl DBWSWorker {
    pub(super) fn update_database(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        db_message: DBInternalMessage,
    ) {
        match db_message {
            DBInternalMessage::Connected(connection) => {
                match connection {
                    Ok(conn) => {
                        self.conn = Some(conn);
                        //     conn.run_pending_migrations(MIGRATIONS).unwrap();
                    }
                    Err(err) => {
                        panic!("Error connecting to SQLite database: {err:?}");
                    }
                }
            }
            DBInternalMessage::SAHPool(sahpool) => {
                match sahpool {
                    Ok(_) => {
                        scope.send_future(SyncConnectionWrapper::<SqliteConnection>::establish(
                            "file:emi.db?vfs=opfs-sahpool",
                        ));
                    }
                    Err(err) => {
                        panic!("Error installing OPFS sahpool: {err:?}");
                    }
                }
            }
            DBInternalMessage::InstallSAHPool => {
                scope.send_future(sqlite_wasm_rs::export::install_opfs_sahpool(None, true));
            }
        }
    }

    pub(super) fn received_query(
        &mut self,
        _scope: &yew_agent::prelude::WorkerScope<Self>,
        _component_message: C2DBMessage,
        _subscriber_id: HandlerId,
    ) {
        todo!();
    }
}
