use crate::models::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Define the TaskType model
#[derive(Deserialize, Serialize, Debug, Queryable, Identifiable)]
#[diesel(table_name = task_types)]
pub struct TaskType {
    pub id: i32,
    pub name: String,
    pub description: String,
}

// Define the TaskType model for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = task_types)]
pub struct NewTaskType<'a> {
    pub name: &'a str,
    pub description: &'a str,
}
