#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::views::schema::*;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Identifiable, QueryableByName, Queryable)]
#[diesel(table_name = public_users)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::views::schema::public_users;
        let query = public_users::dsl::public_users;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "WITH selected_ids AS (",
            "SELECT users.id AS id FROM users ",
            "WHERE $1 % f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)  ",
            "ORDER BY similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
         ")",
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id FROM public_users ",
            "JOIN selected_ids ON selected_ids.id = public_users.id"
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "WITH selected_ids AS (",
            "SELECT users.id AS id FROM users ",
            "WHERE $1 <% f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)  ",
            "ORDER BY word_similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
         ")",
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id FROM public_users ",
            "JOIN selected_ids ON selected_ids.id = public_users.id"
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "WITH selected_ids AS (",
            "SELECT users.id AS id FROM users ",
            "WHERE $1 <<% f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)  ",
            "ORDER BY strict_word_similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
         ")",
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at, thumbnail_id, picture_id FROM public_users ",
            "JOIN selected_ids ON selected_ids.id = public_users.id"
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
