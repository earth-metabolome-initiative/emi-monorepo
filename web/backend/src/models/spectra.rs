use crate::models::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Define the Spectrum model
#[derive(Deserialize, Serialize, Debug, Queryable, Identifiable)]
#[diesel(table_name = spectra)]
pub struct Spectrum {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub spectra_collection_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Define the Spectrum model for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = spectra)]
pub struct NewSpectrum<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub spectra_collection_id: i32,
}
