//! GlueSQL-based client-side database for handling the frontend state.
//!
//! # Implementative details
//! The frontend state is stored in a GlueSQL database, which is a simple, IndexedDB-based database.
//! Since the application needs to be able to work offline, the database is designed to be able to work
//! offline as well, without having to consult the server for every operation.
//! You can learn more about GlueSQL [here](https://gluesql.org/docs/0.15/)
//! and about IndexedDB [here](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API).
use gluesql::prelude::*;

/// Initializes the database.
pub(crate) async fn init_database() -> Result<Glue<IdbStorage>, Error> {
    let storage = IdbStorage::new(Some("Earth Metabolome Initiative".to_string())).await?;
    let glue = Glue::new(storage);
    Ok(glue)
}
