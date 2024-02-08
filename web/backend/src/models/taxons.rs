use crate::models::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Define the Taxon model
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = taxons)]
pub struct Taxon {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Define the Taxon model for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = taxons)]
pub struct NewTaxon<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub user_id: i32,
}
