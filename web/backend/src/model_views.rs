//! Submodule handling the views in the database.
//!
//! # Implementative details
//! Diesel does not natively support the SQL views. As such, we have to manually
//! create the analogues of the table structs for the views. This module contains
//! the structs for the views and the functions to interact with them.

use crate::schema_views::documents_view;
use crate::DieselConn;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Clone, Serialize, Debug, Deserialize)]
#[diesel(table_name = documents_view)]
/// Struct representing the documents view in the database.
pub struct DocumentView {
    pub id: Uuid,
    pub format_id: Uuid,
    pub document_path: String,
    pub bytes: i32,
    pub document_name: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub creator_first_name: Option<String>,
    pub creator_middle_name: Option<String>,
    pub creator_last_name: Option<String>,
    pub last_edit_id: Option<Uuid>,
    pub last_edit_at: Option<String>,
    pub last_editor_id: Option<Uuid>,
    pub last_editor_first_name: Option<String>,
    pub last_editor_middle_name: Option<String>,
    pub last_editor_last_name: Option<String>,
    pub last_edit_title: Option<String>,
    pub last_edit_extended_reason: Option<String>,
    pub extension: String,
}

impl DocumentView {
    pub fn get(conn: &mut DieselConn, id: Uuid) -> Result<DocumentView, diesel::result::Error> {
        documents_view::table
            .filter(documents_view::id.eq(id))
            .first(conn)
    }

    pub fn internal_path(&self) -> String {
        format!(
            "{}/{}.{}",
            std::env::var("DOCUMENTS_DIRECTORY").unwrap(),
            self.id,
            self.extension
        )
    }
}
