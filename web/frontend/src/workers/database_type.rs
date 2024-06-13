//! Submodule handling the database type.
//!
//! # Implementative details
//! The reason for this file being separated is that Rust Analyzer fails to
//! understand that the target architecture of this crate is WebAssembly,
//! and as such it also fails to load IdbStorage.
//! The crate is still compiled correctly, but the IDE is unable to provide
//! autocompletion and type checking for the IdbStorage type.
use gluesql::prelude::*;

pub type Database = Glue<IdbStorage>;
use super::database_schema::create_schema;

pub(super) async fn create_database() -> Database {
    let mut database = Glue::new(
        IdbStorage::new(Some("Earth Metabolome Initiative".to_string()))
            .await
            .unwrap(),
    );
    create_schema(&mut database).await;
    database
}
