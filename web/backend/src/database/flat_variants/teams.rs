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
use web_common::database::filter_structs::*;

#[derive(
    Eq,
    PartialEq,
    PartialOrd,
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
#[diesel(table_name = crate::database::schema::teams)]
#[diesel(primary_key(id))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub state_id: i32,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for Team {}
unsafe impl Sync for Team {}
impl From<Team> for web_common::database::flat_variants::Team {
    fn from(item: Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            state_id: item.state_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::flat_variants::Team> for Team {
    fn from(item: web_common::database::flat_variants::Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            state_id: item.state_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Team {
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
        filter: Option<&TeamFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .order_by(teams::dsl::id);
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        filter: Option<&TeamFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .order_by(teams::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        use crate::database::schema::teams;
        teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its name.
    ///
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_name(
        name: &str,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let flat_variant = teams::dsl::teams
            .filter(teams::dsl::name.eq(name))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn strict_word_similarity_search_viewable(
        filter: Option<&TeamFilter>,
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
        use crate::database::schema::teams;
        let mut query = teams::dsl::teams
            .select(Team::as_select())
            .filter(
                crate::database::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        use crate::database::schema::teams;
        teams::dsl::teams
            .select((
                Team::as_select(),
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::database::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
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
        diesel::select(crate::database::sql_function_bindings::can_update_teams(
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
        filter: Option<&TeamFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_update_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .order_by(teams::dsl::id);
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        filter: Option<&TeamFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_update_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .order_by(teams::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        filter: Option<&TeamFilter>,
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
        use crate::database::schema::teams;
        let mut query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_update_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .filter(
                crate::database::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        diesel::select(crate::database::sql_function_bindings::can_admin_teams(
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
        filter: Option<&TeamFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .order_by(teams::dsl::id);
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        filter: Option<&TeamFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::database::schema::teams;
        let query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .order_by(teams::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
        filter: Option<&TeamFilter>,
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
        use crate::database::schema::teams;
        let mut query = teams::dsl::teams
            .select(Team::as_select())
            .filter(crate::database::sql_function_bindings::can_admin_teams(
                author_user_id,
                teams::dsl::id,
            ))
            .filter(
                crate::database::sql_function_bindings::concat_teams_name_description(
                    teams::dsl::name,
                    teams::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::database::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::database::sql_function_bindings::concat_teams_name_description(
                        teams::dsl::name,
                        teams::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(teams::dsl::state_id.eq(state_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
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
            crate::database::teams::dsl::teams.filter(crate::database::teams::dsl::id.eq(id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
