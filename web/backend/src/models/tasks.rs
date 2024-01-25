use crate::repository::schema::*;
use diesel::backend::Backend;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::{self, FromSql};
use diesel::prelude::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::SmallInt;
use diesel::*;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub status: TaskStatus,
    pub task_type_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub started_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub user_id: i32,
    pub status: TaskStatus,
    pub task_type_id: i32,
}

/// This is the enum that will be stored in the database
///
/// # Implementative details
/// You can read more about the diesel_enum crate here: https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/custom_types.rs
#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = SmallInt)]
pub enum TaskStatus {
    PENDING,
    STARTED,
    SUCCESS,
    FAILURE,
}

impl<DB> ToSql<SmallInt, DB> for TaskStatus
where
    DB: Backend,
    u8: ToSql<SmallInt, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match *self {
            TaskStatus::PENDING => 0_u8.to_sql(out),
            TaskStatus::STARTED => 1_u8.to_sql(out),
            TaskStatus::SUCCESS => 2_u8.to_sql(out),
            TaskStatus::FAILURE => 3_u8.to_sql(out),
        }
    }
}

impl<DB> FromSql<SmallInt, DB> for TaskStatus
where
    DB: Backend,
    u8: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match u8::from_sql(bytes)? {
            0 => Ok(TaskStatus::PENDING),
            1 => Ok(TaskStatus::STARTED),
            2 => Ok(TaskStatus::SUCCESS),
            3 => Ok(TaskStatus::FAILURE),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
