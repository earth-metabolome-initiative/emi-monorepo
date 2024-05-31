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
#[diesel(table_name = organism_bio_ott_taxon_items)]
#[diesel(belongs_to(crate::models::users::User, foreign_key = created_by))]
#[diesel(belongs_to(crate::models::organisms::Organism, foreign_key = organism_id))]
#[diesel(belongs_to(crate::models::bio_ott_taxon_items::BioOttTaxonItem, foreign_key = taxon_id))]
#[diesel(primary_key(organism_id, taxon_id))]
pub struct OrganismBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
    pub organism_id: uuid::Uuid,
    pub taxon_id: i32,
}

impl From<OrganismBioOttTaxonItem> for web_common::database::tables::OrganismBioOttTaxonItem {
    fn from(item: OrganismBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            organism_id: item.organism_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::OrganismBioOttTaxonItem> for OrganismBioOttTaxonItem {
    fn from(item: web_common::database::tables::OrganismBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            organism_id: item.organism_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl OrganismBioOttTaxonItem {
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
            (self.organism_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can view the struct associated to the provided ids.
    ///
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_view_by_id(
        (organism_id, taxon_id): (uuid::Uuid, i32),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                author_user_id,
                organism_id,
                taxon_id,
            ),
        )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .order_by(organism_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ID.
    ///
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `connection` - The connection to the database.
    pub fn get(
        (organism_id, taxon_id): (uuid::Uuid, i32),
        author_user_id: Option<i32>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        if !Self::can_view_by_id((organism_id, taxon_id), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        use crate::schema::organism_bio_ott_taxon_items;
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
            .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
            )
            .order(
                crate::sql_function_bindings::similarity_dist(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::similarity_dist(nameplates::dsl::barcode, query)
                    + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_view_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::strict_word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_update_by_id(
            (self.organism_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can update the struct associated to the provided ids.
    ///
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_update_by_id(
        (organism_id, taxon_id): (uuid::Uuid, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                author_user_id,
                organism_id,
                taxon_id,
            ),
        )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .order_by(organism_bio_ott_taxon_items::dsl::created_at.desc())
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
            )
            .order(
                crate::sql_function_bindings::similarity_dist(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::similarity_dist(nameplates::dsl::barcode, query)
                    + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_update_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::strict_word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        Self::can_admin_by_id(
            (self.organism_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Check whether the user can admin the struct associated to the provided ids.
    ///
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to check.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    pub fn can_admin_by_id(
        (organism_id, taxon_id): (uuid::Uuid, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<bool, web_common::api::ApiError> {
        diesel::select(
            crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                author_user_id,
                organism_id,
                taxon_id,
            ),
        )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::organism_bio_ott_taxon_items;
        let mut query =
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items.into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .filter(
                crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .order_by(organism_bio_ott_taxon_items::dsl::created_at.desc())
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::similarity_dist(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
            )
            .order(
                crate::sql_function_bindings::similarity_dist(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::similarity_dist(nameplates::dsl::barcode, query)
                    + crate::sql_function_bindings::similarity_dist(
                        crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(crate::sql_function_bindings::word_similarity_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    )
                    .or(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        )
                        .ilike(format!("%{}%", query)),
                    )),
                )
                .order(
                    crate::sql_function_bindings::word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        filter: Option<&OrganismBioOttTaxonItemFilter>,
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
        use crate::schema::organism_bio_ott_taxon_items;
        let (organisms0, organisms1) = diesel::alias!(
            crate::schema::organisms as organisms0,
            crate::schema::organisms as organisms1
        );
        if filter
            .map(|f| f.created_by.is_some() && f.organism_id.is_some() && f.taxon_id.is_some())
            .unwrap_or(false)
        {
            unimplemented!();
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::created_by.eq(created_by))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(organism_id) = filter.and_then(|f| f.organism_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            return organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
                .select(OrganismBioOttTaxonItem::as_select())
                // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
                .inner_join(bio_ott_taxon_items::dsl::bio_ott_taxon_items.on(
                    organism_bio_ott_taxon_items::dsl::taxon_id.eq(bio_ott_taxon_items::dsl::id),
                ))
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms0.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
                .inner_join(
                    nameplates::dsl::nameplates.on(organisms0
                        .field(organisms::dsl::nameplate_id)
                        .eq(nameplates::dsl::id)),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                        .eq(organisms1.field(organisms::dsl::id))),
                )
                // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
                .inner_join(
                    projects::dsl::projects.on(organisms1
                        .field(organisms::dsl::project_id)
                        .eq(projects::dsl::id)),
                )
                .filter(
                    crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                        author_user_id,
                        organism_bio_ott_taxon_items::dsl::organism_id,
                        organism_bio_ott_taxon_items::dsl::taxon_id,
                    ),
                )
                .filter(
                    crate::sql_function_bindings::strict_word_similarity_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    )
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                    .or(crate::sql_function_bindings::strict_word_similarity_op(
                        nameplates::dsl::barcode,
                        query,
                    )
                    .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                    .or(
                        crate::sql_function_bindings::strict_word_similarity_op(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            ),
                            query,
                        )
                        .or(
                            crate::sql_function_bindings::concat_projects_name_description(
                                projects::dsl::name,
                                projects::dsl::description,
                            )
                            .ilike(format!("%{}%", query)),
                        ),
                    ),
                )
                .order(
                    crate::sql_function_bindings::strict_word_similarity_dist_op(
                        bio_ott_taxon_items::dsl::name,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        nameplates::dsl::barcode,
                        query,
                    ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                        crate::sql_function_bindings::concat_projects_name_description(
                            projects::dsl::name,
                            projects::dsl::description,
                        ),
                        query,
                    ),
                )
                .limit(limit.unwrap_or(10))
                .offset(offset.unwrap_or(0))
                .load::<Self>(connection)
                .map_err(web_common::api::ApiError::from);
        }
        organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
            .select(OrganismBioOttTaxonItem::as_select())
            // This operation is defined by a first order index linking organism_bio_ott_taxon_items.taxon_id to bio_ott_taxon_items.
            .inner_join(
                bio_ott_taxon_items::dsl::bio_ott_taxon_items
                    .on(organism_bio_ott_taxon_items::dsl::taxon_id
                        .eq(bio_ott_taxon_items::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                organisms0.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms0.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.nameplate_id to nameplates.
            .inner_join(
                nameplates::dsl::nameplates.on(organisms0
                    .field(organisms::dsl::nameplate_id)
                    .eq(nameplates::dsl::id)),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                organisms1.on(organism_bio_ott_taxon_items::dsl::organism_id
                    .eq(organisms1.field(organisms::dsl::id))),
            )
            // This operation is defined by a second order index linking organism_bio_ott_taxon_items.organism_id to organisms and organisms.project_id to projects.
            .inner_join(
                projects::dsl::projects.on(organisms1
                    .field(organisms::dsl::project_id)
                    .eq(projects::dsl::id)),
            )
            .filter(
                crate::sql_function_bindings::can_admin_organism_bio_ott_taxon_items(
                    author_user_id,
                    organism_bio_ott_taxon_items::dsl::organism_id,
                    organism_bio_ott_taxon_items::dsl::taxon_id,
                ),
            )
            .filter(
                crate::sql_function_bindings::strict_word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query)))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    nameplates::dsl::barcode,
                    query,
                )
                .or(nameplates::dsl::barcode.ilike(format!("%{}%", query))))
                .or(crate::sql_function_bindings::strict_word_similarity_op(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    ),
                    query,
                )
                .or(
                    crate::sql_function_bindings::concat_projects_name_description(
                        projects::dsl::name,
                        projects::dsl::description,
                    )
                    .ilike(format!("%{}%", query)),
                )),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    nameplates::dsl::barcode,
                    query,
                ) + crate::sql_function_bindings::strict_word_similarity_dist_op(
                    crate::sql_function_bindings::concat_projects_name_description(
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
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        Self::delete_by_id(
            (self.organism_id, self.taxon_id),
            author_user_id,
            connection,
        )
    }
    /// Delete the struct from the database by its ID.
    ///
    /// * `( organism_id, taxon_id )` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    pub fn delete_by_id(
        (organism_id, taxon_id): (uuid::Uuid, i32),
        author_user_id: i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<usize, web_common::api::ApiError> {
        if !Self::can_admin_by_id((organism_id, taxon_id), author_user_id, connection)? {
            return Err(web_common::api::ApiError::Unauthorized);
        }
        diesel::delete(
            organism_bio_ott_taxon_items::dsl::organism_bio_ott_taxon_items
                .filter(organism_bio_ott_taxon_items::dsl::organism_id.eq(organism_id))
                .filter(organism_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id)),
        )
        .execute(connection)
        .map_err(web_common::api::ApiError::from)
    }
}