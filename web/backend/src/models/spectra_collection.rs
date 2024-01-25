use crate::models::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Define the SpectraCollection model
#[derive(Deserialize, Serialize, Debug, Queryable, Identifiable)]
#[diesel(table_name = spectra_collection)]
pub struct SpectraCollection {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub sample_id: i32,
    pub user_id: i32,
    pub updated_by_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Define the SpectraCollection model for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = spectra_collection)]
pub struct NewSpectraCollection<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub sample_id: i32,
    pub user_id: i32,
    pub updated_by_id: i32,
}