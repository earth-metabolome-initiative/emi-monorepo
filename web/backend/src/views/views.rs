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
#[diesel(table_name = crate::views::schema::public_users)]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub thumbnail_id: Option<Uuid>,
    pub picture_id: Option<Uuid>,
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
            thumbnail_id: item.thumbnail_id,
            picture_id: item.picture_id,
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
            thumbnail_id: item.thumbnail_id,
            picture_id: item.picture_id,
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
        use crate::views::schema::public_users;
        public_users::dsl::public_users
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
        use crate::views::schema::public_users;
        public_users::dsl::public_users
            .filter(public_users::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `threshold` - The similarity threshold, by default `0.6`.
    /// * `connection` - The connection to the database.
    ///
    pub fn search(
        query: &str,
        limit: Option<i32>,
        threshold: Option<f64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        let threshold = threshold.unwrap_or(0.3);
        let similarity_query = concat!(
            "WITH selected_ids AS (",
            "SELECT users.id FROM users ",
            "ORDER BY similarity(users.first_name, $1) + similarity(users.middle_name, $1) + similarity(users.last_name, $1) DESC LIMIT $3",
         ")",
            "SELECT public_users.id, public_users.first_name, public_users.middle_name, public_users.last_name, public_users.created_at, public_users.updated_at, public_users.thumbnail_id, public_users.picture_id FROM public_users ",
            "JOIN selected_ids ON selected_ids.id = public_users.id"
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Float8, _>(threshold)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}


