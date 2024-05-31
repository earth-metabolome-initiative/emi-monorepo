//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use crate::schema::*;
use crate::sql_function_bindings::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::Identifiable;
use diesel::Insertable;
use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
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
#[diesel(table_name = spectra_collections)]
#[diesel(belongs_to(crate::models::samples::Sample, foreign_key = sample_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SpectraCollection {
    pub id: i32,
    pub notes: Option<String>,
    pub sample_id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<SpectraCollection> for web_common::database::tables::SpectraCollection {
    fn from(item: SpectraCollection) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            sample_id: item.sample_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::SpectraCollection> for SpectraCollection {
    fn from(item: web_common::database::tables::SpectraCollection) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            sample_id: item.sample_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl SpectraCollection {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_view_spectra_collections(author_user_id, id))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_view_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::spectra_collections;
        spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_view_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                similarity_op(sample_containers::dsl::barcode, query)
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(sample_containers::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + similarity_dist(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_viewable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_view_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_view_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_view_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(strict_word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + strict_word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_update_spectra_collections(author_user_id, id))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_update_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_updatable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_update_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                similarity_op(sample_containers::dsl::barcode, query)
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(sample_containers::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + similarity_dist(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_updatable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_update_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_update_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_update_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(strict_word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + strict_word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_admin_spectra_collections(author_user_id, id))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections.into_boxed();
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(sample_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(updated_by));
        }
        query
            .filter(can_admin_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_administrable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    similarity_op(sample_containers::dsl::barcode, query)
                        .or(similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    similarity_dist(sample_containers::dsl::barcode, query)
                        + similarity_dist(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + similarity_dist(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_admin_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                similarity_op(sample_containers::dsl::barcode, query)
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(sample_containers::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + similarity_dist(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn word_similarity_search_administrable(
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_admin_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
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
        filter: Option<&SpectraCollectionFilter>,
        author_user_id: i32,
        query: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::spectra_collections;
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        if filter
            .map(|f| f.sample_id.is_some() && f.created_by.is_some() && f.updated_by.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::sample_id.eq(sample_id))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::created_by.eq(created_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            return spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::updated_by.eq(updated_by))
                .select(SpectraCollection::as_select())
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples0
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples0.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers::dsl::sample_containers.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples1
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples1.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples2
                        .on(spectra_collections::dsl::sample_id
                            .eq(samples2.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states::dsl::sample_states.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states::dsl::id)),
                )
                .filter(can_admin_spectra_collections(
                    author_user_id,
                    spectra_collections::dsl::id,
                ))
                .filter(
                    strict_word_similarity_op(sample_containers::dsl::barcode, query)
                        .or(strict_word_similarity_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        ))
                        .or(strict_word_similarity_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        )),
                )
                .order(
                    strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                        + strict_word_similarity_dist_op(
                            concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        + strict_word_similarity_dist_op(
                            concat_sample_states_name_description(
                                sample_states::dsl::name,
                                sample_states::dsl::description,
                            ),
                            query,
                        ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        spectra_collections::dsl::spectra_collections
            .select(SpectraCollection::as_select())
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(spectra_collections::dsl::sample_id.eq(samples0.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(spectra_collections::dsl::sample_id.eq(samples1.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(spectra_collections::dsl::sample_id.eq(samples2.field(samples::dsl::id))),
            )
            // This operation is defined by a second order index linking spectra_collections.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .filter(can_admin_spectra_collections(
                author_user_id,
                spectra_collections::dsl::id,
            ))
            .filter(
                strict_word_similarity_op(sample_containers::dsl::barcode, query)
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ))
                    .or(strict_word_similarity_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    + strict_word_similarity_dist_op(
                        concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        ),
                        query,
                    ),
            )
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
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
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            spectra_collections::dsl::spectra_collections
                .filter(spectra_collections::dsl::id.eq(id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
