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
#[diesel(table_name = observations)]
#[diesel(belongs_to(crate::models::observations::Observation, foreign_key = parent_observation_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::models::projects::Project, foreign_key = project_id))]
#[diesel(belongs_to(crate::models::organisms::Organism, foreign_key = organism_id))]
#[diesel(belongs_to(crate::models::samples::Sample, foreign_key = sample_id))]
#[diesel(belongs_to(crate::models::observation_subjects::ObservationSubject, foreign_key = subject_id))]
#[diesel(primary_key(id))]
pub struct Observation {
    pub id: uuid::Uuid,
    pub parent_observation_id: Option<uuid::Uuid>,
    pub created_by: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub project_id: i32,
    pub organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub subject_id: i32,
    pub notes: Option<String>,
    pub picture: Vec<u8>,
}

impl From<Observation> for web_common::database::tables::Observation {
    fn from(item: Observation) -> Self {
        Self {
            id: item.id,
            parent_observation_id: item.parent_observation_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            organism_id: item.organism_id,
            sample_id: item.sample_id,
            subject_id: item.subject_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::Observation> for Observation {
    fn from(item: web_common::database::tables::Observation) -> Self {
        Self {
            id: item.id,
            parent_observation_id: item.parent_observation_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            organism_id: item.organism_id,
            sample_id: item.sample_id,
            subject_id: item.subject_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl Observation {
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
        id: uuid::Uuid,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_view_observations(
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
        filter: Option<&ObservationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_view_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::id);
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        filter: Option<&ObservationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_view_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        id: uuid::Uuid,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::observations;
        observations::dsl::observations
            .filter(observations::dsl::id.eq(id))
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
        filter: Option<&ObservationFilter>,
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
        use crate::schema::observations;
        let (projects0, projects1, projects2) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1,
            crate::schema::projects as projects2
        );
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = observations::dsl::observations
            // This operation is defined by a first order index linking observations.project_id to projects.
            .inner_join(
                projects0.on(observations::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a first order index linking observations.subject_id to observation_subjects.
            .inner_join(
                observation_subjects::dsl::observation_subjects
                    .on(observations::dsl::subject_id.eq(observation_subjects::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(organisms0.on(
                observations::dsl::organism_id.eq(organisms0.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(organisms1.on(
                observations::dsl::organism_id.eq(organisms1.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects1.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(observations::dsl::sample_id
                        .eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(observations::dsl::sample_id
                        .eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects2.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects2.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(observations::dsl::sample_id
                        .eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_view_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .filter(
                crate::sql_function_bindings::concat_projects_name_description(
                    projects0.field(projects::dsl::name),
                    projects0.field(projects::dsl::description),
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(sample_containers::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects2.field(projects::dsl::name),
                            projects2.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        use crate::schema::observations;
        let (projects0, projects1, projects2) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1,
            crate::schema::projects as projects2
        );
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        observations::dsl::observations
            // This operation is defined by a first order index linking observations.project_id to projects.
            .inner_join(
                projects0.on(observations::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a first order index linking observations.subject_id to observation_subjects.
            .inner_join(
                observation_subjects::dsl::observation_subjects
                    .on(observations::dsl::subject_id.eq(observation_subjects::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(organisms0.on(
                observations::dsl::organism_id.eq(organisms0.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(organisms1.on(
                observations::dsl::organism_id.eq(organisms1.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects1.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(observations::dsl::sample_id
                        .eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(observations::dsl::sample_id
                        .eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects2.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects2.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(observations::dsl::sample_id
                        .eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select((
                Observation::as_select(),
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    ),
                    query,
                ),
            ))
            .filter(crate::sql_function_bindings::can_view_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .filter(
                crate::sql_function_bindings::concat_projects_name_description(
                    projects0.field(projects::dsl::name),
                    projects0.field(projects::dsl::description),
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(sample_containers::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects2.field(projects::dsl::name),
                            projects2.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
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
        id: uuid::Uuid,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_update_observations(
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
        filter: Option<&ObservationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_update_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::id);
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        filter: Option<&ObservationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_update_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        filter: Option<&ObservationFilter>,
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
        use crate::schema::observations;
        let (projects0, projects1, projects2) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1,
            crate::schema::projects as projects2
        );
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = observations::dsl::observations
            // This operation is defined by a first order index linking observations.project_id to projects.
            .inner_join(
                projects0.on(observations::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a first order index linking observations.subject_id to observation_subjects.
            .inner_join(
                observation_subjects::dsl::observation_subjects
                    .on(observations::dsl::subject_id.eq(observation_subjects::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(organisms0.on(
                observations::dsl::organism_id.eq(organisms0.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(organisms1.on(
                observations::dsl::organism_id.eq(organisms1.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects1.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(observations::dsl::sample_id
                        .eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(observations::dsl::sample_id
                        .eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects2.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects2.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(observations::dsl::sample_id
                        .eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_update_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .filter(
                crate::sql_function_bindings::concat_projects_name_description(
                    projects0.field(projects::dsl::name),
                    projects0.field(projects::dsl::description),
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(sample_containers::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects2.field(projects::dsl::name),
                            projects2.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        id: uuid::Uuid,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_admin_observations(
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
        filter: Option<&ObservationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_admin_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::id);
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        filter: Option<&ObservationFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::observations;
        let query = observations::dsl::observations
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_admin_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .order_by(observations::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        filter: Option<&ObservationFilter>,
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
        use crate::schema::observations;
        let (projects0, projects1, projects2) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1,
            crate::schema::projects as projects2
        );
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = observations::dsl::observations
            // This operation is defined by a first order index linking observations.project_id to projects.
            .inner_join(
                projects0.on(observations::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a first order index linking observations.subject_id to observation_subjects.
            .inner_join(
                observation_subjects::dsl::observation_subjects
                    .on(observations::dsl::subject_id.eq(observation_subjects::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(organisms0.on(
                observations::dsl::organism_id.eq(organisms0.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(organisms1.on(
                observations::dsl::organism_id.eq(organisms1.field(organisms::dsl::id).nullable()),
            ))
            // This operation is defined by a second order index linking observations.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects1.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                samples0
                    .on(observations::dsl::sample_id
                        .eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                samples1
                    .on(observations::dsl::sample_id
                        .eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects2.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects2.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                samples2
                    .on(observations::dsl::sample_id
                        .eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking observations.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Observation::as_select())
            .filter(crate::sql_function_bindings::can_admin_observations(
                author_user_id,
                observations::dsl::id,
            ))
            .filter(
                crate::sql_function_bindings::concat_projects_name_description(
                    projects0.field(projects::dsl::name),
                    projects0.field(projects::dsl::description),
                )
                .strict_word_similarity_commutator_op(query)
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    )
                    .ilike(format!("%{}%", query)),
                )
                .or(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_observation_subjects_name_description(
                            observation_subjects::dsl::name,
                            observation_subjects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(sample_containers::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects2.field(projects::dsl::name),
                            projects2.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .or(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    )
                    .strict_word_similarity_commutator_op(query)
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states::dsl::name,
                            sample_states::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                ),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_observation_subjects_name_description(
                        observation_subjects::dsl::name,
                        observation_subjects::dsl::description,
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects2.field(projects::dsl::name),
                        projects2.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_sample_states_name_description(
                        sample_states::dsl::name,
                        sample_states::dsl::description,
                    ),
                    query,
                ),
            )
            .into_boxed();
        if let Some(parent_observation_id) = filter.and_then(|f| f.parent_observation_id) {
            query =
                query.filter(observations::dsl::parent_observation_id.eq(parent_observation_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(observations::dsl::organism_id.eq(organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(observations::dsl::sample_id.eq(sample_id));
        }
        if let Some(subject_id) = filter.and_then(|f| f.subject_id) {
            query = query.filter(observations::dsl::subject_id.eq(subject_id));
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
        id: uuid::Uuid,
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(id, author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(observations::dsl::observations.filter(observations::dsl::id.eq(id)))
            .execute(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
