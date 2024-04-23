use serde::Deserialize;
use serde::Serialize;
use diesel::Queryable;
use diesel::QueryableByName;
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
#[derive(Deserialize, Serialize, Debug, PartialEq, Queryable, QueryableByName, Eq, Clone)]
#[diesel(table_name = crate::views::schema::public_user)]
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::views::schema::public_user;
        public_user::dsl::public_user
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::views::schema::public_user;
        public_user::dsl::public_user
            .filter(public_user::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}


