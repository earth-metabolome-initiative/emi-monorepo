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
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::models::sampled_individuals::SampledIndividual, foreign_key = sampled_individual_id))]
#[diesel(belongs_to(crate::models::bio_ott_taxon_items::BioOttTaxonItem, foreign_key = taxon_id))]
#[diesel(primary_key(sampled_individual_id, taxon_id))]
pub struct SampledIndividualBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampledIndividualBioOttTaxonItem>
    for web_common::database::tables::SampledIndividualBioOttTaxonItem
{
    fn from(item: SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampledIndividualBioOttTaxonItem>
    for SampledIndividualBioOttTaxonItem
{
    fn from(item: web_common::database::tables::SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampledIndividualBioOttTaxonItem {
    /// Check whether the user can view the struct.
    ///
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view(
        &self,
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_view_by_id(
            (self.sampled_individual_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        (sampled_individual_id, taxon_id): (Uuid, i32),
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_view_sampled_individual_bio_ott_taxon_items(
            author_user_id,
            sampled_individual_id,
            taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_view_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        (sampled_individual_id, taxon_id): (Uuid, i32),
        author_user_id: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id(
            (sampled_individual_id, taxon_id),
            author_user_id,
            connection,
        )? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            )
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_view_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(similarity_op(nameplates::dsl::barcode, query))
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(bio_ott_taxon_items::dsl::name, query)
                    + similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_view_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(word_similarity_op(nameplates::dsl::barcode, query))
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_view_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_view_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(strict_word_similarity_op(nameplates::dsl::barcode, query))
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        Self::can_update_by_id(
            (self.sampled_individual_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        (sampled_individual_id, taxon_id): (Uuid, i32),
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_update_sampled_individual_bio_ott_taxon_items(
            author_user_id,
            sampled_individual_id,
            taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_update_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_update_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(similarity_op(nameplates::dsl::barcode, query))
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(bio_ott_taxon_items::dsl::name, query)
                    + similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_update_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(word_similarity_op(nameplates::dsl::barcode, query))
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_update_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_update_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(strict_word_similarity_op(nameplates::dsl::barcode, query))
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        Self::can_admin_by_id(
            (self.sampled_individual_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        (sampled_individual_id, taxon_id): (Uuid, i32),
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(can_admin_sampled_individual_bio_ott_taxon_items(
            author_user_id,
            sampled_individual_id,
            taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query =
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query
                .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                    .eq(sampled_individual_id),
            );
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query =
                query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
similarity_op(nameplates::dsl::barcode, query)    )
    .or(
similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
similarity_dist(bio_ott_taxon_items::dsl::name, query)    +
similarity_dist(nameplates::dsl::barcode, query)    +
similarity_dist(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(similarity_op(nameplates::dsl::barcode, query))
                    .or(similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                similarity_dist(bio_ott_taxon_items::dsl::name, query)
                    + similarity_dist(nameplates::dsl::barcode, query)
                    + similarity_dist(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
word_similarity_dist_op(nameplates::dsl::barcode, query)    +
word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(word_similarity_op(nameplates::dsl::barcode, query))
                    .or(word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        filter: Option<&SampledIndividualBioOttTaxonItemFilter>,
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let (sampled_individuals0, sampled_individuals1) = diesel::alias!(
            crate::schema::sampled_individuals as sampled_individuals0,
            crate::schema::sampled_individuals as sampled_individuals1
        );
        if filter
            .map(|f| {
                f.created_by.is_some() && f.sampled_individual_id.is_some() && f.taxon_id.is_some()
            })
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
.inner_join(
   bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
       sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(
           bio_ott_taxon_items::dsl::id
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   sampled_individuals0.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals0.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
.inner_join(
   nameplates::dsl::nameplates.on(
       sampled_individuals0.field(sampled_individuals::dsl::nameplate_id).eq(
           nameplates::dsl::id.nullable()
        )
    )
)

// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   sampled_individuals1.on(
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(
           sampled_individuals1.field(sampled_individuals::dsl::id)
        )
    )
)
// This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
.inner_join(
   projects::dsl::projects.on(
       sampled_individuals1.field(sampled_individuals::dsl::project_id).eq(
           projects::dsl::id
        )
    )
)

            .filter(can_admin_sampled_individual_bio_ott_taxon_items(author_user_id, sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id, sampled_individual_bio_ott_taxon_items::dsl::taxon_id))
            .filter(
strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)    .or(
strict_word_similarity_op(nameplates::dsl::barcode, query)    )
    .or(
strict_word_similarity_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query)    )
)
            .order(
strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)    +
strict_word_similarity_dist_op(nameplates::dsl::barcode, query)    +
strict_word_similarity_dist_op(concat_projects_name_description(projects::dsl::name, projects::dsl::description), query))
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection).map_err(web_common::api::ApiError::from);
        }
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .select(SampledIndividualBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking sampled_individual_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(sampled_individual_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                sampled_individuals0.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals0.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(sampled_individuals0
                    .field(sampled_individuals::dsl::nameplate_id)
                    .eq(nameplates::dsl::id.nullable())),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                sampled_individuals1.on(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individuals1.field(sampled_individuals::dsl::id)),
                ),
            )
            // This operation is defined by a second order index linking sampled_individual_bio_ott_taxon_items.sampled_individual_id to sampled_individuals and sampled_individuals.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(sampled_individuals1
                    .field(sampled_individuals::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(can_admin_sampled_individual_bio_ott_taxon_items(
                author_user_id,
                sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id,
                sampled_individual_bio_ott_taxon_items::dsl::taxon_id,
            ))
            .filter(
                strict_word_similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(strict_word_similarity_op(nameplates::dsl::barcode, query))
                    .or(strict_word_similarity_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )),
            )
            .order(
                strict_word_similarity_dist_op(bio_ott_taxon_items::dsl::name, query)
                    + strict_word_similarity_dist_op(nameplates::dsl::barcode, query)
                    + strict_word_similarity_dist_op(
                        concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
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
        Self::delete_by_id(
            (self.sampled_individual_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        (sampled_individual_id, taxon_id): (Uuid, i32),
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id(
            (sampled_individual_id, taxon_id),
            author_user_id,
            connection,
        )? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
                .filter(
                    sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id
                        .eq(sampled_individual_id),
                )
                .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}
