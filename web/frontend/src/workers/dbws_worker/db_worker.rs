//! Submodule proving the Database-related methods for the `DBWSWorker`
//! struct, including operations such as connecting to the database and
//! installing the `SAHPool`.

use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use yew_agent::worker::HandlerId;

use super::{C2DBMessage, DBWSWorker, internal_message::db_internal_message::DBInternalMessage};

pub mod listen_notify;

const CSV_MIGRATIONS: &str =
    sqlite_migration_generator::load_sqlite_from_csvs!("../core-structures/csvs");
const MIGRATIONS: &str =
    sqlite_migration_generator::load_sqlite_from_migrations!("../core-structures/migrations");

impl DBWSWorker {
    pub(super) fn update_database(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        db_message: DBInternalMessage,
    ) {
        match db_message {
            DBInternalMessage::SAHPoolInstalled => {
                self.conn = Some(
                    SqliteConnection::establish("file:emi.db?vfs=opfs-sahpool")
                        .expect("Failed to establish connection"),
                );
                if let Err(err) = self.conn.as_mut().unwrap().batch_execute(CSV_MIGRATIONS) {
                    scope.send_message(err);
                }
                if let Err(err) = self.conn.as_mut().unwrap().batch_execute(MIGRATIONS) {
                    scope.send_message(err);
                }
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
