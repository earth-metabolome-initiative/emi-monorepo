use crate::models::schema::*;
use diesel::prelude::*;

// Define the Document model
#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = documents)]
pub struct Document {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub path: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Define the Document model for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub path: &'a str,
    pub user_id: i32,
}