use serde::Deserialize;
use serde::Serialize;
use diesel::Queryable;
use diesel::QueryableByName;
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use crate::views::schema::*;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Queryable, QueryableByName, )]
#[diesel(table_name = public_user)]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<PublicUser> for web_common::database::views::PublicUser {
    fn from(item: PublicUser) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::views::PublicUser> for PublicUser {
    fn from(item: web_common::database::views::PublicUser) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl PublicUser {
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ViewRow {
    PublicUser(PublicUser),
}

impl From<web_common::database::views::ViewRow> for ViewRow {
    fn from(item: web_common::database::views::ViewRow) -> Self {
        match item {
            web_common::database::views::ViewRow::PublicUser(item) => ViewRow::PublicUser(item.into()),
        }
    }
}
impl From<ViewRow> for web_common::database::views::ViewRow {
    fn from(item: ViewRow) -> Self {
        match item {
            ViewRow::PublicUser(item) => web_common::database::views::ViewRow::PublicUser(item.into()),
        }
    }
}
impl From<PublicUser> for ViewRow {
    fn from(item: PublicUser) -> Self {
        ViewRow::PublicUser(item)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy, Eq, )]
pub enum View {
    PublicUser,
}

impl View {
    pub fn name(&self) -> &'static str {
        match self {
            View::PublicUser => "public_user",
        }
    }
}
impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
impl From<&str> for View {
    fn from(item: &str) -> Self {
        match item {
            "public_user" => View::PublicUser,
            _ => panic!("Unknown views name"),
        }
    }
}
impl From<web_common::database::views::View> for View {
    fn from(item: web_common::database::views::View) -> Self {
        match item {
            web_common::database::views::View::PublicUser => View::PublicUser,
        }
    }
}
impl From<View> for web_common::database::views::View {
    fn from(item: View) -> Self {
        match item {
            View::PublicUser => web_common::database::views::View::PublicUser,
        }
    }
}
