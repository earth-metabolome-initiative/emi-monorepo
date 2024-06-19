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
    Eq,
    PartialEq,
    PartialOrd,
    Debug,
    Clone,
    Copy,
    Ord,
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
#[diesel(table_name = crate::database::schema::projects_users_role_invitations)]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsUsersRoleInvitation {}
unsafe impl Sync for ProjectsUsersRoleInvitation {}
impl From<web_common::database::flat_variants::ProjectsUsersRoleInvitation>
    for crate::database::flat_variants::ProjectsUsersRoleInvitation
{
    fn from(item: web_common::database::flat_variants::ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<crate::database::flat_variants::ProjectsUsersRoleInvitation>
    for web_common::database::flat_variants::ProjectsUsersRoleInvitation
{
    fn from(item: crate::database::flat_variants::ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRoleInvitation {
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
            crate::database::sql_function_bindings::can_view_projects_users_role_invitations(
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_invitations;
        let query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_view_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .order_by(projects_users_role_invitations::dsl::table_id);
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_invitations;
        let query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_view_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .order_by(projects_users_role_invitations::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
        use crate::database::schema::projects_users_role_invitations;
        projects_users_role_invitations::dsl::projects_users_role_invitations
            .filter(projects_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_users_role_invitations::dsl::user_id.eq(user_id))
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
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
        use crate::database::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            // This operation is defined by a first order index linking projects_users_role_invitations.table_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(projects_users_role_invitations::dsl::table_id.eq(projects::dsl::id)),
            )
            // This operation is defined by a first order index linking projects_users_role_invitations.role_id to roles.
            .inner_join(
                roles::dsl::roles
                    .on(projects_users_role_invitations::dsl::role_id.eq(roles::dsl::id)),
            )
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_view_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .filter(
                crate::database::sql_function_bindings::concat_projects_name_description(
                    projects::dsl::name,
                    projects::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                ) + crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
        use crate::database::schema::projects_users_role_invitations;
        projects_users_role_invitations::dsl::projects_users_role_invitations
            // This operation is defined by a first order index linking projects_users_role_invitations.table_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(projects_users_role_invitations::dsl::table_id.eq(projects::dsl::id)),
            )
            // This operation is defined by a first order index linking projects_users_role_invitations.role_id to roles.
            .inner_join(
                roles::dsl::roles
                    .on(projects_users_role_invitations::dsl::role_id.eq(roles::dsl::id)),
            )
            .select((
                ProjectsUsersRoleInvitation::as_select(),
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                ) + crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::database::sql_function_bindings::can_view_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .filter(
                crate::database::sql_function_bindings::concat_projects_name_description(
                    projects::dsl::name,
                    projects::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                ) + crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<(Self, f32)>(connection)
            .map(|mut entries| {
                entries.iter_mut().for_each(|entry| {
                    entry.1 /= 2.0;
                });
                entries
            })
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
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
            crate::database::sql_function_bindings::can_admin_projects_users_role_invitations(
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_invitations;
        let query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_admin_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .order_by(projects_users_role_invitations::dsl::table_id);
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::projects_users_role_invitations;
        let query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_admin_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .order_by(projects_users_role_invitations::dsl::created_at.desc());
        let mut query = query.into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
        filter: Option<&web_common::database::filter_variants::ProjectsUsersRoleInvitationFilter>,
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
        use crate::database::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            // This operation is defined by a first order index linking projects_users_role_invitations.table_id to projects.
            .inner_join(
                projects::dsl::projects
                    .on(projects_users_role_invitations::dsl::table_id.eq(projects::dsl::id)),
            )
            // This operation is defined by a first order index linking projects_users_role_invitations.role_id to roles.
            .inner_join(
                roles::dsl::roles
                    .on(projects_users_role_invitations::dsl::role_id.eq(roles::dsl::id)),
            )
            .select(ProjectsUsersRoleInvitation::as_select())
            .filter(
                crate::database::sql_function_bindings::can_admin_projects_users_role_invitations(
                    author_user_id,
                    projects_users_role_invitations::dsl::table_id,
                    projects_users_role_invitations::dsl::user_id,
                ),
            )
            .filter(
                crate::database::sql_function_bindings::concat_projects_name_description(
                    projects::dsl::name,
                    projects::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(crate::database::sql_function_bindings::concat_roles_name(
                    roles::dsl::name,
                    roles::dsl::description,
                )
                .ilike(format!("%{}%", query)))),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                ) + crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_roles_name(
                        roles::dsl::name,
                        roles::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
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
            crate::database::projects_users_role_invitations::dsl::projects_users_role_invitations
                .filter(
                    crate::database::projects_users_role_invitations::dsl::table_id.eq(table_id),
                )
                .filter(crate::database::projects_users_role_invitations::dsl::user_id.eq(user_id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
