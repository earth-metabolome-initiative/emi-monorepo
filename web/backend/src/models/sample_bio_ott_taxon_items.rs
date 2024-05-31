//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use crate::sql_function_bindings::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use web_common::database::filter_structs::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::models::samples::Sample, foreign_key = sample_id))]
#[diesel(belongs_to(crate::models::bio_ott_taxon_items::BioOttTaxonItem, foreign_key = taxon_id))]
#[diesel(primary_key(sample_id, taxon_id))]
pub struct SampleBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampleBioOttTaxonItem> for web_common::database::tables::SampleBioOttTaxonItem {
    fn from(item: SampleBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampleBioOttTaxonItem> for SampleBioOttTaxonItem {
    fn from(item: web_common::database::tables::SampleBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampleBioOttTaxonItem {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view(
        &self,
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_view_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_view_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted viewable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: Option<i32>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Self, web_common::api::ApiError>{
        if !Self::can_view_by_id(( sample_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sample_bio_ott_taxon_items;
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .first::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: Option<i32>,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_view_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can update the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_update_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_update_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted updatable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_updatable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the updatable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_updatable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_updatable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the updatable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_updatable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_updatable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_update_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Check whether the user can admin the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            ( self.sample_id, self.taxon_id ),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn can_admin_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<bool, web_common::api::ApiError>{
       diesel::select(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_id, taxon_id))
            .get_result(connection).map_err(web_common::api::ApiError::from)
}
    /// Get all of the administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Get all of the sorted administrable structs from the database.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_administrable_sorted(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
    }
    /// Search for the administrable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_administrable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(sample_containers::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
similarity_dist(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_administrable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Search for the administrable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_administrable(
filter: Option<&SampleBioOttTaxonItemFilter>,
author_user_id: i32,
query: &str,
limit: Option<i64>,
offset: Option<i64>,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<Vec<Self>, web_common::api::ApiError>{
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_administrable(filter, author_user_id, limit, offset, connection);
        }
        use crate::schema::sample_bio_ott_taxon_items;
let (samples0, samples1, samples2) = diesel::alias!(crate::schema::samples as samples0, crate::schema::samples as samples1, crate::schema::samples as samples2);
 if filter.map(|f| f.created_by.is_some()&&f.sample_id.is_some()&&f.taxon_id.is_some()).unwrap_or(false) {
       unimplemented!();
 }
if let Some(created_by) = filter.and_then(|f| f.created_by) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(sample_id) = filter.and_then(|f| f.sample_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
        return sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
}
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .select(SampleBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sample_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sample_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   samples0.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples0.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.container_id to sample_containers.
.inner_join(
   sample_containers::dsl::sample_containers.on(
       samples0.field(samples::dsl::container_id).eq(
           sample_containers::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   samples1.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples1.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       samples1.field(samples::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   samples2.on(
       sample_bio_ott_taxon_items::dsl::sample_id.eq(
           samples2.field(samples::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sample_bio_ott_taxon_items.sample_id to samples and samples.state_id to sample_states.
.inner_join(
   sample_states::dsl::sample_states.on(
       samples2.field(samples::dsl::state_id).eq(
           sample_states::dsl::id
        )
    )
)

            .filter(can_admin_sample_bio_ott_taxon_items(author_user_id, sample_bio_ott_taxon_items::dsl::sample_id, sample_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(sample_containers::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
    .or(
strict_word_similarity_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(sample_containers::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    +
strict_word_similarity_dist_op(concat_sample_states_name_description(sample_states::dsl::name, sample_states::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from)
}
    /// Delete the struct from the database.
    ///
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        Self::delete_by_id(( self.sample_id, self.taxon_id ), author_user_id, connection)
}
    /// Delete the struct from the database by its ID.
    ///
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
( sample_id, taxon_id ): ( Uuid, i32 ),
author_user_id: i32,
connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
) -> Result<usize, web_common::api::ApiError>{
        if !Self::can_admin_by_id(( sample_id, taxon_id ), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
        ).execute(connection).map_err(web_common::api::ApiError::from)
    }
}
