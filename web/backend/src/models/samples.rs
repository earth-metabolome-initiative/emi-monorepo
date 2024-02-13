use crate::models::schema::*;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable, Identifiable)]
#[diesel(table_name = samples)]
pub struct Sample {
    pub id: i32,
    pub taxon_id: i32,
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub derived_from: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = samples)]
pub struct NewSample<'a> {
    pub taxon_id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub derived_from: Option<i32>,
    pub user_id: i32,
}
