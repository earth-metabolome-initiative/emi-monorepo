//! GlueSQL-based client-side database for handling the frontend state.
//!
//! # Implementative details
//! The frontend state is stored in a GlueSQL database, which is a simple, IndexedDB-based database.
//! Since the application needs to be able to work offline, the database is designed to be able to work
//! offline as well, without having to consult the server for every operation.
//! You can learn more about GlueSQL [here](https://gluesql.org/docs/0.15/)
//! and about IndexedDB [here](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API).
use gluesql::prelude::*;

pub type Database = Glue<IdbStorage>;

/// Initializes the database.
pub(crate) async fn init_database() -> Result<Database, Error> {
    let storage = IdbStorage::new(Some("Earth Metabolome Initiative".to_string())).await?;
    let mut glue = Glue::new(storage);

    // For most of the tables, we can fortunately reuse the SQL used for the tables creation
    // in the diesel migration in the backend, which we can load as a static string on compilation,
    // hence not requiring to either duplicate the code or generate it at runtime when the website
    // is loaded and server requests would be needed to fetch the schema.
    let taxa = include_str!("../../backend/migrations/00000000000012_create_taxa_table/up.sql");

    let result = glue.execute(taxa).await?;
    log::info!("Result of executing the SQL for creating the taxa table: {:?}", result);

    Ok(glue)
}
