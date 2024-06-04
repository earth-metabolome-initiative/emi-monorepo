//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::schema::*;
use crate::sql_operator_bindings::HasStrictWordSimilarityCommutatorOp;
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
    Copy,
    Ord,
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = teams_users_role_requests)]
#[diesel(belongs_to(crate::models::teams::Team, foreign_key = table_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = user_id))]
#[diesel(belongs_to(crate::models::roles::Role, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl From<TeamsUsersRoleRequest> for web_common::database::tables::TeamsUsersRoleRequest {
    fn from(item: TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRoleRequest> for TeamsUsersRoleRequest {
    fn from(item: web_common::database::tables::TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRoleRequest {
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
        Self::can_view_by_id((self.table_id, self.user_id), author_user_id, connection)
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        (table_id, user_id): (i32, i32),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_view_teams_users_role_requests(
                author_user_id,
                table_id,
                user_id,
            ),
        )
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_view_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::table_id);
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_view_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        (table_id, user_id): (i32, i32),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id((table_id, user_id), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::teams_users_role_requests;
        teams_users_role_requests::dsl::teams_users_role_requests
            .filter(teams_users_role_requests::dsl::table_id.eq(table_id))
            .filter(teams_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
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
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            // This operation is defined by a first order index linking teams_users_role_requests.table_id to teams.
            .inner_join(
                teams::dsl::teams.on(teams_users_role_requests::dsl::table_id.eq(teams::dsl::id)),
            )
            // This operation is defined by a first order index linking teams_users_role_requests.role_id to roles.
            .inner_join(
                roles::dsl::roles.on(teams_users_role_requests::dsl::role_id.eq(roles::dsl::id)),
            )
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_view_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        use crate::schema::teams_users_role_requests;
        teams_users_role_requests::dsl::teams_users_role_requests
            // This operation is defined by a first order index linking teams_users_role_requests.table_id to teams.
            .inner_join(
                teams::dsl::teams.on(teams_users_role_requests::dsl::table_id.eq(teams::dsl::id)),
            )
            // This operation is defined by a first order index linking teams_users_role_requests.role_id to roles.
            .inner_join(
                roles::dsl::roles.on(teams_users_role_requests::dsl::role_id.eq(roles::dsl::id)),
            )
            .select((
                TeamsUsersRoleRequest::as_select(),
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::sql_function_bindings::can_view_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
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
        Self::can_update_by_id((self.table_id, self.user_id), author_user_id, connection)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        (table_id, user_id): (i32, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_update_teams_users_role_requests(
                author_user_id,
                table_id,
                user_id,
            ),
        )
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_update_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::table_id);
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_update_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
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
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            // This operation is defined by a first order index linking teams_users_role_requests.table_id to teams.
            .inner_join(
                teams::dsl::teams.on(teams_users_role_requests::dsl::table_id.eq(teams::dsl::id)),
            )
            // This operation is defined by a first order index linking teams_users_role_requests.role_id to roles.
            .inner_join(
                roles::dsl::roles.on(teams_users_role_requests::dsl::role_id.eq(roles::dsl::id)),
            )
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_update_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        Self::can_admin_by_id((self.table_id, self.user_id), author_user_id, connection)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        (table_id, user_id): (i32, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_admin_teams_users_role_requests(
                author_user_id,
                table_id,
                user_id,
            ),
        )
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_admin_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::table_id);
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::teams_users_role_requests;
        let query = teams_users_role_requests::dsl::teams_users_role_requests
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_admin_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .order_by(teams_users_role_requests::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        filter: Option<&TeamsUsersRoleRequestFilter>,
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
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            // This operation is defined by a first order index linking teams_users_role_requests.table_id to teams.
            .inner_join(
                teams::dsl::teams.on(teams_users_role_requests::dsl::table_id.eq(teams::dsl::id)),
            )
            // This operation is defined by a first order index linking teams_users_role_requests.role_id to roles.
            .inner_join(
                roles::dsl::roles.on(teams_users_role_requests::dsl::role_id.eq(roles::dsl::id)),
            )
            .select(TeamsUsersRoleRequest::as_select())
            .filter(
                crate::sql_function_bindings::can_admin_teams_users_role_requests(
                    author_user_id,
                    teams_users_role_requests::dsl::table_id,
                    teams_users_role_requests::dsl::user_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
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
        Self::delete_by_id((self.table_id, self.user_id), author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `( table_id, user_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        (table_id, user_id): (i32, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id((table_id, user_id), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            teams_users_role_requests::dsl::teams_users_role_requests
                .filter(teams_users_role_requests::dsl::table_id.eq(table_id))
                .filter(teams_users_role_requests::dsl::user_id.eq(user_id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
