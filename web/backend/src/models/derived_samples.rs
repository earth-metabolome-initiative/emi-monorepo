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
    PartialEq,
    PartialOrd,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    QueryableByName,
    Associations,
    Insertable,
    Selectable,
    AsChangeset,
)]
#[diesel(table_name = derived_samples)]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::models::samples::Sample, foreign_key = parent_sample_id))]
#[diesel(belongs_to(crate::models::units::Unit, foreign_key = unit_id))]
#[diesel(primary_key(parent_sample_id, child_sample_id))]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: chrono::NaiveDateTime,
    pub parent_sample_id: uuid::Uuid,
    pub child_sample_id: uuid::Uuid,
    pub quantity: f64,
    pub unit_id: i32,
}

impl From<DerivedSample> for web_common::database::tables::DerivedSample {
    fn from(item: DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::DerivedSample> for DerivedSample {
    fn from(item: web_common::database::tables::DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl DerivedSample {
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
        Self::can_view_by_id(
            (self.parent_sample_id, self.child_sample_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        (parent_sample_id, child_sample_id): (uuid::Uuid, uuid::Uuid),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_view_derived_samples(
            author_user_id,
            parent_sample_id,
            child_sample_id,
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_view_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::parent_sample_id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_view_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        (parent_sample_id, child_sample_id): (uuid::Uuid, uuid::Uuid),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(
            (parent_sample_id, child_sample_id),
            author_user_id,
            connection,
        )? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::derived_samples;
        derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
            .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id))
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
        filter: Option<&DerivedSampleFilter>,
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
        use crate::schema::derived_samples;
        let (samples0, samples1, samples2, samples3, samples4, samples5) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2,
            crate::schema::samples as samples3,
            crate::schema::samples as samples4,
            crate::schema::samples as samples5
        );
        let (sample_containers0, sample_containers1) = diesel::alias!(
            crate::schema::sample_containers as sample_containers0,
            crate::schema::sample_containers as sample_containers1
        );
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (sample_states0, sample_states1) = diesel::alias!(
            crate::schema::sample_states as sample_states0,
            crate::schema::sample_states as sample_states1
        );
        let mut query =
            derived_samples::dsl::derived_samples
                // This operation is defined by a first order index linking derived_samples.unit_id to units.
                .inner_join(units::dsl::units.on(derived_samples::dsl::unit_id.eq(units::dsl::id)))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(samples0.on(
                    derived_samples::dsl::parent_sample_id.eq(samples0.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers0.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers0.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(samples1.on(
                    derived_samples::dsl::parent_sample_id.eq(samples1.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects0.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects0.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(samples2.on(
                    derived_samples::dsl::parent_sample_id.eq(samples2.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states0.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states0.field(sample_states::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples3
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples3.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers1.on(samples3
                        .field(samples::dsl::container_id)
                        .eq(sample_containers1.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples4
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples4.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects1.on(samples4
                        .field(samples::dsl::project_id)
                        .eq(projects1.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples5
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples5.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states1.on(samples5
                        .field(samples::dsl::state_id)
                        .eq(sample_states1.field(sample_states::dsl::id))),
                )
                .select(DerivedSample::as_select())
                .filter(crate::sql_function_bindings::can_view_derived_samples(
                    author_user_id,
                    derived_samples::dsl::parent_sample_id,
                    derived_samples::dsl::child_sample_id,
                ))
                .filter(
                    crate::sql_function_bindings::concat_units_name_unit(
                        units::dsl::name,
                        units::dsl::unit,
                    )
                    .ilike(format!("%{}%", query))
                    .or(sample_containers0
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(sample_containers1
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_units_name_unit(
                            units::dsl::name,
                            units::dsl::unit,
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers0.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers1.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        ),
                        query,
                    ),
                )
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        use crate::schema::derived_samples;
        let (samples0, samples1, samples2, samples3, samples4, samples5) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2,
            crate::schema::samples as samples3,
            crate::schema::samples as samples4,
            crate::schema::samples as samples5
        );
        let (sample_containers0, sample_containers1) = diesel::alias!(
            crate::schema::sample_containers as sample_containers0,
            crate::schema::sample_containers as sample_containers1
        );
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (sample_states0, sample_states1) = diesel::alias!(
            crate::schema::sample_states as sample_states0,
            crate::schema::sample_states as sample_states1
        );
        derived_samples::dsl::derived_samples
            // This operation is defined by a first order index linking derived_samples.unit_id to units.
.inner_join(
   units::dsl::units.on(
       derived_samples::dsl::unit_id.eq(
           units::dsl::id
        )
    )
)

// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       derived_samples::dsl::parent_sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers0.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers0.field(sample_containers::dsl::id)
        )
    )
)

// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       derived_samples::dsl::parent_sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
.inner_join(
   projects0.on(
       samples1.field(samples::dsl::project_id).eq(
           projects0.field(projects::dsl::id)
        )
    )
)

// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       derived_samples::dsl::parent_sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states0.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states0.field(sample_states::dsl::id)
        )
    )
)

// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples3.on(
       derived_samples::dsl::child_sample_id.eq(
           samples3.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers1.on(
       samples3.field(samples::dsl::container_id).eq(
           sample_containers1.field(sample_containers::dsl::id)
        )
    )
)

// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
.inner_join(
   samples4.on(
       derived_samples::dsl::child_sample_id.eq(
           samples4.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
.inner_join(
   projects1.on(
       samples4.field(samples::dsl::project_id).eq(
           projects1.field(projects::dsl::id)
        )
    )
)

// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples5.on(
       derived_samples::dsl::child_sample_id.eq(
           samples5.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states1.on(
       samples5.field(samples::dsl::state_id).eq(
           sample_states1.field(sample_states::dsl::id)
        )
    )
)

            .select((
    DerivedSample::as_select(),
    crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_units_name_unit(units::dsl::name, units::dsl::unit), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(sample_containers0.field(sample_containers::dsl::barcode), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_projects_name_description(projects0.field(projects::dsl::name), projects0.field(projects::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_sample_states_name_description(sample_states0.field(sample_states::dsl::name), sample_states0.field(sample_states::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(sample_containers1.field(sample_containers::dsl::barcode), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_projects_name_description(projects1.field(projects::dsl::name), projects1.field(projects::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_sample_states_name_description(sample_states1.field(sample_states::dsl::name), sample_states1.field(sample_states::dsl::description)), query)
))
            .filter(crate::sql_function_bindings::can_view_derived_samples(author_user_id, derived_samples::dsl::parent_sample_id, derived_samples::dsl::child_sample_id))
            .filter(
    crate::sql_function_bindings::concat_units_name_unit(units::dsl::name, units::dsl::unit).ilike(format!("%{}%", query))
    .or(
    sample_containers0.field(sample_containers::dsl::barcode).ilike(format!("%{}%", query))
    )
    .or(
    crate::sql_function_bindings::concat_projects_name_description(projects0.field(projects::dsl::name), projects0.field(projects::dsl::description)).ilike(format!("%{}%", query))
    )
    .or(
    crate::sql_function_bindings::concat_sample_states_name_description(sample_states0.field(sample_states::dsl::name), sample_states0.field(sample_states::dsl::description)).ilike(format!("%{}%", query))
    )
    .or(
    sample_containers1.field(sample_containers::dsl::barcode).ilike(format!("%{}%", query))
    )
    .or(
    crate::sql_function_bindings::concat_projects_name_description(projects1.field(projects::dsl::name), projects1.field(projects::dsl::description)).ilike(format!("%{}%", query))
    )
    .or(
    crate::sql_function_bindings::concat_sample_states_name_description(sample_states1.field(sample_states::dsl::name), sample_states1.field(sample_states::dsl::description)).ilike(format!("%{}%", query))
    )
)
            .order(crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_units_name_unit(units::dsl::name, units::dsl::unit), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(sample_containers0.field(sample_containers::dsl::barcode), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_projects_name_description(projects0.field(projects::dsl::name), projects0.field(projects::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_sample_states_name_description(sample_states0.field(sample_states::dsl::name), sample_states0.field(sample_states::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(sample_containers1.field(sample_containers::dsl::barcode), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_projects_name_description(projects1.field(projects::dsl::name), projects1.field(projects::dsl::description)), query)    +
crate::sql_function_bindings::strict_word_similarity_dist_op(crate::sql_function_bindings::concat_sample_states_name_description(sample_states1.field(sample_states::dsl::name), sample_states1.field(sample_states::dsl::description)), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<(Self, f32)>(connection).map_err(web_common::api::ApiError::from)
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
        Self::can_update_by_id(
            (self.parent_sample_id, self.child_sample_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        (parent_sample_id, child_sample_id): (uuid::Uuid, uuid::Uuid),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_update_derived_samples(
            author_user_id,
            parent_sample_id,
            child_sample_id,
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_update_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::parent_sample_id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_update_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        filter: Option<&DerivedSampleFilter>,
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
        use crate::schema::derived_samples;
        let (samples0, samples1, samples2, samples3, samples4, samples5) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2,
            crate::schema::samples as samples3,
            crate::schema::samples as samples4,
            crate::schema::samples as samples5
        );
        let (sample_containers0, sample_containers1) = diesel::alias!(
            crate::schema::sample_containers as sample_containers0,
            crate::schema::sample_containers as sample_containers1
        );
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (sample_states0, sample_states1) = diesel::alias!(
            crate::schema::sample_states as sample_states0,
            crate::schema::sample_states as sample_states1
        );
        let mut query =
            derived_samples::dsl::derived_samples
                // This operation is defined by a first order index linking derived_samples.unit_id to units.
                .inner_join(units::dsl::units.on(derived_samples::dsl::unit_id.eq(units::dsl::id)))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(samples0.on(
                    derived_samples::dsl::parent_sample_id.eq(samples0.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers0.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers0.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(samples1.on(
                    derived_samples::dsl::parent_sample_id.eq(samples1.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects0.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects0.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(samples2.on(
                    derived_samples::dsl::parent_sample_id.eq(samples2.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states0.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states0.field(sample_states::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples3
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples3.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers1.on(samples3
                        .field(samples::dsl::container_id)
                        .eq(sample_containers1.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples4
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples4.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects1.on(samples4
                        .field(samples::dsl::project_id)
                        .eq(projects1.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples5
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples5.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states1.on(samples5
                        .field(samples::dsl::state_id)
                        .eq(sample_states1.field(sample_states::dsl::id))),
                )
                .select(DerivedSample::as_select())
                .filter(crate::sql_function_bindings::can_update_derived_samples(
                    author_user_id,
                    derived_samples::dsl::parent_sample_id,
                    derived_samples::dsl::child_sample_id,
                ))
                .filter(
                    crate::sql_function_bindings::concat_units_name_unit(
                        units::dsl::name,
                        units::dsl::unit,
                    )
                    .ilike(format!("%{}%", query))
                    .or(sample_containers0
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(sample_containers1
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_units_name_unit(
                            units::dsl::name,
                            units::dsl::unit,
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers0.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers1.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        ),
                        query,
                    ),
                )
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        Self::can_admin_by_id(
            (self.parent_sample_id, self.child_sample_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        (parent_sample_id, child_sample_id): (uuid::Uuid, uuid::Uuid),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(crate::sql_function_bindings::can_admin_derived_samples(
            author_user_id,
            parent_sample_id,
            child_sample_id,
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_admin_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::parent_sample_id);
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        filter: Option<&DerivedSampleFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples
            .select(DerivedSample::as_select())
            .filter(crate::sql_function_bindings::can_admin_derived_samples(
                author_user_id,
                derived_samples::dsl::parent_sample_id,
                derived_samples::dsl::child_sample_id,
            ))
            .order_by(derived_samples::dsl::updated_at.desc());
        let mut query = query.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        filter: Option<&DerivedSampleFilter>,
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
        use crate::schema::derived_samples;
        let (samples0, samples1, samples2, samples3, samples4, samples5) = diesel::alias!(
            crate::schema::samples as samples0,
            crate::schema::samples as samples1,
            crate::schema::samples as samples2,
            crate::schema::samples as samples3,
            crate::schema::samples as samples4,
            crate::schema::samples as samples5
        );
        let (sample_containers0, sample_containers1) = diesel::alias!(
            crate::schema::sample_containers as sample_containers0,
            crate::schema::sample_containers as sample_containers1
        );
        let (projects0, projects1) = diesel::alias!(
            crate::schema::projects as projects0,
            crate::schema::projects as projects1
        );
        let (sample_states0, sample_states1) = diesel::alias!(
            crate::schema::sample_states as sample_states0,
            crate::schema::sample_states as sample_states1
        );
        let mut query =
            derived_samples::dsl::derived_samples
                // This operation is defined by a first order index linking derived_samples.unit_id to units.
                .inner_join(units::dsl::units.on(derived_samples::dsl::unit_id.eq(units::dsl::id)))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(samples0.on(
                    derived_samples::dsl::parent_sample_id.eq(samples0.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers0.on(samples0
                        .field(samples::dsl::container_id)
                        .eq(sample_containers0.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(samples1.on(
                    derived_samples::dsl::parent_sample_id.eq(samples1.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects0.on(samples1
                        .field(samples::dsl::project_id)
                        .eq(projects0.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(samples2.on(
                    derived_samples::dsl::parent_sample_id.eq(samples2.field(samples::dsl::id)),
                ))
                // This operation is defined by a second order index linking derived_samples.parent_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states0.on(samples2
                        .field(samples::dsl::state_id)
                        .eq(sample_states0.field(sample_states::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    samples3
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples3.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.container_id to sample_containers.
                .inner_join(
                    sample_containers1.on(samples3
                        .field(samples::dsl::container_id)
                        .eq(sample_containers1.field(sample_containers::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    samples4
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples4.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.project_id to projects.
                .inner_join(
                    projects1.on(samples4
                        .field(samples::dsl::project_id)
                        .eq(projects1.field(projects::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    samples5
                        .on(derived_samples::dsl::child_sample_id
                            .eq(samples5.field(samples::dsl::id))),
                )
                // This operation is defined by a second order index linking derived_samples.child_sample_id to samples and samples.state_id to sample_states.
                .inner_join(
                    sample_states1.on(samples5
                        .field(samples::dsl::state_id)
                        .eq(sample_states1.field(sample_states::dsl::id))),
                )
                .select(DerivedSample::as_select())
                .filter(crate::sql_function_bindings::can_admin_derived_samples(
                    author_user_id,
                    derived_samples::dsl::parent_sample_id,
                    derived_samples::dsl::child_sample_id,
                ))
                .filter(
                    crate::sql_function_bindings::concat_units_name_unit(
                        units::dsl::name,
                        units::dsl::unit,
                    )
                    .ilike(format!("%{}%", query))
                    .or(sample_containers0
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(sample_containers1
                        .field(sample_containers::dsl::barcode)
                        .ilike(format!("%{}%", query)))
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    )
                    .or(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        )
                        .ilike(format!("%{}%", query)),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_units_name_unit(
                            units::dsl::name,
                            units::dsl::unit,
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers0.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects0.field(projects::dsl::name),
                            projects0.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states0.field(sample_states::dsl::name),
                            sample_states0.field(sample_states::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        sample_containers1.field(sample_containers::dsl::barcode),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects1.field(projects::dsl::name),
                            projects1.field(projects::dsl::description),
                        ),
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_sample_states_name_description(
                            sample_states1.field(sample_states::dsl::name),
                            sample_states1.field(sample_states::dsl::description),
                        ),
                        query,
                    ),
                )
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(derived_samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        if let Some(unit_id) = filter.and_then(|f| f.unit_id) {
            query = query.filter(derived_samples::dsl::unit_id.eq(unit_id));
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
        Self::delete_by_id(
            (self.parent_sample_id, self.child_sample_id),
            author_user_id,
            connection,
        )
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        (parent_sample_id, child_sample_id): (uuid::Uuid, uuid::Uuid),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(
            (parent_sample_id, child_sample_id),
            author_user_id,
            connection,
        )? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            derived_samples::dsl::derived_samples
                .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
                .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
