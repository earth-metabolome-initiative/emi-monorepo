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
#[diesel(table_name = organisms)]
#[diesel(belongs_to(crate::models::organisms::Organism, foreign_key = host_organism_id))]
#[diesel(belongs_to(crate::models::samples::Sample, foreign_key = sample_id))]
#[diesel(belongs_to(crate::models::nameplates::Nameplate, foreign_key = nameplate_id))]
#[diesel(belongs_to(crate::models::projects::Project, foreign_key = project_id))]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Organism {
    pub id: uuid::Uuid,
    pub host_organism_id: Option<uuid::Uuid>,
    pub sample_id: Option<uuid::Uuid>,
    pub notes: Option<String>,
    pub nameplate_id: i32,
    pub project_id: i32,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub picture: Vec<u8>,
}

unsafe impl Send for Organism {}
unsafe impl Sync for Organism {}
impl From<Organism> for web_common::database::tables::Organism {
    fn from(item: Organism) -> Self {
        Self {
            id: item.id,
            host_organism_id: item.host_organism_id,
            sample_id: item.sample_id,
            notes: item.notes,
            nameplate_id: item.nameplate_id,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::Organism> for Organism {
    fn from(item: web_common::database::tables::Organism) -> Self {
        Self {
            id: item.id,
            host_organism_id: item.host_organism_id,
            sample_id: item.sample_id,
            notes: item.notes,
            nameplate_id: item.nameplate_id,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl Organism {
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
        diesel::select(crate::sql_function_bindings::can_view_organisms(
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
        filter: Option<&OrganismFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_view_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::id);
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        filter: Option<&OrganismFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_view_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        use crate::schema::organisms;
        organisms::dsl::organisms
            .filter(organisms::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its nameplate_id.
    ///
    /// * `nameplate_id` - The nameplate_id of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn from_nameplate_id(
        nameplate_id: &i32,
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::organisms;
        let flat_variant = organisms::dsl::organisms
            .filter(organisms::dsl::nameplate_id.eq(nameplate_id))
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
        filter: Option<&OrganismFilter>,
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
        use crate::schema::organisms;
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = organisms::dsl::organisms
            // This operation is defined by a first order index linking organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates
                    .on(organisms::dsl::nameplate_id.eq(nameplates::dsl::id)),
            )
            // This operation is defined by a first order index linking organisms.project_id to projects.
            .inner_join(
                projects0.on(organisms::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .left_join(
                samples0
                    .on(organisms::dsl::sample_id.eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .left_join(
                samples1
                    .on(organisms::dsl::sample_id.eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects1.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .left_join(
                samples2
                    .on(organisms::dsl::sample_id.eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_view_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .filter(
                nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
                    .or(
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
                        ),
                    )
                    .or(sample_containers::dsl::barcode
                        .strict_word_similarity_commutator_op(query)
                        .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
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
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
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
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        use crate::schema::organisms;
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        organisms::dsl::organisms
            // This operation is defined by a first order index linking organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates
                    .on(organisms::dsl::nameplate_id.eq(nameplates::dsl::id)),
            )
            // This operation is defined by a first order index linking organisms.project_id to projects.
            .inner_join(
                projects0.on(organisms::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .left_join(
                samples0
                    .on(organisms::dsl::sample_id.eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .left_join(
                samples1
                    .on(organisms::dsl::sample_id.eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects1.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .left_join(
                samples2
                    .on(organisms::dsl::sample_id.eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select((
                Organism::as_select(),
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
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
            .filter(crate::sql_function_bindings::can_view_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .filter(
                nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
                    .or(
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
                        ),
                    )
                    .or(sample_containers::dsl::barcode
                        .strict_word_similarity_commutator_op(query)
                        .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
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
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
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
            .map(|mut entries| {
                entries.iter_mut().for_each(|entry| {
                    entry.1 /= 5.0;
                });
                entries
            })
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
        diesel::select(crate::sql_function_bindings::can_update_organisms(
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
        filter: Option<&OrganismFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_update_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::id);
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        filter: Option<&OrganismFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_update_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        filter: Option<&OrganismFilter>,
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
        use crate::schema::organisms;
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = organisms::dsl::organisms
            // This operation is defined by a first order index linking organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates
                    .on(organisms::dsl::nameplate_id.eq(nameplates::dsl::id)),
            )
            // This operation is defined by a first order index linking organisms.project_id to projects.
            .inner_join(
                projects0.on(organisms::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .left_join(
                samples0
                    .on(organisms::dsl::sample_id.eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .left_join(
                samples1
                    .on(organisms::dsl::sample_id.eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects1.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .left_join(
                samples2
                    .on(organisms::dsl::sample_id.eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_update_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .filter(
                nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
                    .or(
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
                        ),
                    )
                    .or(sample_containers::dsl::barcode
                        .strict_word_similarity_commutator_op(query)
                        .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
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
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
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
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        diesel::select(crate::sql_function_bindings::can_admin_organisms(
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
        filter: Option<&OrganismFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_admin_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::id);
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        filter: Option<&OrganismFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organisms;
        let query = organisms::dsl::organisms
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_admin_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .order_by(organisms::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        filter: Option<&OrganismFilter>,
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
        use crate::schema::organisms;
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (samples0, samples1, samples2) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2
        );
        let mut query = organisms::dsl::organisms
            // This operation is defined by a first order index linking organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates
                    .on(organisms::dsl::nameplate_id.eq(nameplates::dsl::id)),
            )
            // This operation is defined by a first order index linking organisms.project_id to projects.
            .inner_join(
                projects0.on(organisms::dsl::project_id.eq(projects0.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .left_join(
                samples0
                    .on(organisms::dsl::sample_id.eq(samples0.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.container_id to sample_containers.
            .inner_join(
                sample_containers::dsl::sample_containers.on(samples0
                    .field(samples::dsl::container_id)
                    .eq(sample_containers::dsl::id)),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .left_join(
                samples1
                    .on(organisms::dsl::sample_id.eq(samples1.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.project_id to projects.
            .inner_join(
                projects1.on(samples1
                    .field(samples::dsl::project_id)
                    .eq(projects1.field(projects::dsl::id))),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .left_join(
                samples2
                    .on(organisms::dsl::sample_id.eq(samples2.field(samples::dsl::id).nullable())),
            )
            // This operation is defined by a second order index linking organisms.sample_id to samples and samples.state_id to sample_states.
            .inner_join(
                sample_states::dsl::sample_states.on(samples2
                    .field(samples::dsl::state_id)
                    .eq(sample_states::dsl::id)),
            )
            .select(Organism::as_select())
            .filter(crate::sql_function_bindings::can_admin_organisms(
                author_user_id,
                organisms::dsl::id,
            ))
            .filter(
                nameplates::dsl::barcode
                    .strict_word_similarity_commutator_op(query)
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query)))
                    .or(
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
                        ),
                    )
                    .or(sample_containers::dsl::barcode
                        .strict_word_similarity_commutator_op(query)
                        .or(sample_containers::dsl::barcode.ilike(format!("%{}%", query))))
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
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects0.field(projects::dsl::name),
                        projects0.field(projects::dsl::description),
                    ),
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    sample_containers::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects1.field(projects::dsl::name),
                        projects1.field(projects::dsl::description),
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
        if let Some(host_organism_id) = filter.and_then(|f| f.host_organism_id) {
            query = query.filter(organisms::dsl::host_organism_id.eq(host_organism_id));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(organisms::dsl::sample_id.eq(sample_id));
        }
        if let Some(nameplate_id) = filter.and_then(|f| f.nameplate_id) {
            query = query.filter(organisms::dsl::nameplate_id.eq(nameplate_id));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(organisms::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organisms::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(organisms::dsl::updated_by.eq(updated_by));
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
        diesel::delete(organisms::dsl::organisms.filter(organisms::dsl::id.eq(id)))
            .execute(connection)
            .map_err(web_common::api::ApiError::from)
    }
}
