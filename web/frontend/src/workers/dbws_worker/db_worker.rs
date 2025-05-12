//! Submodule proving the Database-related methods for the `DBWSWorker`
//! struct, including operations such as connecting to the database and
//! installing the `SAHPool`.

use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use web_common_traits::{
    crud::{CRUD, ExecuteCrudOperation},
    database::{Row, Tabular},
};
use web_sys::console;
use ws_messages::DBMessage;
use yew_agent::worker::HandlerId;

use super::{C2DBMessage, DBWSWorker, internal_message::db_internal_message::DBInternalMessage};

pub mod listen_notify;

const CSV_MIGRATIONS: &str =
    sqlite_migration_generator::load_sqlite_from_csvs!("../../data-migrations/init-db/csvs");
const MIGRATIONS: &str = sqlite_migration_generator::load_sqlite_from_migrations!(
    "../../data-migrations/init-db/migrations"
);

impl DBWSWorker {
    pub(super) fn update_database(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        db_message: DBInternalMessage,
    ) {
        match db_message {
            DBInternalMessage::DB(DBMessage::Row(row, crud)) => {
                // We received a message with information regarding a table read,
                // which we need to integrate into the SQLite database and then
                // notify the components that are listening to this table.

                match crud {
                    CRUD::Update | CRUD::Create | CRUD::Read => {
                        let updated_row = match row.sqlite_upsert(self.conn.as_mut().unwrap()) {
                            Ok(rows) => rows,
                            Err(err) => {
                                scope.send_message(err);
                                return;
                            }
                        };

                        if let Some(updated_row) = updated_row {
                            self.listen_notify.notify_row_listeners(&updated_row, crud, scope);
                        }
                    }
                    CRUD::Delete => {
                        todo!("Handle delete operation");
                    }
                }
            }
            DBInternalMessage::DB(DBMessage::Rows(rows, crud)) => {
                console::log_1(&format!("Received rows: {rows:?} for crud: {crud}").into());
                // We received a message with information regarding a table read,
                // which we need to integrate into the SQLite database and then
                // notify the components that are listening to this table.

                // TODO! Actually update according to CRUD!
                let updated_rows = match rows.sqlite_upsert(self.conn.as_mut().unwrap()) {
                    Ok(rows) => rows,
                    Err(err) => {
                        scope.send_message(err);
                        return;
                    }
                };

                self.listen_notify.notify_rows_listeners(&updated_rows, crud, scope);
            }
            DBInternalMessage::Connect => {
                console::log_1(&"Connecting to SQLite database".into());
                match SqliteConnection::establish("file:emi.db?vfs=relaxed-idb") {
                    Ok(mut conn) => {
                        console::log_1(&"Connected to SQLite database".into());
                        if let Err(err) = conn.batch_execute(CSV_MIGRATIONS) {
                            scope.send_message(err);
                            return;
                        }
                        if let Err(err) = conn.batch_execute(MIGRATIONS) {
                            scope.send_message(err);
                            return;
                        }
                        self.conn = Some(conn);
                    }
                    Err(err) => scope.send_message(err),
                }
            }
        }
    }

    pub(super) fn received_query(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        component_message: C2DBMessage,
        subscriber_id: HandlerId,
    ) {
        match component_message {
            C2DBMessage::Row(operation) => {
                // We execute the query locally and send the result back to the component.
                let operation_kind = *operation.as_ref();
                // Secondly, we register the component as a listener
                // for the modification of the table.
                self.listen_notify.register_row_listener(subscriber_id, operation.primary_key());

                // Finally, we also submit the query to the database
                // via the WebSocket connection.
                self.update_websocket(scope, operation.clone());

                match operation.execute(self.conn.as_mut().unwrap()) {
                    Ok(Some(row)) => {
                        scope.respond(subscriber_id, (row, operation_kind).into());
                    }
                    Ok(None) => {}
                    Err(err) => {
                        scope.send_message(err);
                    }
                }
            }
            C2DBMessage::Table(operation) => {
                // Secondly, we register the component as a listener
                // for the modification of the table.
                self.listen_notify.register_table_listener(subscriber_id, operation.table_name());

                // Finally, we also submit the query to the database
                // via the WebSocket connection.
                self.update_websocket(scope, operation.clone());
                // We execute the query locally and send the result back to the component.
                let operation_kind = *operation.as_ref();
                match operation.execute(self.conn.as_mut().unwrap()) {
                    Ok(rows) => {
                        scope.respond(subscriber_id, (rows, operation_kind).into());
                    }
                    Err(err) => {
                        scope.send_message(err);
                    }
                }
            }
        }
    }
}
