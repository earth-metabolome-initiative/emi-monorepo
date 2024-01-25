use diesel::deserialize::FromSqlRow;
use diesel::prelude::*;
use diesel::*;
use diesel::sql_types::SmallInt;
use diesel_enum::DbEnum;

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = crate::repository::schema::tasks)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub status: TaskStatus,
    pub task_type_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::repository::schema::tasks)]
pub struct NewTask {
    pub user_id: i32,
    pub status: TaskStatus,
    pub task_type_id: i32,
}


#[derive(Debug, thiserror::Error)]
#[error("CustomError: {msg}, {status}")]
pub struct CustomError {
    msg: String,
    status: u16,
}

impl CustomError {
    fn not_found(msg: String) -> Self {
        Self {
            msg,
            status: 404,
        }
    }
}

/// This is the enum that will be stored in the database
/// 
/// # Implementative details
/// You can read more about the diesel_enum crate here: https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, DbEnum, SqlType, Eq)]
#[diesel(sql_type = SmallInt)]
#[diesel_enum(error_fn = CustomError::not_found)]
#[diesel_enum(error_type = CustomError)]
pub enum TaskStatus {
    PENDING,
    STARTED,
    SUCCESS,
    FAILURE,
}
