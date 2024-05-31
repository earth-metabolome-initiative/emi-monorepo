//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use crate::sql_function_bindings::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use web_common::database::filter_structs::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = countries)]
#[diesel(primary_key(id))]
pub struct Country {
    pub id: i32,
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
}

impl From<Country> for web_common::database::tables::Country {
    fn from(item: Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::Country> for Country {
    fn from(item: web_common::database::tables::Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl Country {
    /// Check whether the user can view the struct.
    pub fn can_view(
        &self,
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id(
) -> Result<bool, web_common::api::ApiError>{
        Ok(true)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its emoji.
    ///
    /// * `emoji` - The emoji of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_emoji(
emoji: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::emoji.eq(emoji))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its iso.
    ///
    /// * `iso` - The iso of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_iso(
iso: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::iso.eq(iso))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
name: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its unicode.
    ///
    /// * `unicode` - The unicode of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_unicode(
unicode: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::countries;
        let flat_variant = countries::dsl::countries
            .filter(countries::dsl::unicode.eq(unicode))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(
similarity_op(countries::dsl::name, query))
            .order(
similarity_dist(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(
word_similarity_op(countries::dsl::name, query))
            .order(
word_similarity_dist_op(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(limit, offset, connection);
        }
        use crate::schema::countries;
        countries::dsl::countries
            .filter(
strict_word_similarity_op(countries::dsl::name, query))
            .order(
strict_word_similarity_dist_op(countries::dsl::name, query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
