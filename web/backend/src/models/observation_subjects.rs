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
#[diesel(table_name = observation_subjects)]
#[diesel(belongs_to(crate::models::font_awesome_icons::FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(crate::models::colors::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct ObservationSubject {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<ObservationSubject> for web_common::database::tables::ObservationSubject {
    fn from(item: ObservationSubject) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::ObservationSubject> for ObservationSubject {
    fn from(item: web_common::database::tables::ObservationSubject) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl ObservationSubject {
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
        filter: Option<&ObservationSubjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observation_subjects;
        let mut query = observation_subjects::dsl::observation_subjects.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(observation_subjects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(observation_subjects::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&ObservationSubjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observation_subjects;
        let mut query = observation_subjects::dsl::observation_subjects.into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(observation_subjects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(observation_subjects::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        use crate::schema::observation_subjects;
        observation_subjects::dsl::observation_subjects
            .filter(observation_subjects::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        filter: Option<&ObservationSubjectFilter>,
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
        use crate::schema::observation_subjects;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::icon_id.eq(icon_id))
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(crate::sql_function_bindings::similarity_dist(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::color_id.eq(color_id))
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(crate::sql_function_bindings::similarity_dist(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        observation_subjects::dsl::observation_subjects
            .filter(
                crate::sql_function_bindings::similarity_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(crate::sql_function_bindings::similarity_dist(
                crate::sql_function_bindings::concat_observation_subjects_name_description(
                    observation_subjects::dsl::name,
                    observation_subjects::dsl::description,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_viewable(
        filter: Option<&ObservationSubjectFilter>,
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
        use crate::schema::observation_subjects;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::icon_id.eq(icon_id))
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(crate::sql_function_bindings::word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::color_id.eq(color_id))
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(crate::sql_function_bindings::word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ))
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        observation_subjects::dsl::observation_subjects
            .filter(
                crate::sql_function_bindings::word_similarity_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(crate::sql_function_bindings::word_similarity_dist_op(
                crate::sql_function_bindings::concat_observation_subjects_name_description(
                    observation_subjects::dsl::name,
                    observation_subjects::dsl::description,
                ),
                query,
            ))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
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
        filter: Option<&ObservationSubjectFilter>,
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
        use crate::schema::observation_subjects;
        if filter
            .map(|f| f.icon_id.is_some() && f.color_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::icon_id.eq(icon_id))
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            return observation_subjects::dsl::observation_subjects
                .filter(observation_subjects::dsl::color_id.eq(color_id))
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        observation_subjects::dsl::observation_subjects
            .filter(
                crate::sql_function_bindings::strict_word_similarity_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
