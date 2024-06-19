use crate::database::*;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NestedProjectsUsersRoleInvitation {
    pub inner: crate::database::flat_variants::ProjectsUsersRoleInvitation,
    pub table: crate::database::nested_variants::NestedProject,
    pub user: crate::database::nested_variants::NestedUser,
    pub role: crate::database::nested_variants::NestedRole,
    pub created_by: crate::database::nested_variants::NestedUser,
}

unsafe impl Send for NestedProjectsUsersRoleInvitation {}
unsafe impl Sync for NestedProjectsUsersRoleInvitation {}
impl NestedProjectsUsersRoleInvitation {
    /// Convert the flat struct to the nested struct.
    ///
    /// # Arguments
    /// * `flat_variant` - The flat struct.
    /// * `author_user_id` - The author user id.
    /// * `connection` - The database connection.
    pub(crate) fn from_flat(
        flat_variant: ProjectsUsersRoleInvitation,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        Ok(Self {
            table: crate::database::nested_variants::NestedProject::get(
                flat_variant.table_id,
                author_user_id,
                connection,
            )?,
            user: crate::database::nested_variants::NestedUser::get(
                flat_variant.user_id,
                connection,
            )?,
            role: crate::database::nested_variants::NestedRole::get(
                flat_variant.role_id,
                connection,
            )?,
            created_by: crate::database::nested_variants::NestedUser::get(
                flat_variant.created_by,
                connection,
            )?,
            inner: flat_variant,
        })
    }
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
        self.inner.can_view(author_user_id, connection)
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
        ProjectsUsersRoleInvitation::can_view_by_id((table_id, user_id), author_user_id, connection)
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
        ProjectsUsersRoleInvitation::all_viewable(
            filter,
            author_user_id,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
        .collect()
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
        ProjectsUsersRoleInvitation::all_viewable_sorted(
            filter,
            author_user_id,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
        .collect()
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
        ProjectsUsersRoleInvitation::get((table_id, user_id), author_user_id, connection)
            .and_then(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
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
        ProjectsUsersRoleInvitation::strict_word_similarity_search_viewable(
            filter,
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, author_user_id, connection))
        .collect()
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
        ProjectsUsersRoleInvitation::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|(flat_variant, score)| {
            Ok((
                Self::from_flat(flat_variant, author_user_id, connection)?,
                score,
            ))
        })
        .collect()
    }
    /// Check whether the user can update the struct.
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        self.inner.can_update()
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        ProjectsUsersRoleInvitation::can_update_by_id()
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
        self.inner.can_admin(author_user_id, connection)
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
        ProjectsUsersRoleInvitation::can_admin_by_id(
            (table_id, user_id),
            author_user_id,
            connection,
        )
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
        ProjectsUsersRoleInvitation::all_administrable(
            filter,
            author_user_id,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
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
        ProjectsUsersRoleInvitation::all_administrable_sorted(
            filter,
            author_user_id,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
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
        ProjectsUsersRoleInvitation::strict_word_similarity_search_administrable(
            filter,
            author_user_id,
            query,
            limit,
            offset,
            connection,
        )?
        .into_iter()
        .map(|flat_variant| Self::from_flat(flat_variant, Some(author_user_id), connection))
        .collect()
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
        self.inner.delete(author_user_id, connection)
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
        ProjectsUsersRoleInvitation::delete_by_id((table_id, user_id), author_user_id, connection)
    }
}
impl From<web_common::database::nested_variants::NestedProjectsUsersRoleInvitation>
    for crate::database::nested_variants::NestedProjectsUsersRoleInvitation
{
    fn from(
        item: web_common::database::nested_variants::NestedProjectsUsersRoleInvitation,
    ) -> Self {
        Self {
            inner: crate::database::flat_variants::ProjectsUsersRoleInvitation::from(item.inner),
            table: crate::database::nested_variants::NestedProject::from(
                item.table.as_ref().clone(),
            ),
            user: crate::database::nested_variants::NestedUser::from(item.user.as_ref().clone()),
            role: crate::database::nested_variants::NestedRole::from(item.role.as_ref().clone()),
            created_by: crate::database::nested_variants::NestedUser::from(
                item.created_by.as_ref().clone(),
            ),
        }
    }
}
impl From<crate::database::nested_variants::NestedProjectsUsersRoleInvitation>
    for web_common::database::nested_variants::NestedProjectsUsersRoleInvitation
{
    fn from(item: crate::database::nested_variants::NestedProjectsUsersRoleInvitation) -> Self {
        Self {
            inner: web_common::database::flat_variants::ProjectsUsersRoleInvitation::from(
                item.inner,
            ),
            table: Rc::from(web_common::database::nested_variants::NestedProject::from(
                item.table,
            )),
            user: Rc::from(web_common::database::nested_variants::NestedUser::from(
                item.user,
            )),
            role: Rc::from(web_common::database::nested_variants::NestedRole::from(
                item.role,
            )),
            created_by: Rc::from(web_common::database::nested_variants::NestedUser::from(
                item.created_by,
            )),
        }
    }
}
