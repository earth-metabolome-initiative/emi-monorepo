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
#[diesel(table_name = colors)]
#[diesel(primary_key(id))]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
    pub description: String,
}

impl From<Color> for web_common::database::tables::Color {
    fn from(item: Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl From<web_common::database::tables::Color> for Color {
    fn from(item: web_common::database::tables::Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl Color {
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
        use crate::schema::colors;
        colors::dsl::colors
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
        use crate::schema::colors;
        colors::dsl::colors
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
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::id.eq(id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its hexadecimal_value.
    ///
    /// * `hexadecimal_value` - The hexadecimal_value of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_hexadecimal_value(
hexadecimal_value: &str,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        use crate::schema::colors;
        let flat_variant = colors::dsl::colors
            .filter(colors::dsl::hexadecimal_value.eq(hexadecimal_value))
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
        use crate::schema::colors;
        let flat_variant = colors::dsl::colors
            .filter(colors::dsl::name.eq(name))
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
        use crate::schema::colors;
        colors::dsl::colors
            .filter(
similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order(
similarity_dist(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
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
        use crate::schema::colors;
        colors::dsl::colors
            .filter(
word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order(
word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
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
        use crate::schema::colors;
        colors::dsl::colors
            .filter(
strict_word_similarity_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .order(
strict_word_similarity_dist_op(concat_colors_name(colors::dsl::name, colors::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
}
