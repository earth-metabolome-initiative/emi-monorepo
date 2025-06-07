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

use super::{
    C2DBMessage, DBWSWorker, InternalMessage,
    internal_message::db_internal_message::DBInternalMessage,
};

pub mod listen_notify;

const CSV_MIGRATIONS: &str =
    sqlite_migration_generator::load_sqlite_from_csvs!("../../data_migrations/init_db/csvs");
const MIGRATIONS: &str = sqlite_migration_generator::load_sqlite_from_migrations!(
    "../../data_migrations/init_db/migrations"
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
                        let Some(conn) = self.conn.as_mut() else {
                            scope.send_future(async move {
                                gloo_timers::future::sleep(std::time::Duration::from_millis(500))
                                    .await;
                                DBInternalMessage::DB(DBMessage::Row(row, crud))
                            });
                            return;
                        };

                        console::log_1(&format!("Executing upsert on row: {row:?}").into());

                        let updated_row = match row.sqlite_upsert(conn) {
                            Ok(rows) => rows,
                            Err(err) => {
                                console::log_1(&format!("Error updating row: {err:?}").into());
                                scope.send_message(err);
                                return;
                            }
                        };

                        console::log_1(&format!("Updated row: {updated_row:?}").into());

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
                // We received a message with information regarding a table read,
                // which we need to integrate into the SQLite database and then
                // notify the components that are listening to this table.

                let Some(conn) = self.conn.as_mut() else {
                    scope.send_future(async move {
                        gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                        DBInternalMessage::DB(DBMessage::Rows(rows, crud))
                    });
                    return;
                };

                console::log_1(&format!("Executing upsert on row: {rows:?}").into());

                // TODO! Actually update according to CRUD!
                let updated_rows = match rows.sqlite_upsert(conn) {
                    Ok(rows) => rows,
                    Err(err) => {
                        console::log_1(&format!("Error updating rows: {err:?}").into());
                        scope.send_message(err);
                        return;
                    }
                };

                console::log_1(&format!("Updated rows: {updated_rows:?}").into());

                self.listen_notify.notify_rows_listeners(&updated_rows, crud, scope);
            }
            DBInternalMessage::Connect => {
                match SqliteConnection::establish("file:emi.db?vfs=opfs-sahpool") {
                    Ok(mut conn) => {
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
                    Err(err) => {
                        scope.send_message(err);
                    }
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
                let Some(conn) = self.conn.as_mut() else {
                    // If the connection is not established yet, we submit the
                    // message to the future, waiting one second before trying again.
                    scope.send_future(async move {
                        gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                        InternalMessage::RetryC2DB((
                            C2DBMessage::Row(operation).into(),
                            subscriber_id,
                        ))
                    });
                    return;
                };

                // We execute the query locally and send the result back to the component.
                let operation_kind = *operation.as_ref();
                // Secondly, we register the component as a listener
                // for the modification of the table.
                self.listen_notify.register_row_listener(subscriber_id, operation.primary_key());

                match operation.clone().execute(conn) {
                    Ok(Some(row)) => {
                        scope.respond(subscriber_id, (row, operation_kind).into());
                    }
                    Ok(None) => {}
                    Err(err) => {
                        scope.send_message(err);
                    }
                }

                // Finally, we also submit the query to the database
                // via the WebSocket connection.
                self.update_websocket(scope, operation);
            }
            C2DBMessage::Table(operation) => {
                let Some(conn) = self.conn.as_mut() else {
                    // If the connection is not established yet, we submit the
                    // message to the future, waiting one second before trying again.
                    scope.send_future(async move {
                        gloo_timers::future::sleep(std::time::Duration::from_millis(500)).await;
                        InternalMessage::RetryC2DB((
                            C2DBMessage::Table(operation).into(),
                            subscriber_id,
                        ))
                    });
                    return;
                };
                // Secondly, we register the component as a listener
                // for the modification of the table.
                self.listen_notify.register_table_listener(subscriber_id, operation.table_name());

                // We execute the query locally and send the result back to the component.
                let operation_kind = *operation.as_ref();

                match operation.clone().execute(conn) {
                    Ok(rows) => {
                        scope.respond(subscriber_id, (rows, operation_kind).into());
                    }
                    Err(err) => {
                        scope.send_message(err);
                    }
                }

                // Finally, we also submit the query to the database
                // via the WebSocket connection.
                self.update_websocket(scope, operation);
            }
        }
    }
}
