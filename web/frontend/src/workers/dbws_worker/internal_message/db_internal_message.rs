//! Submodule providing the enumeration of internal messages used
//! for the DB portion of the DB/WebSocket worker.
use diesel::SqliteConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use sqlite_wasm_rs::export::{OpfsSAHError, OpfsSAHPoolUtil};

pub enum DBInternalMessage {
    Connected(Result<SyncConnectionWrapper<SqliteConnection>, diesel::ConnectionError>),
    SAHPool(Result<OpfsSAHPoolUtil, OpfsSAHError>),
    InstallSAHPool,
}

impl From<Result<SyncConnectionWrapper<SqliteConnection>, diesel::ConnectionError>>
    for DBInternalMessage
{
    fn from(
        value: Result<SyncConnectionWrapper<SqliteConnection>, diesel::ConnectionError>,
    ) -> Self {
        Self::Connected(value)
    }
}

impl From<Result<OpfsSAHPoolUtil, OpfsSAHError>> for DBInternalMessage {
    fn from(value: Result<OpfsSAHPoolUtil, OpfsSAHError>) -> Self {
        Self::SAHPool(value)
    }
}

impl From<Result<SyncConnectionWrapper<SqliteConnection>, diesel::ConnectionError>>
    for super::InternalMessage
{
    fn from(
        value: Result<SyncConnectionWrapper<SqliteConnection>, diesel::ConnectionError>,
    ) -> Self {
        Self::DB(value.into())
    }
}

impl From<Result<OpfsSAHPoolUtil, OpfsSAHError>> for super::InternalMessage {
    fn from(value: Result<OpfsSAHPoolUtil, OpfsSAHError>) -> Self {
        Self::DB(value.into())
    }
}
