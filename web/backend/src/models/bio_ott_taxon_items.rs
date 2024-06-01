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
#[diesel(table_name = bio_ott_taxon_items)]
#[diesel(belongs_to(crate::models::bio_ott_ranks::BioOttRank, foreign_key = ott_rank_id))]
#[diesel(belongs_to(crate::models::bio_ott_taxon_items::BioOttTaxonItem, foreign_key = domain_id))]
#[diesel(belongs_to(crate::models::font_awesome_icons::FontAwesomeIcon, foreign_key = icon_id))]
#[diesel(belongs_to(crate::models::colors::Color, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct BioOttTaxonItem {
    pub id: i32,
    pub name: String,
    pub ott_id: i32,
    pub ott_rank_id: i32,
    pub wikidata_id: Option<i32>,
    pub ncbi_id: Option<i32>,
    pub gbif_id: Option<i32>,
    pub irmng_id: Option<i32>,
    pub worms_id: Option<i32>,
    pub domain_id: Option<i32>,
    pub kingdom_id: Option<i32>,
    pub phylum_id: Option<i32>,
    pub class_id: Option<i32>,
    pub order_id: Option<i32>,
    pub family_id: Option<i32>,
    pub genus_id: Option<i32>,
    pub parent_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<BioOttTaxonItem> for web_common::database::tables::BioOttTaxonItem {
    fn from(item: BioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            ott_id: item.ott_id,
            ott_rank_id: item.ott_rank_id,
            wikidata_id: item.wikidata_id,
            ncbi_id: item.ncbi_id,
            gbif_id: item.gbif_id,
            irmng_id: item.irmng_id,
            worms_id: item.worms_id,
            domain_id: item.domain_id,
            kingdom_id: item.kingdom_id,
            phylum_id: item.phylum_id,
            class_id: item.class_id,
            order_id: item.order_id,
            family_id: item.family_id,
            genus_id: item.genus_id,
            parent_id: item.parent_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::BioOttTaxonItem> for BioOttTaxonItem {
    fn from(item: web_common::database::tables::BioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            name: item.name,
            ott_id: item.ott_id,
            ott_rank_id: item.ott_rank_id,
            wikidata_id: item.wikidata_id,
            ncbi_id: item.ncbi_id,
            gbif_id: item.gbif_id,
            irmng_id: item.irmng_id,
            worms_id: item.worms_id,
            domain_id: item.domain_id,
            kingdom_id: item.kingdom_id,
            phylum_id: item.phylum_id,
            class_id: item.class_id,
            order_id: item.order_id,
            family_id: item.family_id,
            genus_id: item.genus_id,
            parent_id: item.parent_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl BioOttTaxonItem {
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
        filter: Option<&BioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items.into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(domain_id) = filter.and_then(|f| f.domain_id) {
            query = query.filter(bio_ott_taxon_items::dsl::domain_id.eq(domain_id));
        }
        if let Some(kingdom_id) = filter.and_then(|f| f.kingdom_id) {
            query = query.filter(bio_ott_taxon_items::dsl::kingdom_id.eq(kingdom_id));
        }
        if let Some(phylum_id) = filter.and_then(|f| f.phylum_id) {
            query = query.filter(bio_ott_taxon_items::dsl::phylum_id.eq(phylum_id));
        }
        if let Some(class_id) = filter.and_then(|f| f.class_id) {
            query = query.filter(bio_ott_taxon_items::dsl::class_id.eq(class_id));
        }
        if let Some(order_id) = filter.and_then(|f| f.order_id) {
            query = query.filter(bio_ott_taxon_items::dsl::order_id.eq(order_id));
        }
        if let Some(family_id) = filter.and_then(|f| f.family_id) {
            query = query.filter(bio_ott_taxon_items::dsl::family_id.eq(family_id));
        }
        if let Some(genus_id) = filter.and_then(|f| f.genus_id) {
            query = query.filter(bio_ott_taxon_items::dsl::genus_id.eq(genus_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
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
        filter: Option<&BioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Vec<Self>, web_common::api::ApiError> {
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items.into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(domain_id) = filter.and_then(|f| f.domain_id) {
            query = query.filter(bio_ott_taxon_items::dsl::domain_id.eq(domain_id));
        }
        if let Some(kingdom_id) = filter.and_then(|f| f.kingdom_id) {
            query = query.filter(bio_ott_taxon_items::dsl::kingdom_id.eq(kingdom_id));
        }
        if let Some(phylum_id) = filter.and_then(|f| f.phylum_id) {
            query = query.filter(bio_ott_taxon_items::dsl::phylum_id.eq(phylum_id));
        }
        if let Some(class_id) = filter.and_then(|f| f.class_id) {
            query = query.filter(bio_ott_taxon_items::dsl::class_id.eq(class_id));
        }
        if let Some(order_id) = filter.and_then(|f| f.order_id) {
            query = query.filter(bio_ott_taxon_items::dsl::order_id.eq(order_id));
        }
        if let Some(family_id) = filter.and_then(|f| f.family_id) {
            query = query.filter(bio_ott_taxon_items::dsl::family_id.eq(family_id));
        }
        if let Some(genus_id) = filter.and_then(|f| f.genus_id) {
            query = query.filter(bio_ott_taxon_items::dsl::genus_id.eq(genus_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
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
        use crate::schema::bio_ott_taxon_items;
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::id.eq(id))
            .first::<Self>(connection)
            .map_err(web_common::api::ApiError::from)
    }
    /// Get the struct from the database by its ott_id.
    ///
    /// * `ott_id` - The ott_id of the struct to get.
    /// * `connection` - The connection to the database.
    pub fn from_ott_id(
        ott_id: &i32,
        connection: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
    ) -> Result<Self, web_common::api::ApiError> {
        use crate::schema::bio_ott_taxon_items;
        let flat_variant = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_id.eq(ott_id))
            .first::<Self>(connection)?;
        Ok(flat_variant)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// * `filter` - The optional filter to apply to the query.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    /// * `connection` - The connection to the database.
    pub fn similarity_search_viewable(
        filter: Option<&BioOttTaxonItemFilter>,
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
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(
                crate::sql_function_bindings::similarity_op(bio_ott_taxon_items::dsl::name, query)
                    .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query))),
            )
            .order(crate::sql_function_bindings::similarity_dist(
                bio_ott_taxon_items::dsl::name,
                query,
            ))
            .into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
        }
        query
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
        filter: Option<&BioOttTaxonItemFilter>,
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
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(
                crate::sql_function_bindings::word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query))),
            )
            .order(crate::sql_function_bindings::word_similarity_dist_op(
                bio_ott_taxon_items::dsl::name,
                query,
            ))
            .into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
        }
        query
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
        filter: Option<&BioOttTaxonItemFilter>,
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
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::domain_id.eq(filter.and_then(|f| f.domain_id)))
            .filter(bio_ott_taxon_items::dsl::kingdom_id.eq(filter.and_then(|f| f.kingdom_id)))
            .filter(bio_ott_taxon_items::dsl::phylum_id.eq(filter.and_then(|f| f.phylum_id)))
            .filter(bio_ott_taxon_items::dsl::class_id.eq(filter.and_then(|f| f.class_id)))
            .filter(bio_ott_taxon_items::dsl::order_id.eq(filter.and_then(|f| f.order_id)))
            .filter(bio_ott_taxon_items::dsl::family_id.eq(filter.and_then(|f| f.family_id)))
            .filter(bio_ott_taxon_items::dsl::genus_id.eq(filter.and_then(|f| f.genus_id)))
            .filter(
                crate::sql_function_bindings::strict_word_similarity_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                )
                .or(bio_ott_taxon_items::dsl::name.ilike(format!("%{}%", query))),
            )
            .order(
                crate::sql_function_bindings::strict_word_similarity_dist_op(
                    bio_ott_taxon_items::dsl::name,
                    query,
                ),
            )
            .into_boxed();
        if let Some(ott_rank_id) = filter.and_then(|f| f.ott_rank_id) {
            query = query.filter(bio_ott_taxon_items::dsl::ott_rank_id.eq(ott_rank_id));
        }
        if let Some(parent_id) = filter.and_then(|f| f.parent_id) {
            query = query.filter(bio_ott_taxon_items::dsl::parent_id.eq(parent_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_taxon_items::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_taxon_items::dsl::color_id.eq(color_id));
        }
        query
            .limit(limit.unwrap_or(10))
            .offset(offset.unwrap_or(0))
            .load::<Self>(connection)
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
