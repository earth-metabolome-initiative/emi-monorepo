//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::schema::*;
use diesel::prelude::*;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use web_common::database::filter_structs::*;

#[derive(
    Queryable,
    Debug,
    Identifiable,
    Eq,
    PartialEq,
    PartialOrd,
    Clone,
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = nameplates)]
#[diesel(belongs_to(crate::models::projects::Project, foreign_key = project_id))]
#[diesel(belongs_to(crate::models::nameplate_categories::NameplateCategory, foreign_key = category_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Nameplate {
    pub id: i32,
    pub barcode: String,
    pub project_id: i32,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<Nameplate> for web_common::database::tables::Nameplate {
    fn from(item: Nameplate) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            project_id: item.project_id,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::Nameplate> for Nameplate {
    fn from(item: web_common::database::tables::Nameplate) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            project_id: item.project_id,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Nameplate {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_view_nameplates(
            author_user_id,
            id,
        ))
        .get_result(connection)
        .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&NameplateFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_view_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::id);
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&NameplateFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_view_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::nameplates;
        nameplates::dsl::nameplates
            .filter(nameplates::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its barcode.
    ///
    /// * `barcode` - The barcode of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn from_barcode(
        barcode: &str,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let flat_variant = nameplates::dsl::nameplates
            .filter(nameplates::dsl::barcode.eq(barcode))
            .first::<Self>(connection)?;
        if !flat_variant.can_view(author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&NameplateFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::nameplates;
        let mut query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_view_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .filter(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ),
            )
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_with_score_viewable(
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<(Self, f32)>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        nameplates::dsl::nameplates
            .select((
                Nameplate::as_select(),
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ),
            ))
            .filter(crate::sql_function_bindings::can_view_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .filter(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<(Self, f32)>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_update_nameplates(
            author_user_id,
            id,
        ))
        .get_result(connection)
        .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_update_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::id);
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_updatable_sorted(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_update_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_updatable(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::nameplates;
        let mut query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_update_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .filter(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ),
            )
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(self.id, author_user_id, connection)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `id` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_admin_nameplates(
            author_user_id,
            id,
        ))
        .get_result(connection)
        .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_admin_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::id);
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_administrable_sorted(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::nameplates;
        let query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_admin_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .order_by(nameplates::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_administrable(
        filter: Option<&NameplateFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::nameplates;
        let mut query = nameplates::dsl::nameplates
            .select(Nameplate::as_select())
            .filter(crate::sql_function_bindings::can_admin_nameplates(
                author_user_id,
                nameplates::dsl::id,
            ))
            .filter(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ),
            )
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(nameplates::dsl::project_id.eq(project_id));
        }
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(nameplates::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(nameplates::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(nameplates::dsl::updated_by.eq(updated_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete(
        &self,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        Self::delete_by_id(self.id, author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(nameplates::dsl::nameplates.filter(nameplates::dsl::id.eq(id)))
            .execute(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
