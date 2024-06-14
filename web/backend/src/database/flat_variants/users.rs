//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::database::*;
use diesel::prelude::*;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;

#[derive(
    PartialEq,
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Identifiable,
    QueryableByName,
    Queryable,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = crate::database::schema::users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub organization_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub picture: crate::database::diesel_types::JPEG,
}

unsafe impl Send for User {}
unsafe impl Sync for User {}
impl From<web_common::database::flat_variants::User> for crate::database::flat_variants::User {
    fn from(item: web_common::database::flat_variants::User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            organization_id: item.organization_id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            picture: item.picture.convert(),
        }
    }
}

impl From<crate::database::flat_variants::User> for web_common::database::flat_variants::User {
    fn from(item: crate::database::flat_variants::User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            organization_id: item.organization_id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            picture: item.picture.convert(),
        }
    }
}

impl User {
    /// Check whether the user can view the struct.
    pub fn can_view(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    pub fn can_view_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(true)
    }
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable(
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .order_by(users::dsl::id);
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn all_viewable_sorted(
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .order_by(users::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
    /// * `connection` - The connection to the database.
    pub fn get(
        id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::database::schema::users;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&web_common::database::filter_variants::UserFilter>,
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
            return Self::all_viewable(filter, limit, offset, connection);
        }
        use crate::database::schema::users;
        let mut query = users::dsl::users
            .select(User::as_select())
            .filter(
                crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .ilike(format!("%{}%", query))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_users_name(
                        users::dsl::first_name,
                        users::dsl::last_name,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_with_score_viewable(
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<(Self, f32)>, web_common::api::ApiError> {
        use crate::database::schema::users;
        users::dsl::users
            .select((
                User::as_select(),
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_users_name(
                        users::dsl::first_name,
                        users::dsl::last_name,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .ilike(format!("%{}%", query))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_users_name(
                        users::dsl::first_name,
                        users::dsl::last_name,
                    ),
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
        diesel::select(crate::database::sql_function_bindings::can_update_users(
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_update_users(
                author_user_id,
                users::dsl::id,
            ))
            .order_by(users::dsl::id);
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_update_users(
                author_user_id,
                users::dsl::id,
            ))
            .order_by(users::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
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
        use crate::database::schema::users;
        let mut query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_update_users(
                author_user_id,
                users::dsl::id,
            ))
            .filter(
                crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .ilike(format!("%{}%", query))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_users_name(
                        users::dsl::first_name,
                        users::dsl::last_name,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        diesel::select(crate::database::sql_function_bindings::can_admin_users(
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_users(
                author_user_id,
                users::dsl::id,
            ))
            .order_by(users::dsl::id);
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::users;
        let query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_users(
                author_user_id,
                users::dsl::id,
            ))
            .order_by(users::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        filter: Option<&web_common::database::filter_variants::UserFilter>,
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
        use crate::database::schema::users;
        let mut query = users::dsl::users
            .select(User::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_users(
                author_user_id,
                users::dsl::id,
            ))
            .filter(
                crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_users_name(
                    users::dsl::first_name,
                    users::dsl::last_name,
                )
                .ilike(format!("%{}%", query))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_users_name(
                        users::dsl::first_name,
                        users::dsl::last_name,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(organization_id) = filter.and_then(|f| f.organization_id) {
            query = query.filter(users::dsl::organization_id.eq(organization_id));
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
        diesel::delete(
            crate::database::users::dsl::users.filter(crate::database::users::dsl::id.eq(id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}