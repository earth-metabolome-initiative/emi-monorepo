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
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = project_states)]
#[diesel(belongs_to(crate::models::font_awesome_icons::FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(crate::models::colors::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<ProjectState> for web_common::database::tables::ProjectState {
    fn from(item: ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::ProjectState> for ProjectState {
    fn from(item: web_common::database::tables::ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl ProjectState {
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
        filter: Option<&ProjectStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::project_states;
        let query = project_states::dsl::project_states
            .select(ProjectState::as_select())
            .order_by(project_states::dsl::id);
        let mut query = query.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(project_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(project_states::dsl::color_id.eq(color_id));
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
        filter: Option<&ProjectStateFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        Self::all_viewable(filter, limit, offset, connection)
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
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
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
        use crate::schema::project_states;
        let flat_variant = project_states::dsl::project_states
            .filter(project_states::dsl::name.eq(name))
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
        filter: Option<&ProjectStateFilter>,
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
        use crate::schema::project_states;
        let mut query = project_states::dsl::project_states
            .select(ProjectState::as_select())
            .filter(
                crate::sql_function_bindings::concat_project_states_name_description(
                    project_states::dsl::name,
                    project_states::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_project_states_name_description(
                        project_states::dsl::name,
                        project_states::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_project_states_name_description(
                        project_states::dsl::name,
                        project_states::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(project_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(project_states::dsl::color_id.eq(color_id));
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
        use crate::schema::project_states;
        project_states::dsl::project_states
            .select((
                ProjectState::as_select(),
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_project_states_name_description(
                        project_states::dsl::name,
                        project_states::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(
                crate::sql_function_bindings::concat_project_states_name_description(
                    project_states::dsl::name,
                    project_states::dsl::description,
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_project_states_name_description(
                        project_states::dsl::name,
                        project_states::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_project_states_name_description(
                        project_states::dsl::name,
                        project_states::dsl::description,
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
    pub fn can_update(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can update the struct associated to the provided ids.
    pub fn can_update_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can admin the struct.
    pub fn can_admin(&self) -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    pub fn can_admin_by_id() -> Result<bool, web_common::api::ApiError> {
        Ok(false)
    }
}
