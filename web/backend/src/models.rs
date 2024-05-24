//! This file is generated automatically and should not be modified.
//!
//! Any edits you may apply to this document will be overwritten next time the
//! backend is generated. Refrain from making any changes to this file.

//! If you need to make changes to the backend, please modify the `generate_models`
//! document in the `migrations` folder.

#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use diesel::QueryableByName;
use diesel::Identifiable;
use diesel::Insertable;
use crate::schema::*;
use diesel::Selectable;
use serde::Deserialize;
use serde::Serialize;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_ranks)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct BioOttRank {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<BioOttRank> for web_common::database::tables::BioOttRank {
    fn from(item: BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::BioOttRank> for BioOttRank {
    fn from(item: web_common::database::tables::BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl BioOttRank {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::BioOttRankFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_ranks;
        let mut query = bio_ott_ranks::dsl::bio_ott_ranks
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(bio_ott_ranks::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(bio_ott_ranks::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::bio_ott_ranks;
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::bio_ott_ranks;
        bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM bio_ott_ranks ",
            "WHERE $1 % f_concat_bio_ott_ranks_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_bio_ott_ranks_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM bio_ott_ranks ",
            "WHERE $1 <% f_concat_bio_ott_ranks_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_bio_ott_ranks_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM bio_ott_ranks ",
            "WHERE $1 <<% f_concat_bio_ott_ranks_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_bio_ott_ranks_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_taxon_items)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709839158ec0>, foreign_key = ott_rank_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa1760>, foreign_key = domain_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
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
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::BioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
        let mut query = bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .into_boxed();
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
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its ott_id.
    ///
    /// # Arguments
    /// * `ott_id` - The ott_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_ott_id(
        ott_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_id.eq(ott_id))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = colors)]
#[diesel(primary_key(id))]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
    pub description: String,
}

impl From<Color> for web_common::database::tables::Color {
    fn from(item: Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl From<web_common::database::tables::Color> for Color {
    fn from(item: web_common::database::tables::Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
            description: item.description,
        }
    }
}

impl Color {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::colors;
        colors::dsl::colors
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its hexadecimal_value.
    ///
    /// # Arguments
    /// * `hexadecimal_value` - The hexadecimal_value of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_hexadecimal_value(
        hexadecimal_value: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::hexadecimal_value.eq(hexadecimal_value))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value, description FROM colors ",
            "WHERE $1 % f_concat_colors_name(name, description) ",
            "ORDER BY similarity($1, f_concat_colors_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value, description FROM colors ",
            "WHERE $1 <% f_concat_colors_name(name, description) ",
            "ORDER BY word_similarity($1, f_concat_colors_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value, description FROM colors ",
            "WHERE $1 <<% f_concat_colors_name(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_colors_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = countries)]
#[diesel(primary_key(id))]
pub struct Country {
    pub id: i32,
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
}

impl From<Country> for web_common::database::tables::Country {
    fn from(item: Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::Country> for Country {
    fn from(item: web_common::database::tables::Country) -> Self {
        Self {
            id: item.id,
            iso: item.iso,
            emoji: item.emoji,
            unicode: item.unicode,
            name: item.name,
        }
    }
}

impl Country {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its emoji.
    ///
    /// # Arguments
    /// * `emoji` - The emoji of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_emoji(
        emoji: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::emoji.eq(emoji))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its iso.
    ///
    /// # Arguments
    /// * `iso` - The iso of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_iso(
        iso: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::iso.eq(iso))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its unicode.
    ///
    /// # Arguments
    /// * `unicode` - The unicode of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_unicode(
        unicode: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::unicode.eq(unicode))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = derived_samples)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a96b10>, foreign_key = parent_sample_id))]
#[diesel(primary_key(parent_sample_id, child_sample_id))]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

impl From<DerivedSample> for web_common::database::tables::DerivedSample {
    fn from(item: DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl From<web_common::database::tables::DerivedSample> for DerivedSample {
    fn from(item: web_common::database::tables::DerivedSample) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl DerivedSample {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::DerivedSampleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(created_by));
        }
        if let Some(parent_sample_id) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id));
        }
        if let Some(child_sample_id) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(child_sample_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::DerivedSampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::derived_samples;
        let mut query = derived_samples::dsl::derived_samples
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(derived_samples::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.parent_sample_id) {
            query = query.filter(derived_samples::dsl::parent_sample_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.child_sample_id) {
            query = query.filter(derived_samples::dsl::child_sample_id.eq(value));
        }
        query
            .order_by(derived_samples::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::derived_samples;
        derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
            .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = document_formats)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<DocumentFormat> for web_common::database::tables::DocumentFormat {
    fn from(item: DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::DocumentFormat> for DocumentFormat {
    fn from(item: web_common::database::tables::DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl DocumentFormat {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::DocumentFormatFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::document_formats;
        let mut query = document_formats::dsl::document_formats
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(document_formats::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(document_formats::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::document_formats;
        document_formats::dsl::document_formats
            .filter(document_formats::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its extension.
    ///
    /// # Arguments
    /// * `extension` - The extension of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_extension(
        extension: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::document_formats;
        document_formats::dsl::document_formats
            .filter(document_formats::dsl::extension.eq(extension))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type, description, icon_id, color_id FROM document_formats ",
            "WHERE $1 % f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type, description, icon_id, color_id FROM document_formats ",
            "WHERE $1 <% f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY word_similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type, description, icon_id, color_id FROM document_formats ",
            "WHERE $1 <<% f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY strict_word_similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = font_awesome_icons)]
#[diesel(primary_key(id))]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl From<FontAwesomeIcon> for web_common::database::tables::FontAwesomeIcon {
    fn from(item: FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
        }
    }
}

impl From<web_common::database::tables::FontAwesomeIcon> for FontAwesomeIcon {
    fn from(item: web_common::database::tables::FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
        }
    }
}

impl FontAwesomeIcon {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::font_awesome_icons;
        font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description FROM font_awesome_icons ",
            "WHERE $1 % f_concat_font_awesome_icons_name(name, description) ",
            "ORDER BY similarity($1, f_concat_font_awesome_icons_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description FROM font_awesome_icons ",
            "WHERE $1 <% f_concat_font_awesome_icons_name(name, description) ",
            "ORDER BY word_similarity($1, f_concat_font_awesome_icons_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description FROM font_awesome_icons ",
            "WHERE $1 <<% f_concat_font_awesome_icons_name(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_font_awesome_icons_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = login_providers)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub client_id_var_name: String,
    pub redirect_uri_var_name: String,
    pub oauth_url: String,
    pub scope: String,
}

impl From<LoginProvider> for web_common::database::tables::LoginProvider {
    fn from(item: LoginProvider) -> Self {
        Self {
            id: item.id,
            name: item.name,
            icon_id: item.icon_id,
            color_id: item.color_id,
            client_id_var_name: item.client_id_var_name,
            redirect_uri_var_name: item.redirect_uri_var_name,
            oauth_url: item.oauth_url,
            scope: item.scope,
        }
    }
}

impl From<web_common::database::tables::LoginProvider> for LoginProvider {
    fn from(item: web_common::database::tables::LoginProvider) -> Self {
        Self {
            id: item.id,
            name: item.name,
            icon_id: item.icon_id,
            color_id: item.color_id,
            client_id_var_name: item.client_id_var_name,
            redirect_uri_var_name: item.redirect_uri_var_name,
            oauth_url: item.oauth_url,
            scope: item.scope,
        }
    }
}

impl LoginProvider {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::LoginProviderFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::login_providers;
        let mut query = login_providers::dsl::login_providers
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(login_providers::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(login_providers::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// # Arguments
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::color_id.eq(color_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// # Arguments
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::name.eq(name))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = materials)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: Option<i32>,
    pub color_id: Option<i32>,
}

impl From<Material> for web_common::database::tables::Material {
    fn from(item: Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::Material> for Material {
    fn from(item: web_common::database::tables::Material) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl Material {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::MaterialFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::materials;
        let mut query = materials::dsl::materials
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(materials::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(materials::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::materials;
        materials::dsl::materials
            .filter(materials::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::materials;
        materials::dsl::materials
            .filter(materials::dsl::name.eq(name))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = notifications)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(primary_key(id))]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: String,
    pub read: bool,
}

impl From<Notification> for web_common::database::tables::Notification {
    fn from(item: Notification) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            operation: item.operation,
            table_name: item.table_name,
            record: item.record,
            read: item.read,
        }
    }
}

impl From<web_common::database::tables::Notification> for Notification {
    fn from(item: web_common::database::tables::Notification) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            operation: item.operation,
            table_name: item.table_name,
            record: item.record,
            read: item.read,
        }
    }
}

impl Notification {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::NotificationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::notifications;
        let mut query = notifications::dsl::notifications
            .into_boxed();
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(notifications::dsl::user_id.eq(user_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::notifications;
        notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = observations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = project_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a966c0>, foreign_key = individual_id))]
#[diesel(primary_key(id))]
pub struct Observation {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub project_id: i32,
    pub individual_id: Option<Uuid>,
    pub notes: Option<String>,
    pub picture: Vec<u8>,
}

impl From<Observation> for web_common::database::tables::Observation {
    fn from(item: Observation) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            individual_id: item.individual_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::Observation> for Observation {
    fn from(item: web_common::database::tables::Observation) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            project_id: item.project_id,
            individual_id: item.individual_id,
            notes: item.notes,
            picture: item.picture,
        }
    }
}

impl Observation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ObservationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(updated_by));
        }
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(project_id));
        }
        if let Some(individual_id) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(individual_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ObservationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::observations;
        let mut query = observations::dsl::observations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(observations::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(observations::dsl::updated_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.project_id) {
            query = query.filter(observations::dsl::project_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.individual_id) {
            query = query.filter(observations::dsl::individual_id.eq(value));
        }
        query
            .order_by(observations::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::observations;
        observations::dsl::observations
            .filter(observations::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = organizations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa22a0>, foreign_key = country_id))]
#[diesel(primary_key(id))]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub country_id: i32,
    pub state_province: Option<String>,
    pub domain: String,
}

impl From<Organization> for web_common::database::tables::Organization {
    fn from(item: Organization) -> Self {
        Self {
            id: item.id,
            name: item.name,
            url: item.url,
            country_id: item.country_id,
            state_province: item.state_province,
            domain: item.domain,
        }
    }
}

impl From<web_common::database::tables::Organization> for Organization {
    fn from(item: web_common::database::tables::Organization) -> Self {
        Self {
            id: item.id,
            name: item.name,
            url: item.url,
            country_id: item.country_id,
            state_province: item.state_province,
            domain: item.domain,
        }
    }
}

impl Organization {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::OrganizationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::organizations;
        let mut query = organizations::dsl::organizations
            .into_boxed();
        if let Some(country_id) = filter.and_then(|f| f.country_id) {
            query = query.filter(organizations::dsl::country_id.eq(country_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its domain.
    ///
    /// # Arguments
    /// * `domain` - The domain of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_domain(
        domain: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::domain.eq(domain))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name, country_id and state_province.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `country_id` - The country_id of the struct to get.
    /// * `state_province` - The state_province of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name_and_country_id_and_state_province(
        name: &str,
        country_id: &i32,
        state_province: Option<&str>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::name.eq(name))
            .filter(organizations::dsl::country_id.eq(country_id))
            .filter(organizations::dsl::state_province.eq(state_province))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its url.
    ///
    /// # Arguments
    /// * `url` - The url of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_url(
        url: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::url.eq(url))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = project_states)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
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
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectStateFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::project_states;
        let mut query = project_states::dsl::project_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(project_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(project_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// # Arguments
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// # Arguments
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM project_states ",
            "WHERE $1 % f_concat_project_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM project_states ",
            "WHERE $1 <% f_concat_project_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM project_states ",
            "WHERE $1 <<% f_concat_project_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa3e30>, foreign_key = state_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = parent_project_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

impl From<Project> for web_common::database::tables::Project {
    fn from(item: Project) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            public: item.public,
            state_id: item.state_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_project_id: item.parent_project_id,
            budget: item.budget,
            expenses: item.expenses,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            expected_end_date: item.expected_end_date,
            end_date: item.end_date,
        }
    }
}

impl From<web_common::database::tables::Project> for Project {
    fn from(item: web_common::database::tables::Project) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            public: item.public,
            state_id: item.state_id,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_project_id: item.parent_project_id,
            budget: item.budget,
            expenses: item.expenses,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            expected_end_date: item.expected_end_date,
            end_date: item.end_date,
        }
    }
}

impl Project {
    /// Check whether the user has a role with a role_id less than or equal to the provided role_id.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `role_id` - The role_id to check against.
    /// * `connection` - The connection to the database.
    ///
    pub fn has_role_by_id(
        id: i32,
        author_user_id: i32,
        role_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        diesel::select(diesel::dsl::exists(projects::dsl::projects
            .filter(projects::dsl::id.eq(id))
           .filter(projects::dsl::created_by.eq(author_user_id))
            .or_filter(
               projects::dsl::id.eq(id)
                   .and(projects::dsl::id.eq_any(
                       projects_users_roles::table
                           .select(projects_users_roles::dsl::table_id)
                           .filter(projects_users_roles::dsl::user_id.eq(author_user_id)
                           .and(projects_users_roles::dsl::role_id.le(role_id)),
                    )),
               )
         )
                    .or_filter(
                       projects::dsl::id.eq(id)
                           .and(projects::dsl::id.eq_any(
                               projects_teams_roles::table
                                   .select(projects_teams_roles::dsl::table_id)
                                   .filter(projects_teams_roles::dsl::role_id.le(role_id))
                                   .inner_join(teams_users_roles::table.on(
                                       projects_teams_roles::dsl::team_id.eq(teams_users_roles::dsl::table_id)
                                           .and(teams_users_roles::dsl::user_id.eq(author_user_id))
                                           .and(teams_users_roles::dsl::role_id.le(role_id)),
                                   )),
                              ))
                       )
            ))
         .get_result::<bool>(connection)
    }
    /// Check whether the user is a Viewer (role_id >= 3).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_viewer(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_viewer_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is a Viewer (role_id >= 3) for the provided primary key(s).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_viewer_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            3,
            connection,
        )
    }
    /// Check whether the user is an Editor (role_id >= 2).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_editor(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_editor_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is an Editor (role_id >= 2).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_editor_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            2,
            connection,
        )
    }
    /// Check whether the user is an Admin (role_id == 1).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_admin(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is an Admin (role_id == 1).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_admin_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            1,
            connection,
        )
    }
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the editable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_editables(
        filter: Option<&web_common::database::ProjectFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
        let mut query = projects::dsl::projects
           .filter(projects::dsl::created_by.eq(author_user_id))
            .or_filter(
               projects::dsl::id.eq_any(
                   projects_users_roles::table
                       .select(projects_users_roles::dsl::table_id)
                       .filter(projects_users_roles::dsl::user_id.eq(author_user_id)
                       .and(projects_users_roles::dsl::role_id.le(2)),
               )),
            )
                .or_filter(
                   projects::dsl::id.eq_any(
                       projects_teams_roles::table
                           .select(projects_teams_roles::dsl::table_id)
                           .filter(projects_teams_roles::dsl::role_id.le(2))
                           .inner_join(teams_users_roles::table.on(
                               projects_teams_roles::dsl::team_id.eq(teams_users_roles::dsl::table_id)
                                   .and(teams_users_roles::dsl::user_id.eq(author_user_id))
                                   .and(teams_users_roles::dsl::role_id.le(2)),
                           )),
                   ),
            )
            .into_boxed();
        if let Some(state_id) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(state_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(color_id));
        }
        if let Some(parent_project_id) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(parent_project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
        let mut query = projects::dsl::projects
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.state_id) {
            query = query.filter(projects::dsl::state_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.icon_id) {
            query = query.filter(projects::dsl::icon_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.color_id) {
            query = query.filter(projects::dsl::color_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.parent_project_id) {
            query = query.filter(projects::dsl::parent_project_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(projects::dsl::updated_by.eq(value));
        }
        query
            .order_by(projects::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        if !Self::is_admin_by_id(id, author_user_id, connection)? {
            return Err(diesel::result::Error::NotFound);
        }
        diesel::delete(projects::dsl::projects
            .filter(projects::dsl::id.eq(id))
        ).execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects;
        projects::dsl::projects
            .filter(projects::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects;
        projects::dsl::projects
            .filter(projects::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 % f_concat_projects_name_description(name, description) AND can_view_projects($3, projects.id) ",
            "ORDER BY similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 % f_concat_projects_name_description(name, description) AND can_edit_projects($3, projects.id) ",
            "ORDER BY similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <% f_concat_projects_name_description(name, description) AND can_view_projects($3, projects.id) ",
            "ORDER BY word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <% f_concat_projects_name_description(name, description) AND can_edit_projects($3, projects.id) ",
            "ORDER BY word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <<% f_concat_projects_name_description(name, description) AND can_view_projects($3, projects.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, icon_id, color_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <<% f_concat_projects_name_description(name, description) AND can_edit_projects($3, projects.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_role_invitations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = team_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRoleInvitation> for web_common::database::tables::ProjectsTeamsRoleInvitation {
    fn from(item: ProjectsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRoleInvitation> for ProjectsTeamsRoleInvitation {
    fn from(item: web_common::database::tables::ProjectsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRoleInvitation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsTeamsRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsTeamsRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_role_invitations;
        let mut query = projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_invitations::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_invitations::dsl::team_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_invitations::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_invitations::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, team_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_teams_role_invitations;
        projects_teams_role_invitations::dsl::projects_teams_role_invitations
            .filter(projects_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_invitations::dsl::team_id.eq(team_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_role_requests)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = team_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRoleRequest {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRoleRequest> for web_common::database::tables::ProjectsTeamsRoleRequest {
    fn from(item: ProjectsTeamsRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRoleRequest> for ProjectsTeamsRoleRequest {
    fn from(item: web_common::database::tables::ProjectsTeamsRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRoleRequest {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsTeamsRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsTeamsRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_role_requests;
        let mut query = projects_teams_role_requests::dsl::projects_teams_role_requests
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_role_requests::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_role_requests::dsl::team_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_role_requests::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_role_requests::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_teams_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, team_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_teams_role_requests;
        projects_teams_role_requests::dsl::projects_teams_role_requests
            .filter(projects_teams_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_teams_role_requests::dsl::team_id.eq(team_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_teams_roles)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = team_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct ProjectsTeamsRole {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsTeamsRole> for web_common::database::tables::ProjectsTeamsRole {
    fn from(item: ProjectsTeamsRole) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsTeamsRole> for ProjectsTeamsRole {
    fn from(item: web_common::database::tables::ProjectsTeamsRole) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsTeamsRole {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsTeamsRoleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsTeamsRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_teams_roles;
        let mut query = projects_teams_roles::dsl::projects_teams_roles
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_teams_roles::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.team_id) {
            query = query.filter(projects_teams_roles::dsl::team_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_teams_roles::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_teams_roles::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_teams_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, team_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_teams_roles;
        projects_teams_roles::dsl::projects_teams_roles
            .filter(projects_teams_roles::dsl::table_id.eq(table_id))
            .filter(projects_teams_roles::dsl::team_id.eq(team_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_role_invitations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRoleInvitation> for web_common::database::tables::ProjectsUsersRoleInvitation {
    fn from(item: ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRoleInvitation> for ProjectsUsersRoleInvitation {
    fn from(item: web_common::database::tables::ProjectsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRoleInvitation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsUsersRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_role_invitations;
        let mut query = projects_users_role_invitations::dsl::projects_users_role_invitations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_invitations::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_invitations::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_invitations::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_invitations::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_users_role_invitations;
        projects_users_role_invitations::dsl::projects_users_role_invitations
            .filter(projects_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(projects_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_role_requests)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRoleRequest> for web_common::database::tables::ProjectsUsersRoleRequest {
    fn from(item: ProjectsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRoleRequest> for ProjectsUsersRoleRequest {
    fn from(item: web_common::database::tables::ProjectsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRoleRequest {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsUsersRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_role_requests;
        let mut query = projects_users_role_requests::dsl::projects_users_role_requests
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_role_requests::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_role_requests::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_role_requests::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_role_requests::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_users_role_requests;
        projects_users_role_requests::dsl::projects_users_role_requests
            .filter(projects_users_role_requests::dsl::table_id.eq(table_id))
            .filter(projects_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_roles)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct ProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<ProjectsUsersRole> for web_common::database::tables::ProjectsUsersRole {
    fn from(item: ProjectsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::ProjectsUsersRole> for ProjectsUsersRole {
    fn from(item: web_common::database::tables::ProjectsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl ProjectsUsersRole {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::ProjectsUsersRoleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::ProjectsUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        let mut query = projects_users_roles::dsl::projects_users_roles
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(projects_users_roles::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(projects_users_roles::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(projects_users_roles::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(projects_users_roles::dsl::created_by.eq(value));
        }
        query
            .order_by(projects_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        projects_users_roles::dsl::projects_users_roles
            .filter(projects_users_roles::dsl::table_id.eq(table_id))
            .filter(projects_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = roles)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<Role> for web_common::database::tables::Role {
    fn from(item: Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::Role> for Role {
    fn from(item: web_common::database::tables::Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl Role {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::RoleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::roles;
        let mut query = roles::dsl::roles
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(roles::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(roles::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// # Arguments
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::color_id.eq(color_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its description.
    ///
    /// # Arguments
    /// * `description` - The description of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_description(
        description: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::description.eq(description))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// # Arguments
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM roles ",
            "WHERE $1 % f_concat_roles_name(name, description) ",
            "ORDER BY similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM roles ",
            "WHERE $1 <% f_concat_roles_name(name, description) ",
            "ORDER BY word_similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM roles ",
            "WHERE $1 <<% f_concat_roles_name(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a96b10>, foreign_key = sample_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa1760>, foreign_key = taxon_id))]
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
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampleBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SampleBioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
        let mut query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.sample_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(value));
        }
        query
            .order_by(sample_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( sample_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( sample_id, taxon_id ): ( Uuid, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::sample_id.eq(sample_id))
            .filter(sample_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_container_categories)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa30b0>, foreign_key = material_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct SampleContainerCategory {
    pub id: i32,
    pub name: String,
    pub volume: f64,
    pub unit: String,
    pub material_id: i32,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<SampleContainerCategory> for web_common::database::tables::SampleContainerCategory {
    fn from(item: SampleContainerCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            volume: item.volume,
            unit: item.unit,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::SampleContainerCategory> for SampleContainerCategory {
    fn from(item: web_common::database::tables::SampleContainerCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            volume: item.volume,
            unit: item.unit,
            material_id: item.material_id,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl SampleContainerCategory {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampleContainerCategoryFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_container_categories;
        let mut query = sample_container_categories::dsl::sample_container_categories
            .into_boxed();
        if let Some(material_id) = filter.and_then(|f| f.material_id) {
            query = query.filter(sample_container_categories::dsl::material_id.eq(material_id));
        }
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_container_categories::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_container_categories::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_container_categories;
        sample_container_categories::dsl::sample_container_categories
            .filter(sample_container_categories::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, volume, unit, material_id, description, icon_id, color_id FROM sample_container_categories ",
            "WHERE $1 % f_concat_sample_container_categories_brand(name, description) ",
            "ORDER BY similarity($1, f_concat_sample_container_categories_brand(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, volume, unit, material_id, description, icon_id, color_id FROM sample_container_categories ",
            "WHERE $1 <% f_concat_sample_container_categories_brand(name, description) ",
            "ORDER BY word_similarity($1, f_concat_sample_container_categories_brand(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, volume, unit, material_id, description, icon_id, color_id FROM sample_container_categories ",
            "WHERE $1 <<% f_concat_sample_container_categories_brand(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sample_container_categories_brand(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_containers)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a95b80>, foreign_key = category_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SampleContainer {
    pub id: i32,
    pub barcode: String,
    pub category_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<SampleContainer> for web_common::database::tables::SampleContainer {
    fn from(item: SampleContainer) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::SampleContainer> for SampleContainer {
    fn from(item: web_common::database::tables::SampleContainer) -> Self {
        Self {
            id: item.id,
            barcode: item.barcode,
            category_id: item.category_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl SampleContainer {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampleContainerFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(category_id) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(category_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SampleContainerFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_containers;
        let mut query = sample_containers::dsl::sample_containers
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.category_id) {
            query = query.filter(sample_containers::dsl::category_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(sample_containers::dsl::created_by.eq(value));
        }
        query
            .order_by(sample_containers::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_containers;
        sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its barcode.
    ///
    /// # Arguments
    /// * `barcode` - The barcode of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_barcode(
        barcode: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_containers;
        sample_containers::dsl::sample_containers
            .filter(sample_containers::dsl::barcode.eq(barcode))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, barcode, category_id, created_by, created_at FROM sample_containers ",
            "WHERE $1 % barcode AND can_view_sample_containers($3, sample_containers.id) ",
            "ORDER BY similarity($1, barcode) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, barcode, category_id, created_by, created_at FROM sample_containers ",
            "WHERE $1 <% barcode AND can_view_sample_containers($3, sample_containers.id) ",
            "ORDER BY word_similarity($1, barcode) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, barcode, category_id, created_by, created_at FROM sample_containers ",
            "WHERE $1 <<% barcode AND can_view_sample_containers($3, sample_containers.id) ",
            "ORDER BY strict_word_similarity($1, barcode) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_states)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<SampleState> for web_common::database::tables::SampleState {
    fn from(item: SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::SampleState> for SampleState {
    fn from(item: web_common::database::tables::SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl SampleState {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampleStateFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_states;
        let mut query = sample_states::dsl::sample_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(sample_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(sample_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_states;
        sample_states::dsl::sample_states
            .filter(sample_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// # Arguments
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_states;
        sample_states::dsl::sample_states
            .filter(sample_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// # Arguments
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sample_states;
        sample_states::dsl::sample_states
            .filter(sample_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM sample_states ",
            "WHERE $1 % f_concat_sample_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM sample_states ",
            "WHERE $1 <% f_concat_sample_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM sample_states ",
            "WHERE $1 <<% f_concat_sample_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a966c0>, foreign_key = sampled_individual_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa1760>, foreign_key = taxon_id))]
#[diesel(primary_key(sampled_individual_id, taxon_id))]
pub struct SampledIndividualBioOttTaxonItem {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampledIndividualBioOttTaxonItem> for web_common::database::tables::SampledIndividualBioOttTaxonItem {
    fn from(item: SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            created_by: item.created_by,
            created_at: item.created_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampledIndividualBioOttTaxonItem> for SampledIndividualBioOttTaxonItem {
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
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampledIndividualBioOttTaxonItemFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_individual_id) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id));
        }
        if let Some(taxon_id) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SampledIndividualBioOttTaxonItemFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let mut query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.sampled_individual_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.taxon_id) {
            query = query.filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(value));
        }
        query
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( sampled_individual_id, taxon_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( sampled_individual_id, taxon_id ): ( Uuid, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_id.eq(sampled_individual_id))
            .filter(sampled_individual_bio_ott_taxon_items::dsl::taxon_id.eq(taxon_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individuals)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a94140>, foreign_key = project_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct SampledIndividual {
    pub id: Uuid,
    pub notes: Option<String>,
    pub barcode: Option<String>,
    pub project_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub picture: Vec<u8>,
}

impl From<SampledIndividual> for web_common::database::tables::SampledIndividual {
    fn from(item: SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            barcode: item.barcode,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl From<web_common::database::tables::SampledIndividual> for SampledIndividual {
    fn from(item: web_common::database::tables::SampledIndividual) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            barcode: item.barcode,
            project_id: item.project_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            picture: item.picture,
        }
    }
}

impl SampledIndividual {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampledIndividualFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(project_id) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(project_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SampledIndividualFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        let mut query = sampled_individuals::dsl::sampled_individuals
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.project_id) {
            query = query.filter(sampled_individuals::dsl::project_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(sampled_individuals::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(sampled_individuals::dsl::updated_by.eq(value));
        }
        query
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, notes, barcode, project_id, created_by, created_at, updated_by, updated_at, picture FROM sampled_individuals ",
            "WHERE $1 % f_concat_sampled_individuals_notes_barcode(notes, barcode) AND can_view_sampled_individuals($3, sampled_individuals.id) ",
            "ORDER BY similarity($1, f_concat_sampled_individuals_notes_barcode(notes, barcode)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, notes, barcode, project_id, created_by, created_at, updated_by, updated_at, picture FROM sampled_individuals ",
            "WHERE $1 <% f_concat_sampled_individuals_notes_barcode(notes, barcode) AND can_view_sampled_individuals($3, sampled_individuals.id) ",
            "ORDER BY word_similarity($1, f_concat_sampled_individuals_notes_barcode(notes, barcode)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, notes, barcode, project_id, created_by, created_at, updated_by, updated_at, picture FROM sampled_individuals ",
            "WHERE $1 <<% f_concat_sampled_individuals_notes_barcode(notes, barcode) AND can_view_sampled_individuals($3, sampled_individuals.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_sampled_individuals_notes_barcode(notes, barcode)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = samples)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a95fd0>, foreign_key = container_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a96240>, foreign_key = state))]
#[diesel(primary_key(id))]
pub struct Sample {
    pub id: Uuid,
    pub container_id: i32,
    pub notes: Option<String>,
    pub created_by: i32,
    pub sampled_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub state: i32,
}

impl From<Sample> for web_common::database::tables::Sample {
    fn from(item: Sample) -> Self {
        Self {
            id: item.id,
            container_id: item.container_id,
            notes: item.notes,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            state: item.state,
        }
    }
}

impl From<web_common::database::tables::Sample> for Sample {
    fn from(item: web_common::database::tables::Sample) -> Self {
        Self {
            id: item.id,
            container_id: item.container_id,
            notes: item.notes,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            state: item.state,
        }
    }
}

impl Sample {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SampleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(container_id) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(container_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(created_by));
        }
        if let Some(sampled_by) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(sampled_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(updated_by));
        }
        if let Some(state) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(state));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SampleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::samples;
        let mut query = samples::dsl::samples
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.container_id) {
            query = query.filter(samples::dsl::container_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(samples::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.sampled_by) {
            query = query.filter(samples::dsl::sampled_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(samples::dsl::updated_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.state) {
            query = query.filter(samples::dsl::state.eq(value));
        }
        query
            .order_by(samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::samples;
        samples::dsl::samples
            .filter(samples::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97170>, foreign_key = spectra_collection_id))]
#[diesel(primary_key(id))]
pub struct Spectra {
    pub id: i32,
    pub notes: Option<String>,
    pub spectra_collection_id: i32,
}

impl From<Spectra> for web_common::database::tables::Spectra {
    fn from(item: Spectra) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            spectra_collection_id: item.spectra_collection_id,
        }
    }
}

impl From<web_common::database::tables::Spectra> for Spectra {
    fn from(item: web_common::database::tables::Spectra) -> Self {
        Self {
            id: item.id,
            notes: item.notes,
            spectra_collection_id: item.spectra_collection_id,
        }
    }
}

impl Spectra {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SpectraFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra;
        let mut query = spectra::dsl::spectra
            .into_boxed();
        if let Some(spectra_collection_id) = filter.and_then(|f| f.spectra_collection_id) {
            query = query.filter(spectra::dsl::spectra_collection_id.eq(spectra_collection_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::spectra;
        spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra_collections)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a96b10>, foreign_key = sample_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
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
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::SpectraCollectionFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
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
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::SpectraCollectionFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra_collections;
        let mut query = spectra_collections::dsl::spectra_collections
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.sample_id) {
            query = query.filter(spectra_collections::dsl::sample_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(spectra_collections::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(spectra_collections::dsl::updated_by.eq(value));
        }
        query
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::spectra_collections;
        spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = team_states)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(primary_key(id))]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
}

impl From<TeamState> for web_common::database::tables::TeamState {
    fn from(item: TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl From<web_common::database::tables::TeamState> for TeamState {
    fn from(item: web_common::database::tables::TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
        }
    }
}

impl TeamState {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamStateFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::team_states;
        let mut query = team_states::dsl::team_states
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(team_states::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(team_states::dsl::color_id.eq(color_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its color_id.
    ///
    /// # Arguments
    /// * `color_id` - The color_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_color_id(
        color_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::color_id.eq(color_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its icon_id.
    ///
    /// # Arguments
    /// * `icon_id` - The icon_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_icon_id(
        icon_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::icon_id.eq(icon_id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM team_states ",
            "WHERE $1 % f_concat_team_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM team_states ",
            "WHERE $1 <% f_concat_team_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id FROM team_states ",
            "WHERE $1 <<% f_concat_team_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2ab0>, foreign_key = icon_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2030>, foreign_key = color_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = parent_team_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(id))]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub icon_id: i32,
    pub color_id: i32,
    pub parent_team_id: Option<i32>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<Team> for web_common::database::tables::Team {
    fn from(item: Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::Team> for Team {
    fn from(item: web_common::database::tables::Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon_id: item.icon_id,
            color_id: item.color_id,
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Team {
    /// Check whether the user has a role with a role_id less than or equal to the provided role_id.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `role_id` - The role_id to check against.
    /// * `connection` - The connection to the database.
    ///
    pub fn has_role_by_id(
        id: i32,
        author_user_id: i32,
        role_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        diesel::select(diesel::dsl::exists(teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
           .filter(teams::dsl::created_by.eq(author_user_id))
            .or_filter(
               teams::dsl::id.eq(id)
                   .and(teams::dsl::id.eq_any(
                       teams_users_roles::table
                           .select(teams_users_roles::dsl::table_id)
                           .filter(teams_users_roles::dsl::user_id.eq(author_user_id)
                           .and(teams_users_roles::dsl::role_id.le(role_id)),
                    )),
               )
         )
            ))
         .get_result::<bool>(connection)
    }
    /// Check whether the user is a Viewer (role_id >= 3).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_viewer(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_viewer_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is a Viewer (role_id >= 3) for the provided primary key(s).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_viewer_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            3,
            connection,
        )
    }
    /// Check whether the user is an Editor (role_id >= 2).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_editor(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_editor_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is an Editor (role_id >= 2).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_editor_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            2,
            connection,
        )
    }
    /// Check whether the user is an Admin (role_id == 1).
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_admin(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::is_admin_by_id(
            self.id,
            author_user_id,
            connection,
        )
    }
    /// Check whether the user is an Admin (role_id == 1).
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user to check.
    /// * `connection` - The connection to the database.
    ///
    pub fn is_admin_by_id(
        id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<bool, diesel::result::Error> {
        Self::has_role_by_id(
            id,
            author_user_id,
            1,
            connection,
        )
    }
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the editable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_editables(
        filter: Option<&web_common::database::TeamFilter>,
        author_user_id: i32,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
        let mut query = teams::dsl::teams
           .filter(teams::dsl::created_by.eq(author_user_id))
            .or_filter(
               teams::dsl::id.eq_any(
                   teams_users_roles::table
                       .select(teams_users_roles::dsl::table_id)
                       .filter(teams_users_roles::dsl::user_id.eq(author_user_id)
                       .and(teams_users_roles::dsl::role_id.le(2)),
               )),
            )
            .into_boxed();
        if let Some(icon_id) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(icon_id));
        }
        if let Some(color_id) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(color_id));
        }
        if let Some(parent_team_id) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(parent_team_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(created_by));
        }
        if let Some(updated_by) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(updated_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::TeamFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
        let mut query = teams::dsl::teams
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.icon_id) {
            query = query.filter(teams::dsl::icon_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.color_id) {
            query = query.filter(teams::dsl::color_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.parent_team_id) {
            query = query.filter(teams::dsl::parent_team_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.updated_by) {
            query = query.filter(teams::dsl::updated_by.eq(value));
        }
        query
            .order_by(teams::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, author_user_id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `author_user_id` - The ID of the user who is deleting the struct.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        author_user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        if !Self::is_admin_by_id(id, author_user_id, connection)? {
            return Err(diesel::result::Error::NotFound);
        }
        diesel::delete(teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
        ).execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams;
        teams::dsl::teams
            .filter(teams::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams;
        teams::dsl::teams
            .filter(teams::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 % f_concat_teams_name_description(name, description) AND can_view_teams($3, teams.id) ",
            "ORDER BY similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 % f_concat_teams_name_description(name, description) AND can_edit_teams($3, teams.id) ",
            "ORDER BY similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <% f_concat_teams_name_description(name, description) AND can_view_teams($3, teams.id) ",
            "ORDER BY word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <% f_concat_teams_name_description(name, description) AND can_edit_teams($3, teams.id) ",
            "ORDER BY word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <<% f_concat_teams_name_description(name, description) AND can_view_teams($3, teams.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the editable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_editables(
       author_user_id: i32,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_editables(author_user_id, None, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, icon_id, color_id, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <<% f_concat_teams_name_description(name, description) AND can_edit_teams($3, teams.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Integer, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_teams_role_invitations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(primary_key(table_id, team_id))]
pub struct TeamsTeamsRoleInvitation {
    pub table_id: i32,
    pub team_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsTeamsRoleInvitation> for web_common::database::tables::TeamsTeamsRoleInvitation {
    fn from(item: TeamsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsTeamsRoleInvitation> for TeamsTeamsRoleInvitation {
    fn from(item: web_common::database::tables::TeamsTeamsRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            team_id: item.team_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsTeamsRoleInvitation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamsTeamsRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(team_id) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(team_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::TeamsTeamsRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_teams_role_invitations;
        let mut query = teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_teams_role_invitations::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.team_id) {
            query = query.filter(teams_teams_role_invitations::dsl::team_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_teams_role_invitations::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_teams_role_invitations::dsl::created_by.eq(value));
        }
        query
            .order_by(teams_teams_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, team_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, team_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams_teams_role_invitations;
        teams_teams_role_invitations::dsl::teams_teams_role_invitations
            .filter(teams_teams_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_teams_role_invitations::dsl::team_id.eq(team_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_role_invitations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRoleInvitation> for web_common::database::tables::TeamsUsersRoleInvitation {
    fn from(item: TeamsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRoleInvitation> for TeamsUsersRoleInvitation {
    fn from(item: web_common::database::tables::TeamsUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRoleInvitation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamsUsersRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::TeamsUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_role_invitations;
        let mut query = teams_users_role_invitations::dsl::teams_users_role_invitations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_invitations::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_invitations::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_invitations::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_invitations::dsl::created_by.eq(value));
        }
        query
            .order_by(teams_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams_users_role_invitations;
        teams_users_role_invitations::dsl::teams_users_role_invitations
            .filter(teams_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(teams_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_role_requests)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRoleRequest> for web_common::database::tables::TeamsUsersRoleRequest {
    fn from(item: TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRoleRequest> for TeamsUsersRoleRequest {
    fn from(item: web_common::database::tables::TeamsUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRoleRequest {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamsUsersRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::TeamsUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_role_requests;
        let mut query = teams_users_role_requests::dsl::teams_users_role_requests
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_role_requests::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_role_requests::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_role_requests::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_role_requests::dsl::created_by.eq(value));
        }
        query
            .order_by(teams_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams_users_role_requests;
        teams_users_role_requests::dsl::teams_users_role_requests
            .filter(teams_users_role_requests::dsl::table_id.eq(table_id))
            .filter(teams_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams_users_roles)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a97770>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = user_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct TeamsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<TeamsUsersRole> for web_common::database::tables::TeamsUsersRole {
    fn from(item: TeamsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::TeamsUsersRole> for TeamsUsersRole {
    fn from(item: web_common::database::tables::TeamsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl TeamsUsersRole {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::TeamsUsersRoleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::TeamsUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams_users_roles;
        let mut query = teams_users_roles::dsl::teams_users_roles
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(teams_users_roles::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(teams_users_roles::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(teams_users_roles::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(teams_users_roles::dsl::created_by.eq(value));
        }
        query
            .order_by(teams_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::teams_users_roles;
        teams_users_roles::dsl::teams_users_roles
            .filter(teams_users_roles::dsl::table_id.eq(table_id))
            .filter(teams_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = units)]
#[diesel(primary_key(id))]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub symbol: String,
}

impl From<Unit> for web_common::database::tables::Unit {
    fn from(item: Unit) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            symbol: item.symbol,
        }
    }
}

impl From<web_common::database::tables::Unit> for Unit {
    fn from(item: web_common::database::tables::Unit) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            symbol: item.symbol,
        }
    }
}

impl Unit {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::units;
        units::dsl::units
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::units;
        units::dsl::units
            .filter(units::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its name.
    ///
    /// # Arguments
    /// * `name` - The name of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_name(
        name: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::units;
        units::dsl::units
            .filter(units::dsl::name.eq(name))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 % f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 <% f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY word_similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 <<% f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY strict_word_similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = user_emails)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = created_by))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838aa2c90>, foreign_key = login_provider_id))]
#[diesel(primary_key(id))]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub login_provider_id: i32,
    pub primary_email: bool,
}

impl From<UserEmail> for web_common::database::tables::UserEmail {
    fn from(item: UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl From<web_common::database::tables::UserEmail> for UserEmail {
    fn from(item: web_common::database::tables::UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            created_by: item.created_by,
            created_at: item.created_at,
            login_provider_id: item.login_provider_id,
            primary_email: item.primary_email,
        }
    }
}

impl UserEmail {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::UserEmailFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(created_by));
        }
        if let Some(login_provider_id) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(login_provider_id));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::UserEmailFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::user_emails;
        let mut query = user_emails::dsl::user_emails
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(user_emails::dsl::created_by.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.login_provider_id) {
            query = query.filter(user_emails::dsl::login_provider_id.eq(value));
        }
        query
            .order_by(user_emails::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its email and login_provider_id.
    ///
    /// # Arguments
    /// * `email` - The email of the struct to get.
    /// * `login_provider_id` - The login_provider_id of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_email_and_login_provider_id(
        email: &str,
        login_provider_id: &i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::email.eq(email))
            .filter(user_emails::dsl::login_provider_id.eq(login_provider_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub description: Option<String>,
    pub profile_picture: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for web_common::database::tables::User {
    fn from(item: User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            profile_picture: item.profile_picture,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::User> for User {
    fn from(item: web_common::database::tables::User) -> Self {
        Self {
            id: item.id,
            first_name: item.first_name,
            middle_name: item.middle_name,
            last_name: item.last_name,
            description: item.description,
            profile_picture: item.profile_picture,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl User {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users;
        users::dsl::users
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users;
        users::dsl::users
            .order_by(users::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the viewable structs by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, description, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 % f_concat_users_name(first_name, middle_name, last_name) AND can_view_users($3, users.id) ",
            "ORDER BY similarity($1, f_concat_users_name(first_name, middle_name, last_name)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, description, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 <% f_concat_users_name(first_name, middle_name, last_name) AND can_view_users($3, users.id) ",
            "ORDER BY word_similarity($1, f_concat_users_name(first_name, middle_name, last_name)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
    /// Search for the viewable structs by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search_viewables(
       author_user_id: Option<i32>,
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all_viewables(author_user_id, Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, description, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 <<% f_concat_users_name(first_name, middle_name, last_name) AND can_view_users($3, users.id) ",
            "ORDER BY strict_word_similarity($1, f_concat_users_name(first_name, middle_name, last_name)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .bind::<diesel::sql_types::Nullable<diesel::sql_types::Integer>, _>(author_user_id)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_role_invitations)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRoleInvitation {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRoleInvitation> for web_common::database::tables::UsersUsersRoleInvitation {
    fn from(item: UsersUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRoleInvitation> for UsersUsersRoleInvitation {
    fn from(item: web_common::database::tables::UsersUsersRoleInvitation) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRoleInvitation {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::UsersUsersRoleInvitationFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::UsersUsersRoleInvitationFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_role_invitations;
        let mut query = users_users_role_invitations::dsl::users_users_role_invitations
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_invitations::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_invitations::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_invitations::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_invitations::dsl::created_by.eq(value));
        }
        query
            .order_by(users_users_role_invitations::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users_users_role_invitations;
        users_users_role_invitations::dsl::users_users_role_invitations
            .filter(users_users_role_invitations::dsl::table_id.eq(table_id))
            .filter(users_users_role_invitations::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_role_requests)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRoleRequest {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRoleRequest> for web_common::database::tables::UsersUsersRoleRequest {
    fn from(item: UsersUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRoleRequest> for UsersUsersRoleRequest {
    fn from(item: web_common::database::tables::UsersUsersRoleRequest) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRoleRequest {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::UsersUsersRoleRequestFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::UsersUsersRoleRequestFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_role_requests;
        let mut query = users_users_role_requests::dsl::users_users_role_requests
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_role_requests::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_role_requests::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_role_requests::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_role_requests::dsl::created_by.eq(value));
        }
        query
            .order_by(users_users_role_requests::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users_users_role_requests;
        users_users_role_requests::dsl::users_users_role_requests
            .filter(users_users_role_requests::dsl::table_id.eq(table_id))
            .filter(users_users_role_requests::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Associations, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users_users_roles)]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a68290>, foreign_key = table_id))]
#[diesel(belongs_to(<constraint_checkers.struct_metadata.StructMetadata object at 0x709838a956d0>, foreign_key = role_id))]
#[diesel(primary_key(table_id, user_id))]
pub struct UsersUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
}

impl From<UsersUsersRole> for web_common::database::tables::UsersUsersRole {
    fn from(item: UsersUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl From<web_common::database::tables::UsersUsersRole> for UsersUsersRole {
    fn from(item: web_common::database::tables::UsersUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
        }
    }
}

impl UsersUsersRole {
    /// Get all of the viewable structs from the database.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `author_user_id` - The ID of the user who is performing the search.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_viewables(
        filter: Option<&web_common::database::UsersUsersRoleFilter>,
        author_user_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(table_id) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(table_id));
        }
        if let Some(user_id) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(user_id));
        }
        if let Some(role_id) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(role_id));
        }
        if let Some(created_by) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(created_by));
        }
        query
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by update time.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to apply to the query.
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_update(
        filter: Option<&web_common::database::UsersUsersRoleFilter>,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users_users_roles;
        let mut query = users_users_roles::dsl::users_users_roles
            .into_boxed();
        if let Some(value) = filter.and_then(|f| f.table_id) {
            query = query.filter(users_users_roles::dsl::table_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.user_id) {
            query = query.filter(users_users_roles::dsl::user_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.role_id) {
            query = query.filter(users_users_roles::dsl::role_id.eq(value));
        }
        if let Some(value) = filter.and_then(|f| f.created_by) {
            query = query.filter(users_users_roles::dsl::created_by.eq(value));
        }
        query
            .order_by(users_users_roles::dsl::created_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id ): ( i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::users_users_roles;
        users_users_roles::dsl::users_users_roles
            .filter(users_users_roles::dsl::table_id.eq(table_id))
            .filter(users_users_roles::dsl::user_id.eq(user_id))
            .first::<Self>(connection)
    }
}
