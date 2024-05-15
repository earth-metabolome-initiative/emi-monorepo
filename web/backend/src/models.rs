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

#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_ranks)]
pub struct BioOttRank {
    pub id: i32,
    pub name: String,
    pub font_awesome_icon_id: i32,
}

impl From<BioOttRank> for web_common::database::tables::BioOttRank {
    fn from(item: BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            font_awesome_icon_id: item.font_awesome_icon_id,
        }
    }
}

impl From<web_common::database::tables::BioOttRank> for BioOttRank {
    fn from(item: web_common::database::tables::BioOttRank) -> Self {
        Self {
            id: item.id,
            name: item.name,
            font_awesome_icon_id: item.font_awesome_icon_id,
        }
    }
}

impl BioOttRank {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_ranks;
       bio_ott_ranks::dsl::bio_ott_ranks
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::id.eq(id))
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, font_awesome_icon_id FROM bio_ott_ranks ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, font_awesome_icon_id FROM bio_ott_ranks ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, font_awesome_icon_id FROM bio_ott_ranks ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = bio_ott_taxon_items)]
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
    pub font_awesome_icon_id: i32,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
        }
    }
}

impl BioOttTaxonItem {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
       bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::id.eq(id))
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
        ott_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
        bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::ott_id.eq(ott_id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, ott_id, ott_rank_id, wikidata_id, ncbi_id, gbif_id, irmng_id, worms_id, domain_id, kingdom_id, phylum_id, class_id, order_id, family_id, genus_id, parent_id, font_awesome_icon_id, color_id FROM bio_ott_taxon_items ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = colors)]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hexadecimal_value: String,
}

impl From<Color> for web_common::database::tables::Color {
    fn from(item: Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
        }
    }
}

impl From<web_common::database::tables::Color> for Color {
    fn from(item: web_common::database::tables::Color) -> Self {
        Self {
            id: item.id,
            name: item.name,
            hexadecimal_value: item.hexadecimal_value,
        }
    }
}

impl Color {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
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
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(colors::dsl::colors
            .filter(colors::dsl::id.eq(id))
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
        use crate::schema::colors;
        colors::dsl::colors
            .filter(colors::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value FROM colors ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value FROM colors ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, hexadecimal_value FROM colors ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = countries)]
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
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
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(countries::dsl::countries
            .filter(countries::dsl::id.eq(id))
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
        use crate::schema::countries;
        countries::dsl::countries
            .filter(countries::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, iso, emoji, unicode, name FROM countries ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = derived_samples)]
pub struct DerivedSample {
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
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
        }
    }
}

impl DerivedSample {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::derived_samples;
       derived_samples::dsl::derived_samples
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::derived_samples;
        derived_samples::dsl::derived_samples
            .order_by(derived_samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(( self.parent_sample_id, self.child_sample_id ), connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( parent_sample_id, child_sample_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       ( parent_sample_id, child_sample_id ): ( Uuid, Uuid ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::parent_sample_id.eq(parent_sample_id))
            .filter(derived_samples::dsl::child_sample_id.eq(child_sample_id))
        ).execute(connection)
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
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = document_formats)]
pub struct DocumentFormat {
    pub id: i32,
    pub extension: String,
    pub mime_type: String,
}

impl From<DocumentFormat> for web_common::database::tables::DocumentFormat {
    fn from(item: DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
        }
    }
}

impl From<web_common::database::tables::DocumentFormat> for DocumentFormat {
    fn from(item: web_common::database::tables::DocumentFormat) -> Self {
        Self {
            id: item.id,
            extension: item.extension,
            mime_type: item.mime_type,
        }
    }
}

impl DocumentFormat {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::document_formats;
       document_formats::dsl::document_formats
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(document_formats::dsl::document_formats
            .filter(document_formats::dsl::id.eq(id))
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type FROM document_formats ",
            "WHERE $1 % f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type FROM document_formats ",
            "WHERE $1 <% f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY word_similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, extension, mime_type FROM document_formats ",
            "WHERE $1 <<% f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text) ",
            "ORDER BY strict_word_similarity($1, f_concat_document_formats_extension_mime_type((extension)::text, (mime_type)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = font_awesome_icons)]
pub struct FontAwesomeIcon {
    pub id: i32,
    pub name: String,
}

impl From<FontAwesomeIcon> for web_common::database::tables::FontAwesomeIcon {
    fn from(item: FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::FontAwesomeIcon> for FontAwesomeIcon {
    fn from(item: web_common::database::tables::FontAwesomeIcon) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

impl FontAwesomeIcon {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
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
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::id.eq(id))
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name FROM font_awesome_icons ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name FROM font_awesome_icons ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name FROM font_awesome_icons ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = login_providers)]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub font_awesome_icon_id: i32,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
            client_id_var_name: item.client_id_var_name,
            redirect_uri_var_name: item.redirect_uri_var_name,
            oauth_url: item.oauth_url,
            scope: item.scope,
        }
    }
}

impl LoginProvider {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::login_providers;
       login_providers::dsl::login_providers
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(login_providers::dsl::login_providers
            .filter(login_providers::dsl::id.eq(id))
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
        use crate::schema::login_providers;
        login_providers::dsl::login_providers
            .filter(login_providers::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = notifications)]
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::notifications;
       notifications::dsl::notifications
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(id))
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
        use crate::schema::notifications;
        notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = organizations)]
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::organizations;
       organizations::dsl::organizations
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(organizations::dsl::organizations
            .filter(organizations::dsl::id.eq(id))
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
        use crate::schema::organizations;
        organizations::dsl::organizations
            .filter(organizations::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 % name ",
            "ORDER BY similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 <% name ",
            "ORDER BY word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, url, country_id, state_province, domain FROM organizations ",
            "WHERE $1 <<% name ",
            "ORDER BY strict_word_similarity($1, name) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = primary_user_emails)]
pub struct PrimaryUserEmail {
    pub id: i32,
}

impl From<PrimaryUserEmail> for web_common::database::tables::PrimaryUserEmail {
    fn from(item: PrimaryUserEmail) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl From<web_common::database::tables::PrimaryUserEmail> for PrimaryUserEmail {
    fn from(item: web_common::database::tables::PrimaryUserEmail) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl PrimaryUserEmail {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::primary_user_emails;
       primary_user_emails::dsl::primary_user_emails
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(primary_user_emails::dsl::primary_user_emails
            .filter(primary_user_emails::dsl::id.eq(id))
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
        use crate::schema::primary_user_emails;
        primary_user_emails::dsl::primary_user_emails
            .filter(primary_user_emails::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = project_states)]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}

impl From<ProjectState> for web_common::database::tables::ProjectState {
    fn from(item: ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
        }
    }
}

impl ProjectState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::project_states;
       project_states::dsl::project_states
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(id))
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
        use crate::schema::project_states;
        project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM project_states ",
            "WHERE $1 % f_concat_project_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM project_states ",
            "WHERE $1 <% f_concat_project_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM project_states ",
            "WHERE $1 <<% f_concat_project_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
       projects::dsl::projects
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
        projects::dsl::projects
            .order_by(projects::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 % f_concat_projects_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <% f_concat_projects_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, updated_by, updated_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <<% f_concat_projects_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Identifiable, Eq, PartialEq, Clone, Serialize, Deserialize, Default, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects_users_roles)]
pub struct ProjectsUsersRole {
    pub table_id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_by: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_by: i32,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<ProjectsUsersRole> for web_common::database::tables::ProjectsUsersRole {
    fn from(item: ProjectsUsersRole) -> Self {
        Self {
            table_id: item.table_id,
            user_id: item.user_id,
            role_id: item.role_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
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
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl ProjectsUsersRole {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_roles;
       projects_users_roles::dsl::projects_users_roles
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        projects_users_roles::dsl::projects_users_roles
            .order_by(projects_users_roles::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(( self.table_id, self.user_id, self.role_id ), connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(projects_users_roles::dsl::projects_users_roles
            .filter(projects_users_roles::dsl::table_id.eq(table_id))
            .filter(projects_users_roles::dsl::user_id.eq(user_id))
            .filter(projects_users_roles::dsl::role_id.eq(role_id))
        ).execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `( table_id, user_id, role_id )` - The primary key(s) of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
       ( table_id, user_id, role_id ): ( i32, i32, i32 ),
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::projects_users_roles;
        projects_users_roles::dsl::projects_users_roles
            .filter(projects_users_roles::dsl::table_id.eq(table_id))
            .filter(projects_users_roles::dsl::user_id.eq(user_id))
            .filter(projects_users_roles::dsl::role_id.eq(role_id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}

impl From<Role> for web_common::database::tables::Role {
    fn from(item: Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
        }
    }
}

impl Role {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::roles;
       roles::dsl::roles
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
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
        use crate::schema::roles;
        roles::dsl::roles
            .filter(roles::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM roles ",
            "WHERE $1 % f_concat_roles_name(name, description) ",
            "ORDER BY similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM roles ",
            "WHERE $1 <% f_concat_roles_name(name, description) ",
            "ORDER BY word_similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM roles ",
            "WHERE $1 <<% f_concat_roles_name(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_roles_name(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
pub struct SampleBioOttTaxonItem {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampleBioOttTaxonItem> for web_common::database::tables::SampleBioOttTaxonItem {
    fn from(item: SampleBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampleBioOttTaxonItem> for SampleBioOttTaxonItem {
    fn from(item: web_common::database::tables::SampleBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampleBioOttTaxonItem {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
       sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .order_by(sample_bio_ott_taxon_items::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::id.eq(id))
        ).execute(connection)
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
        use crate::schema::sample_bio_ott_taxon_items;
        sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_states)]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}

impl From<SampleState> for web_common::database::tables::SampleState {
    fn from(item: SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
        }
    }
}

impl SampleState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_states;
       sample_states::dsl::sample_states
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(sample_states::dsl::sample_states
            .filter(sample_states::dsl::id.eq(id))
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
        use crate::schema::sample_states;
        sample_states::dsl::sample_states
            .filter(sample_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM sample_states ",
            "WHERE $1 % f_concat_sample_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM sample_states ",
            "WHERE $1 <% f_concat_sample_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM sample_states ",
            "WHERE $1 <<% f_concat_sample_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
pub struct SampledIndividualBioOttTaxonItem {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampledIndividualBioOttTaxonItem> for web_common::database::tables::SampledIndividualBioOttTaxonItem {
    fn from(item: SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl From<web_common::database::tables::SampledIndividualBioOttTaxonItem> for SampledIndividualBioOttTaxonItem {
    fn from(item: web_common::database::tables::SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampledIndividualBioOttTaxonItem {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
       sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .order_by(sampled_individual_bio_ott_taxon_items::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::id.eq(id))
        ).execute(connection)
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
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individuals)]
pub struct SampledIndividual {
    pub id: Uuid,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub name: Option<String>,
    pub tagged: bool,
}

impl From<SampledIndividual> for web_common::database::tables::SampledIndividual {
    fn from(item: SampledIndividual) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            name: item.name,
            tagged: item.tagged,
        }
    }
}

impl From<web_common::database::tables::SampledIndividual> for SampledIndividual {
    fn from(item: web_common::database::tables::SampledIndividual) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            name: item.name,
            tagged: item.tagged,
        }
    }
}

impl SampledIndividual {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individuals;
       sampled_individuals::dsl::sampled_individuals
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        sampled_individuals::dsl::sampled_individuals
            .order_by(sampled_individuals::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(id))
        ).execute(connection)
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
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = samples)]
pub struct Sample {
    pub id: Uuid,
    pub created_by: i32,
    pub sampled_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
    pub procedure_id: Uuid,
    pub state: i32,
}

impl From<Sample> for web_common::database::tables::Sample {
    fn from(item: Sample) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            procedure_id: item.procedure_id,
            state: item.state,
        }
    }
}

impl From<web_common::database::tables::Sample> for Sample {
    fn from(item: web_common::database::tables::Sample) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            sampled_by: item.sampled_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
            procedure_id: item.procedure_id,
            state: item.state,
        }
    }
}

impl Sample {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::samples;
       samples::dsl::samples
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::samples;
        samples::dsl::samples
            .order_by(samples::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(samples::dsl::samples
            .filter(samples::dsl::id.eq(id))
        ).execute(connection)
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampling_procedures)]
pub struct SamplingProcedure {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_by: i32,
    pub updated_at: NaiveDateTime,
}

impl From<SamplingProcedure> for web_common::database::tables::SamplingProcedure {
    fn from(item: SamplingProcedure) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl From<web_common::database::tables::SamplingProcedure> for SamplingProcedure {
    fn from(item: web_common::database::tables::SamplingProcedure) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl SamplingProcedure {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampling_procedures;
       sampling_procedures::dsl::sampling_procedures
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampling_procedures;
        sampling_procedures::dsl::sampling_procedures
            .order_by(sampling_procedures::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(sampling_procedures::dsl::sampling_procedures
            .filter(sampling_procedures::dsl::id.eq(id))
        ).execute(connection)
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
        use crate::schema::sampling_procedures;
        sampling_procedures::dsl::sampling_procedures
            .filter(sampling_procedures::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by, created_at, updated_by, updated_at FROM sampling_procedures ",
            "WHERE $1 % f_concat_sampling_procedures_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_sampling_procedures_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by, created_at, updated_by, updated_at FROM sampling_procedures ",
            "WHERE $1 <% f_concat_sampling_procedures_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_sampling_procedures_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by, created_at, updated_by, updated_at FROM sampling_procedures ",
            "WHERE $1 <<% f_concat_sampling_procedures_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sampling_procedures_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra)]
pub struct Spectra {
    pub id: i32,
    pub spectra_collection_id: i32,
}

impl From<Spectra> for web_common::database::tables::Spectra {
    fn from(item: Spectra) -> Self {
        Self {
            id: item.id,
            spectra_collection_id: item.spectra_collection_id,
        }
    }
}

impl From<web_common::database::tables::Spectra> for Spectra {
    fn from(item: web_common::database::tables::Spectra) -> Self {
        Self {
            id: item.id,
            spectra_collection_id: item.spectra_collection_id,
        }
    }
}

impl Spectra {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra;
       spectra::dsl::spectra
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(id))
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
        use crate::schema::spectra;
        spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra_collections)]
pub struct SpectraCollection {
    pub id: i32,
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
            sample_id: item.sample_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl SpectraCollection {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra_collections;
       spectra_collections::dsl::spectra_collections
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra_collections;
        spectra_collections::dsl::spectra_collections
            .order_by(spectra_collections::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
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
        use crate::schema::spectra_collections;
        spectra_collections::dsl::spectra_collections
            .filter(spectra_collections::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = team_states)]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon_id: i32,
    pub color_id: i32,
}

impl From<TeamState> for web_common::database::tables::TeamState {
    fn from(item: TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon_id: item.font_awesome_icon_id,
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
            font_awesome_icon_id: item.font_awesome_icon_id,
            color_id: item.color_id,
        }
    }
}

impl TeamState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::team_states;
       team_states::dsl::team_states
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(team_states::dsl::team_states
            .filter(team_states::dsl::id.eq(id))
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
        use crate::schema::team_states;
        team_states::dsl::team_states
            .filter(team_states::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM team_states ",
            "WHERE $1 % f_concat_team_states_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM team_states ",
            "WHERE $1 <% f_concat_team_states_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon_id, color_id FROM team_states ",
            "WHERE $1 <<% f_concat_team_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
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
            parent_team_id: item.parent_team_id,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_by: item.updated_by,
            updated_at: item.updated_at,
        }
    }
}

impl Team {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
       teams::dsl::teams
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
        teams::dsl::teams
            .order_by(teams::dsl::updated_at.desc())
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 % f_concat_teams_name_description(name, description) ",
            "ORDER BY similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <% f_concat_teams_name_description(name, description) ",
            "ORDER BY word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id, created_by, created_at, updated_by, updated_at FROM teams ",
            "WHERE $1 <<% f_concat_teams_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = units)]
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
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
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
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(units::dsl::units
            .filter(units::dsl::id.eq(id))
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
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 % f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 <% f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY word_similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, symbol FROM units ",
            "WHERE $1 <<% f_concat_units_name_description_symbol(name, description, symbol) ",
            "ORDER BY strict_word_similarity($1, f_concat_units_name_description_symbol(name, description, symbol)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = user_emails)]
pub struct UserEmail {
    pub id: i32,
    pub email: String,
    pub user_id: i32,
    pub login_provider_id: i32,
}

impl From<UserEmail> for web_common::database::tables::UserEmail {
    fn from(item: UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            user_id: item.user_id,
            login_provider_id: item.login_provider_id,
        }
    }
}

impl From<web_common::database::tables::UserEmail> for UserEmail {
    fn from(item: web_common::database::tables::UserEmail) -> Self {
        Self {
            id: item.id,
            email: item.email,
            user_id: item.user_id,
            login_provider_id: item.login_provider_id,
        }
    }
}

impl UserEmail {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::user_emails;
       user_emails::dsl::user_emails
            .offset(offset.unwrap_or(0))
            .limit(limit.unwrap_or(10))
            .load::<Self>(connection)
    }
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
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
        use crate::schema::user_emails;
        user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Default, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub profile_picture: Option<Vec<u8>>,
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
            profile_picture: item.profile_picture,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl User {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
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
    /// Get all of the structs from the database ordered by the updated_at column.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve. By default, this is 10.
    /// * `offset` - The number of structs to skip. By default, this is 0.
    /// * `connection` - The connection to the database.
    ///
    pub fn all_by_updated_at(
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
    /// Delete the struct from the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn delete(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        Self::delete_by_id(self.id, connection)
    }
    /// Delete the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The primary key(s) of the struct to delete.
    /// * `connection` - The connection to the database.
    ///
    pub fn delete_by_id(
       id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users::dsl::users
            .filter(users::dsl::id.eq(id))
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
        use crate::schema::users;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Search for the struct by a given string by Postgres's `similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 % f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text) ",
            "ORDER BY similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 <% f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text) ",
            "ORDER BY word_similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
    /// Search for the struct by a given string by Postgres's `strict_word_similarity`.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The connection to the database.
    ///
    pub fn strict_word_similarity_search(
        query: &str,
        limit: Option<i32>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let limit = limit.unwrap_or(10);
        // If the query string is empty, we run an all query with the
        // limit parameter provided instead of a more complex similarity
        // search.
        if query.is_empty() {
            return Self::all(Some(limit as i64), None, connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, profile_picture, created_at, updated_at FROM users ",
            "WHERE $1 <<% f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text) ",
            "ORDER BY strict_word_similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
