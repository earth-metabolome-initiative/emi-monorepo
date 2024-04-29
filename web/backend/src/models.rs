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

#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_ranks;
        let query = bio_ott_ranks::dsl::bio_ott_ranks;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(bio_ott_ranks::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(bio_ott_ranks::table)
            .values(self)
            .on_conflict(bio_ott_ranks::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(bio_ott_ranks::dsl::bio_ott_ranks
            .filter(bio_ott_ranks::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::bio_ott_taxon_items;
        let query = bio_ott_taxon_items::dsl::bio_ott_taxon_items;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(bio_ott_taxon_items::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(bio_ott_taxon_items::table)
            .values(self)
            .on_conflict(bio_ott_taxon_items::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(bio_ott_taxon_items::dsl::bio_ott_taxon_items
            .filter(bio_ott_taxon_items::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::colors;
        let query = colors::dsl::colors;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(colors::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(colors::table)
            .values(self)
            .on_conflict(colors::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(colors::dsl::colors
            .filter(colors::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = container_horizontal_rules)]
pub struct ContainerHorizontalRule {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub item_type_id: i32,
    pub other_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}

impl From<ContainerHorizontalRule> for web_common::database::tables::ContainerHorizontalRule {
    fn from(item: ContainerHorizontalRule) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            name: item.name,
            item_type_id: item.item_type_id,
            other_item_type_id: item.other_item_type_id,
            minimum_temperature: item.minimum_temperature,
            maximum_temperature: item.maximum_temperature,
            minimum_humidity: item.minimum_humidity,
            maximum_humidity: item.maximum_humidity,
            minimum_pressure: item.minimum_pressure,
            maximum_pressure: item.maximum_pressure,
        }
    }
}

impl From<web_common::database::tables::ContainerHorizontalRule> for ContainerHorizontalRule {
    fn from(item: web_common::database::tables::ContainerHorizontalRule) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            name: item.name,
            item_type_id: item.item_type_id,
            other_item_type_id: item.other_item_type_id,
            minimum_temperature: item.minimum_temperature,
            maximum_temperature: item.maximum_temperature,
            minimum_humidity: item.minimum_humidity,
            maximum_humidity: item.maximum_humidity,
            minimum_pressure: item.minimum_pressure,
            maximum_pressure: item.maximum_pressure,
        }
    }
}

impl ContainerHorizontalRule {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::container_horizontal_rules;
        let query = container_horizontal_rules::dsl::container_horizontal_rules;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(container_horizontal_rules::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(container_horizontal_rules::table)
            .values(self)
            .on_conflict(container_horizontal_rules::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(container_horizontal_rules::dsl::container_horizontal_rules
            .filter(container_horizontal_rules::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::container_horizontal_rules;
        container_horizontal_rules::dsl::container_horizontal_rules
            .filter(container_horizontal_rules::dsl::id.eq(id))
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
        use crate::schema::container_horizontal_rules;
        container_horizontal_rules::dsl::container_horizontal_rules
            .filter(container_horizontal_rules::dsl::name.eq(name))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = container_vertical_rules)]
pub struct ContainerVerticalRule {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub container_item_type_id: i32,
    pub contained_item_type_id: i32,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
}

impl From<ContainerVerticalRule> for web_common::database::tables::ContainerVerticalRule {
    fn from(item: ContainerVerticalRule) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            name: item.name,
            container_item_type_id: item.container_item_type_id,
            contained_item_type_id: item.contained_item_type_id,
            minimum_temperature: item.minimum_temperature,
            maximum_temperature: item.maximum_temperature,
            minimum_humidity: item.minimum_humidity,
            maximum_humidity: item.maximum_humidity,
            minimum_pressure: item.minimum_pressure,
            maximum_pressure: item.maximum_pressure,
        }
    }
}

impl From<web_common::database::tables::ContainerVerticalRule> for ContainerVerticalRule {
    fn from(item: web_common::database::tables::ContainerVerticalRule) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            name: item.name,
            container_item_type_id: item.container_item_type_id,
            contained_item_type_id: item.contained_item_type_id,
            minimum_temperature: item.minimum_temperature,
            maximum_temperature: item.maximum_temperature,
            minimum_humidity: item.minimum_humidity,
            maximum_humidity: item.maximum_humidity,
            minimum_pressure: item.minimum_pressure,
            maximum_pressure: item.maximum_pressure,
        }
    }
}

impl ContainerVerticalRule {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::container_vertical_rules;
        let query = container_vertical_rules::dsl::container_vertical_rules;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(container_vertical_rules::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(container_vertical_rules::table)
            .values(self)
            .on_conflict(container_vertical_rules::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(container_vertical_rules::dsl::container_vertical_rules
            .filter(container_vertical_rules::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::container_vertical_rules;
        container_vertical_rules::dsl::container_vertical_rules
            .filter(container_vertical_rules::dsl::id.eq(id))
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
        use crate::schema::container_vertical_rules;
        container_vertical_rules::dsl::container_vertical_rules
            .filter(container_vertical_rules::dsl::name.eq(name))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = continuous_units)]
pub struct ContinuousUnit {
    pub id: i32,
}

impl From<ContinuousUnit> for web_common::database::tables::ContinuousUnit {
    fn from(item: ContinuousUnit) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl From<web_common::database::tables::ContinuousUnit> for ContinuousUnit {
    fn from(item: web_common::database::tables::ContinuousUnit) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl ContinuousUnit {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::continuous_units;
        let query = continuous_units::dsl::continuous_units;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(continuous_units::table)
            .values(self)
            .get_result(connection)
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
        diesel::delete(continuous_units::dsl::continuous_units
            .filter(continuous_units::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::continuous_units;
        continuous_units::dsl::continuous_units
            .filter(continuous_units::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = derived_samples)]
pub struct DerivedSample {
    pub id: i32,
    pub created_by: i32,
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

impl From<DerivedSample> for web_common::database::tables::DerivedSample {
    fn from(item: DerivedSample) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl From<web_common::database::tables::DerivedSample> for DerivedSample {
    fn from(item: web_common::database::tables::DerivedSample) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            parent_sample_id: item.parent_sample_id,
            child_sample_id: item.child_sample_id,
        }
    }
}

impl DerivedSample {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::derived_samples;
        let query = derived_samples::dsl::derived_samples;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(derived_samples::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(derived_samples::table)
            .values(self)
            .on_conflict(derived_samples::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::derived_samples;
        derived_samples::dsl::derived_samples
            .filter(derived_samples::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = discrete_units)]
pub struct DiscreteUnit {
    pub id: i32,
}

impl From<DiscreteUnit> for web_common::database::tables::DiscreteUnit {
    fn from(item: DiscreteUnit) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl From<web_common::database::tables::DiscreteUnit> for DiscreteUnit {
    fn from(item: web_common::database::tables::DiscreteUnit) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl DiscreteUnit {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::discrete_units;
        let query = discrete_units::dsl::discrete_units;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(discrete_units::table)
            .values(self)
            .get_result(connection)
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
        diesel::delete(discrete_units::dsl::discrete_units
            .filter(discrete_units::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::discrete_units;
        discrete_units::dsl::discrete_units
            .filter(discrete_units::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::document_formats;
        let query = document_formats::dsl::document_formats;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(document_formats::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(document_formats::table)
            .values(self)
            .on_conflict(document_formats::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(document_formats::dsl::document_formats
            .filter(document_formats::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = documents)]
pub struct Document {
    pub id: Uuid,
    pub author_id: i32,
    pub path: String,
    pub format_id: i32,
    pub bytes: i32,
}

impl From<Document> for web_common::database::tables::Document {
    fn from(item: Document) -> Self {
        Self {
            id: item.id,
            author_id: item.author_id,
            path: item.path,
            format_id: item.format_id,
            bytes: item.bytes,
        }
    }
}

impl From<web_common::database::tables::Document> for Document {
    fn from(item: web_common::database::tables::Document) -> Self {
        Self {
            id: item.id,
            author_id: item.author_id,
            path: item.path,
            format_id: item.format_id,
            bytes: item.bytes,
        }
    }
}

impl Document {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::documents;
        let query = documents::dsl::documents;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(documents::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(documents::table)
            .values(self)
            .on_conflict(documents::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(documents::dsl::documents
            .filter(documents::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::documents;
        documents::dsl::documents
            .filter(documents::dsl::id.eq(id))
            .first::<Self>(connection)
    }
    /// Get the struct from the database by its path.
    ///
    /// # Arguments
    /// * `path` - The path of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn from_path(
        path: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::documents;
        documents::dsl::documents
            .filter(documents::dsl::path.eq(path))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::font_awesome_icons;
        let query = font_awesome_icons::dsl::font_awesome_icons;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(font_awesome_icons::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(font_awesome_icons::table)
            .values(self)
            .on_conflict(font_awesome_icons::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(font_awesome_icons::dsl::font_awesome_icons
            .filter(font_awesome_icons::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
            return Self::all(Some(limit as i64), connection);
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_categories)]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_by: i32,
}

impl From<ItemCategory> for web_common::database::tables::ItemCategory {
    fn from(item: ItemCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
        }
    }
}

impl From<web_common::database::tables::ItemCategory> for ItemCategory {
    fn from(item: web_common::database::tables::ItemCategory) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
        }
    }
}

impl ItemCategory {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_categories;
        let query = item_categories::dsl::item_categories;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_categories::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_categories::table)
            .values(self)
            .on_conflict(item_categories::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_categories::dsl::item_categories
            .filter(item_categories::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_categories;
        item_categories::dsl::item_categories
            .filter(item_categories::dsl::id.eq(id))
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
        use crate::schema::item_categories;
        item_categories::dsl::item_categories
            .filter(item_categories::dsl::name.eq(name))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_category_relationships)]
pub struct ItemCategoryRelationship {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
    pub added_by: i32,
}

impl From<ItemCategoryRelationship> for web_common::database::tables::ItemCategoryRelationship {
    fn from(item: ItemCategoryRelationship) -> Self {
        Self {
            id: item.id,
            parent_id: item.parent_id,
            child_id: item.child_id,
            added_by: item.added_by,
        }
    }
}

impl From<web_common::database::tables::ItemCategoryRelationship> for ItemCategoryRelationship {
    fn from(item: web_common::database::tables::ItemCategoryRelationship) -> Self {
        Self {
            id: item.id,
            parent_id: item.parent_id,
            child_id: item.child_id,
            added_by: item.added_by,
        }
    }
}

impl ItemCategoryRelationship {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_category_relationships;
        let query = item_category_relationships::dsl::item_category_relationships;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_category_relationships::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_category_relationships::table)
            .values(self)
            .on_conflict(item_category_relationships::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_category_relationships::dsl::item_category_relationships
            .filter(item_category_relationships::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_category_relationships;
        item_category_relationships::dsl::item_category_relationships
            .filter(item_category_relationships::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_category_units)]
pub struct ItemCategoryUnit {
    pub id: i32,
    pub item_category_id: i32,
    pub unit_id: i32,
}

impl From<ItemCategoryUnit> for web_common::database::tables::ItemCategoryUnit {
    fn from(item: ItemCategoryUnit) -> Self {
        Self {
            id: item.id,
            item_category_id: item.item_category_id,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::ItemCategoryUnit> for ItemCategoryUnit {
    fn from(item: web_common::database::tables::ItemCategoryUnit) -> Self {
        Self {
            id: item.id,
            item_category_id: item.item_category_id,
            unit_id: item.unit_id,
        }
    }
}

impl ItemCategoryUnit {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_category_units;
        let query = item_category_units::dsl::item_category_units;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_category_units::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_category_units::table)
            .values(self)
            .on_conflict(item_category_units::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_category_units::dsl::item_category_units
            .filter(item_category_units::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_category_units;
        item_category_units::dsl::item_category_units
            .filter(item_category_units::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_continuous_quantities)]
pub struct ItemContinuousQuantity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub amount: i32,
    pub unit_id: i32,
    pub sensor_id: Option<Uuid>,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<i32>,
}

impl From<ItemContinuousQuantity> for web_common::database::tables::ItemContinuousQuantity {
    fn from(item: ItemContinuousQuantity) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            amount: item.amount,
            unit_id: item.unit_id,
            sensor_id: item.sensor_id,
            measured_at: item.measured_at,
            measured_by: item.measured_by,
        }
    }
}

impl From<web_common::database::tables::ItemContinuousQuantity> for ItemContinuousQuantity {
    fn from(item: web_common::database::tables::ItemContinuousQuantity) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            amount: item.amount,
            unit_id: item.unit_id,
            sensor_id: item.sensor_id,
            measured_at: item.measured_at,
            measured_by: item.measured_by,
        }
    }
}

impl ItemContinuousQuantity {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_continuous_quantities;
        let query = item_continuous_quantities::dsl::item_continuous_quantities;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_continuous_quantities::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_continuous_quantities::table)
            .values(self)
            .on_conflict(item_continuous_quantities::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_continuous_quantities::dsl::item_continuous_quantities
            .filter(item_continuous_quantities::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_continuous_quantities;
        item_continuous_quantities::dsl::item_continuous_quantities
            .filter(item_continuous_quantities::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_discrete_quantities)]
pub struct ItemDiscreteQuantity {
    pub id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub unit_id: i32,
    pub measured_at: NaiveDateTime,
    pub measured_by: Option<i32>,
}

impl From<ItemDiscreteQuantity> for web_common::database::tables::ItemDiscreteQuantity {
    fn from(item: ItemDiscreteQuantity) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
            measured_at: item.measured_at,
            measured_by: item.measured_by,
        }
    }
}

impl From<web_common::database::tables::ItemDiscreteQuantity> for ItemDiscreteQuantity {
    fn from(item: web_common::database::tables::ItemDiscreteQuantity) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
            measured_at: item.measured_at,
            measured_by: item.measured_by,
        }
    }
}

impl ItemDiscreteQuantity {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_discrete_quantities;
        let query = item_discrete_quantities::dsl::item_discrete_quantities;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_discrete_quantities::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_discrete_quantities::table)
            .values(self)
            .on_conflict(item_discrete_quantities::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_discrete_quantities::dsl::item_discrete_quantities
            .filter(item_discrete_quantities::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_discrete_quantities;
        item_discrete_quantities::dsl::item_discrete_quantities
            .filter(item_discrete_quantities::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_locations)]
pub struct ItemLocation {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub located_by: Option<i32>,
    pub located_at: NaiveDateTime,
    pub location_id: Option<Uuid>,
}

impl From<ItemLocation> for web_common::database::tables::ItemLocation {
    fn from(item: ItemLocation) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            located_by: item.located_by,
            located_at: item.located_at,
            location_id: item.location_id,
        }
    }
}

impl From<web_common::database::tables::ItemLocation> for ItemLocation {
    fn from(item: web_common::database::tables::ItemLocation) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            located_by: item.located_by,
            located_at: item.located_at,
            location_id: item.location_id,
        }
    }
}

impl ItemLocation {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_locations;
        let query = item_locations::dsl::item_locations;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_locations::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_locations::table)
            .values(self)
            .on_conflict(item_locations::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_locations::dsl::item_locations
            .filter(item_locations::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_locations;
        item_locations::dsl::item_locations
            .filter(item_locations::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = item_units)]
pub struct ItemUnit {
    pub id: Uuid,
    pub item_id: Uuid,
    pub unit_id: i32,
}

impl From<ItemUnit> for web_common::database::tables::ItemUnit {
    fn from(item: ItemUnit) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::ItemUnit> for ItemUnit {
    fn from(item: web_common::database::tables::ItemUnit) -> Self {
        Self {
            id: item.id,
            item_id: item.item_id,
            unit_id: item.unit_id,
        }
    }
}

impl ItemUnit {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::item_units;
        let query = item_units::dsl::item_units;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_units::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(item_units::table)
            .values(self)
            .on_conflict(item_units::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(item_units::dsl::item_units
            .filter(item_units::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::item_units;
        item_units::dsl::item_units
            .filter(item_units::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
}

impl From<Item> for web_common::database::tables::Item {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            parent_id: item.parent_id,
        }
    }
}

impl From<web_common::database::tables::Item> for Item {
    fn from(item: web_common::database::tables::Item) -> Self {
        Self {
            id: item.id,
            parent_id: item.parent_id,
        }
    }
}

impl Item {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::items;
        let query = items::dsl::items;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(items::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(items::table)
            .values(self)
            .on_conflict(items::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(items::dsl::items
            .filter(items::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::items;
        items::dsl::items
            .filter(items::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = locations)]
pub struct Location {
    pub id: Uuid,
    pub latitude_degrees: Option<i32>,
    pub latitude_minutes: Option<i32>,
    pub latitude_seconds: Option<i32>,
    pub longitude_degrees: Option<i32>,
    pub longitude_minutes: Option<i32>,
    pub longitude_seconds: Option<i32>,
    pub altitude: Option<i32>,
    pub address: Option<String>,
    pub geolocalization_device_id: Option<Uuid>,
    pub altitude_device_id: Option<Uuid>,
    pub parent_location_id: Option<Uuid>,
}

impl From<Location> for web_common::database::tables::Location {
    fn from(item: Location) -> Self {
        Self {
            id: item.id,
            latitude_degrees: item.latitude_degrees,
            latitude_minutes: item.latitude_minutes,
            latitude_seconds: item.latitude_seconds,
            longitude_degrees: item.longitude_degrees,
            longitude_minutes: item.longitude_minutes,
            longitude_seconds: item.longitude_seconds,
            altitude: item.altitude,
            address: item.address,
            geolocalization_device_id: item.geolocalization_device_id,
            altitude_device_id: item.altitude_device_id,
            parent_location_id: item.parent_location_id,
        }
    }
}

impl From<web_common::database::tables::Location> for Location {
    fn from(item: web_common::database::tables::Location) -> Self {
        Self {
            id: item.id,
            latitude_degrees: item.latitude_degrees,
            latitude_minutes: item.latitude_minutes,
            latitude_seconds: item.latitude_seconds,
            longitude_degrees: item.longitude_degrees,
            longitude_minutes: item.longitude_minutes,
            longitude_seconds: item.longitude_seconds,
            altitude: item.altitude,
            address: item.address,
            geolocalization_device_id: item.geolocalization_device_id,
            altitude_device_id: item.altitude_device_id,
            parent_location_id: item.parent_location_id,
        }
    }
}

impl Location {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::locations;
        let query = locations::dsl::locations;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(locations::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(locations::table)
            .values(self)
            .on_conflict(locations::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(locations::dsl::locations
            .filter(locations::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: Uuid,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::locations;
        locations::dsl::locations
            .filter(locations::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = login_providers)]
pub struct LoginProvider {
    pub id: i32,
    pub name: String,
    pub font_awesome_icon: String,
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
            font_awesome_icon: item.font_awesome_icon,
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
            font_awesome_icon: item.font_awesome_icon,
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::login_providers;
        let query = login_providers::dsl::login_providers;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(login_providers::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(login_providers::table)
            .values(self)
            .on_conflict(login_providers::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(login_providers::dsl::login_providers
            .filter(login_providers::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = manufactured_item_categories)]
pub struct ManufacturedItemCategory {
    pub id: i32,
    pub cost: i32,
    pub cost_per_day: i32,
    pub currency: String,
    pub manifacturer_id: i32,
}

impl From<ManufacturedItemCategory> for web_common::database::tables::ManufacturedItemCategory {
    fn from(item: ManufacturedItemCategory) -> Self {
        Self {
            id: item.id,
            cost: item.cost,
            cost_per_day: item.cost_per_day,
            currency: item.currency,
            manifacturer_id: item.manifacturer_id,
        }
    }
}

impl From<web_common::database::tables::ManufacturedItemCategory> for ManufacturedItemCategory {
    fn from(item: web_common::database::tables::ManufacturedItemCategory) -> Self {
        Self {
            id: item.id,
            cost: item.cost,
            cost_per_day: item.cost_per_day,
            currency: item.currency,
            manifacturer_id: item.manifacturer_id,
        }
    }
}

impl ManufacturedItemCategory {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::manufactured_item_categories;
        let query = manufactured_item_categories::dsl::manufactured_item_categories;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(manufactured_item_categories::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(manufactured_item_categories::table)
            .values(self)
            .on_conflict(manufactured_item_categories::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(manufactured_item_categories::dsl::manufactured_item_categories
            .filter(manufactured_item_categories::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::manufactured_item_categories;
        manufactured_item_categories::dsl::manufactured_item_categories
            .filter(manufactured_item_categories::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = notifications)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub operation: String,
    pub table_name: String,
    pub read: bool,
}

impl From<Notification> for web_common::database::tables::Notification {
    fn from(item: Notification) -> Self {
        Self {
            id: item.id,
            user_id: item.user_id,
            operation: item.operation,
            table_name: item.table_name,
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
            read: item.read,
        }
    }
}

impl Notification {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::notifications;
        let query = notifications::dsl::notifications;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(notifications::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(notifications::table)
            .values(self)
            .on_conflict(notifications::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(notifications::dsl::notifications
            .filter(notifications::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = organizations)]
pub struct Organization {
    pub id: i32,
    pub parent_organization_id: Option<i32>,
    pub name: String,
}

impl From<Organization> for web_common::database::tables::Organization {
    fn from(item: Organization) -> Self {
        Self {
            id: item.id,
            parent_organization_id: item.parent_organization_id,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::Organization> for Organization {
    fn from(item: web_common::database::tables::Organization) -> Self {
        Self {
            id: item.id,
            parent_organization_id: item.parent_organization_id,
            name: item.name,
        }
    }
}

impl Organization {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::organizations;
        let query = organizations::dsl::organizations;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(organizations::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(organizations::table)
            .values(self)
            .on_conflict(organizations::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(organizations::dsl::organizations
            .filter(organizations::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::primary_user_emails;
        let query = primary_user_emails::dsl::primary_user_emails;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(primary_user_emails::table)
            .values(self)
            .get_result(connection)
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
        diesel::delete(primary_user_emails::dsl::primary_user_emails
            .filter(primary_user_emails::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = procedure_continuous_requirements)]
pub struct ProcedureContinuousRequirement {
    pub id: i32,
    pub created_by: i32,
    pub procedure_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}

impl From<ProcedureContinuousRequirement> for web_common::database::tables::ProcedureContinuousRequirement {
    fn from(item: ProcedureContinuousRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            procedure_id: item.procedure_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::ProcedureContinuousRequirement> for ProcedureContinuousRequirement {
    fn from(item: web_common::database::tables::ProcedureContinuousRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            procedure_id: item.procedure_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl ProcedureContinuousRequirement {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::procedure_continuous_requirements;
        let query = procedure_continuous_requirements::dsl::procedure_continuous_requirements;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedure_continuous_requirements::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedure_continuous_requirements::table)
            .values(self)
            .on_conflict(procedure_continuous_requirements::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(procedure_continuous_requirements::dsl::procedure_continuous_requirements
            .filter(procedure_continuous_requirements::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::procedure_continuous_requirements;
        procedure_continuous_requirements::dsl::procedure_continuous_requirements
            .filter(procedure_continuous_requirements::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = procedure_discrete_requirements)]
pub struct ProcedureDiscreteRequirement {
    pub id: i32,
    pub created_by: i32,
    pub procedure_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}

impl From<ProcedureDiscreteRequirement> for web_common::database::tables::ProcedureDiscreteRequirement {
    fn from(item: ProcedureDiscreteRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            procedure_id: item.procedure_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::ProcedureDiscreteRequirement> for ProcedureDiscreteRequirement {
    fn from(item: web_common::database::tables::ProcedureDiscreteRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            procedure_id: item.procedure_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl ProcedureDiscreteRequirement {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::procedure_discrete_requirements;
        let query = procedure_discrete_requirements::dsl::procedure_discrete_requirements;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedure_discrete_requirements::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedure_discrete_requirements::table)
            .values(self)
            .on_conflict(procedure_discrete_requirements::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(procedure_discrete_requirements::dsl::procedure_discrete_requirements
            .filter(procedure_discrete_requirements::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::procedure_discrete_requirements;
        procedure_discrete_requirements::dsl::procedure_discrete_requirements
            .filter(procedure_discrete_requirements::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = procedures)]
pub struct Procedure {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<i32>,
}

impl From<Procedure> for web_common::database::tables::Procedure {
    fn from(item: Procedure) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
        }
    }
}

impl From<web_common::database::tables::Procedure> for Procedure {
    fn from(item: web_common::database::tables::Procedure) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
        }
    }
}

impl Procedure {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::procedures;
        let query = procedures::dsl::procedures;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedures::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(procedures::table)
            .values(self)
            .on_conflict(procedures::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(procedures::dsl::procedures
            .filter(procedures::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::procedures;
        procedures::dsl::procedures
            .filter(procedures::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = project_requirements)]
pub struct ProjectRequirement {
    pub id: i32,
    pub created_by: i32,
    pub project_id: i32,
    pub item_category_id: i32,
    pub quantity: i32,
    pub unit_id: Option<i32>,
}

impl From<ProjectRequirement> for web_common::database::tables::ProjectRequirement {
    fn from(item: ProjectRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            project_id: item.project_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl From<web_common::database::tables::ProjectRequirement> for ProjectRequirement {
    fn from(item: web_common::database::tables::ProjectRequirement) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
            project_id: item.project_id,
            item_category_id: item.item_category_id,
            quantity: item.quantity,
            unit_id: item.unit_id,
        }
    }
}

impl ProjectRequirement {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::project_requirements;
        let query = project_requirements::dsl::project_requirements;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(project_requirements::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(project_requirements::table)
            .values(self)
            .on_conflict(project_requirements::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(project_requirements::dsl::project_requirements
            .filter(project_requirements::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::project_requirements;
        project_requirements::dsl::project_requirements
            .filter(project_requirements::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = project_states)]
pub struct ProjectState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}

impl From<ProjectState> for web_common::database::tables::ProjectState {
    fn from(item: ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl From<web_common::database::tables::ProjectState> for ProjectState {
    fn from(item: web_common::database::tables::ProjectState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl ProjectState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::project_states;
        let query = project_states::dsl::project_states;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(project_states::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(project_states::table)
            .values(self)
            .on_conflict(project_states::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(project_states::dsl::project_states
            .filter(project_states::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM project_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM project_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM project_states ",
            "WHERE $1 <<% f_concat_project_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_project_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub public: bool,
    pub state_id: i32,
    pub parent_project_id: Option<i32>,
    pub budget: Option<i64>,
    pub expenses: Option<i64>,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
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
            expected_end_date: item.expected_end_date,
            end_date: item.end_date,
        }
    }
}

impl Project {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::projects;
        let query = projects::dsl::projects;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(projects::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(projects::table)
            .values(self)
            .on_conflict(projects::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(projects::dsl::projects
            .filter(projects::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date FROM projects ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date FROM projects ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, public, state_id, parent_project_id, budget, expenses, created_by, created_at, expected_end_date, end_date FROM projects ",
            "WHERE $1 <<% f_concat_projects_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_projects_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl From<Role> for web_common::database::tables::Role {
    fn from(item: Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

impl From<web_common::database::tables::Role> for Role {
    fn from(item: web_common::database::tables::Role) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}

impl Role {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::roles;
        let query = roles::dsl::roles;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(roles::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(roles::table)
            .values(self)
            .on_conflict(roles::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(roles::dsl::roles
            .filter(roles::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_bio_ott_taxon_items)]
pub struct SampleBioOttTaxonItem {
    pub id: Uuid,
    pub created_by: i32,
    pub sample_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampleBioOttTaxonItem> for web_common::database::tables::SampleBioOttTaxonItem {
    fn from(item: SampleBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
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
            sample_id: item.sample_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampleBioOttTaxonItem {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_bio_ott_taxon_items;
        let query = sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sample_bio_ott_taxon_items::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sample_bio_ott_taxon_items::table)
            .values(self)
            .on_conflict(sample_bio_ott_taxon_items::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(sample_bio_ott_taxon_items::dsl::sample_bio_ott_taxon_items
            .filter(sample_bio_ott_taxon_items::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sample_states)]
pub struct SampleState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}

impl From<SampleState> for web_common::database::tables::SampleState {
    fn from(item: SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl From<web_common::database::tables::SampleState> for SampleState {
    fn from(item: web_common::database::tables::SampleState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl SampleState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sample_states;
        let query = sample_states::dsl::sample_states;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sample_states::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sample_states::table)
            .values(self)
            .on_conflict(sample_states::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(sample_states::dsl::sample_states
            .filter(sample_states::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM sample_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM sample_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM sample_states ",
            "WHERE $1 <<% f_concat_sample_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sample_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampled_individual_bio_ott_taxon_items)]
pub struct SampledIndividualBioOttTaxonItem {
    pub id: Uuid,
    pub created_by: i32,
    pub sampled_individual_id: Uuid,
    pub taxon_id: i32,
}

impl From<SampledIndividualBioOttTaxonItem> for web_common::database::tables::SampledIndividualBioOttTaxonItem {
    fn from(item: SampledIndividualBioOttTaxonItem) -> Self {
        Self {
            id: item.id,
            created_by: item.created_by,
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
            sampled_individual_id: item.sampled_individual_id,
            taxon_id: item.taxon_id,
        }
    }
}

impl SampledIndividualBioOttTaxonItem {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individual_bio_ott_taxon_items;
        let query = sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sampled_individual_bio_ott_taxon_items::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sampled_individual_bio_ott_taxon_items::table)
            .values(self)
            .on_conflict(sampled_individual_bio_ott_taxon_items::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(sampled_individual_bio_ott_taxon_items::dsl::sampled_individual_bio_ott_taxon_items
            .filter(sampled_individual_bio_ott_taxon_items::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable)]
#[diesel(table_name = sampled_individuals)]
pub struct SampledIndividual {
    pub id: Uuid,
}

impl From<SampledIndividual> for web_common::database::tables::SampledIndividual {
    fn from(item: SampledIndividual) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl From<web_common::database::tables::SampledIndividual> for SampledIndividual {
    fn from(item: web_common::database::tables::SampledIndividual) -> Self {
        Self {
            id: item.id,
        }
    }
}

impl SampledIndividual {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampled_individuals;
        let query = sampled_individuals::dsl::sampled_individuals;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sampled_individuals::table)
            .values(self)
            .get_result(connection)
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
        diesel::delete(sampled_individuals::dsl::sampled_individuals
            .filter(sampled_individuals::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = samples)]
pub struct Sample {
    pub id: Uuid,
    pub inserted_by: i32,
    pub sampled_by: i32,
    pub procedure_id: Uuid,
    pub state: i32,
}

impl From<Sample> for web_common::database::tables::Sample {
    fn from(item: Sample) -> Self {
        Self {
            id: item.id,
            inserted_by: item.inserted_by,
            sampled_by: item.sampled_by,
            procedure_id: item.procedure_id,
            state: item.state,
        }
    }
}

impl From<web_common::database::tables::Sample> for Sample {
    fn from(item: web_common::database::tables::Sample) -> Self {
        Self {
            id: item.id,
            inserted_by: item.inserted_by,
            sampled_by: item.sampled_by,
            procedure_id: item.procedure_id,
            state: item.state,
        }
    }
}

impl Sample {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::samples;
        let query = samples::dsl::samples;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(samples::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(samples::table)
            .values(self)
            .on_conflict(samples::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(samples::dsl::samples
            .filter(samples::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = sampling_procedures)]
pub struct SamplingProcedure {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<i32>,
}

impl From<SamplingProcedure> for web_common::database::tables::SamplingProcedure {
    fn from(item: SamplingProcedure) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            created_by: item.created_by,
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
        }
    }
}

impl SamplingProcedure {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::sampling_procedures;
        let query = sampling_procedures::dsl::sampling_procedures;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sampling_procedures::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sampling_procedures::table)
            .values(self)
            .on_conflict(sampling_procedures::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(sampling_procedures::dsl::sampling_procedures
            .filter(sampling_procedures::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by FROM sampling_procedures ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by FROM sampling_procedures ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, created_by FROM sampling_procedures ",
            "WHERE $1 <<% f_concat_sampling_procedures_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_sampling_procedures_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra;
        let query = spectra::dsl::spectra;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(spectra::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(spectra::table)
            .values(self)
            .on_conflict(spectra::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(spectra::dsl::spectra
            .filter(spectra::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = spectra_collection)]
pub struct SpectraCollection {
    pub id: i32,
    pub sample_id: Uuid,
    pub created_by: i32,
}

impl From<SpectraCollection> for web_common::database::tables::SpectraCollection {
    fn from(item: SpectraCollection) -> Self {
        Self {
            id: item.id,
            sample_id: item.sample_id,
            created_by: item.created_by,
        }
    }
}

impl From<web_common::database::tables::SpectraCollection> for SpectraCollection {
    fn from(item: web_common::database::tables::SpectraCollection) -> Self {
        Self {
            id: item.id,
            sample_id: item.sample_id,
            created_by: item.created_by,
        }
    }
}

impl SpectraCollection {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::spectra_collection;
        let query = spectra_collection::dsl::spectra_collection;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(spectra_collection::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(spectra_collection::table)
            .values(self)
            .on_conflict(spectra_collection::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(spectra_collection::dsl::spectra_collection
            .filter(spectra_collection::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
    /// * `connection` - The connection to the database.
    ///
    pub fn get(
        id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::spectra_collection;
        spectra_collection::dsl::spectra_collection
            .filter(spectra_collection::dsl::id.eq(id))
            .first::<Self>(connection)
    }
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = team_states)]
pub struct TeamState {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub font_awesome_icon: String,
    pub icon_color: String,
}

impl From<TeamState> for web_common::database::tables::TeamState {
    fn from(item: TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl From<web_common::database::tables::TeamState> for TeamState {
    fn from(item: web_common::database::tables::TeamState) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            font_awesome_icon: item.font_awesome_icon,
            icon_color: item.icon_color,
        }
    }
}

impl TeamState {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::team_states;
        let query = team_states::dsl::team_states;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(team_states::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(team_states::table)
            .values(self)
            .on_conflict(team_states::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(team_states::dsl::team_states
            .filter(team_states::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM team_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM team_states ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, font_awesome_icon, icon_color FROM team_states ",
            "WHERE $1 <<% f_concat_team_states_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_team_states_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_team_id: Option<i32>,
}

impl From<Team> for web_common::database::tables::Team {
    fn from(item: Team) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            parent_team_id: item.parent_team_id,
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
        }
    }
}

impl Team {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::teams;
        let query = teams::dsl::teams;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(teams::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(teams::table)
            .values(self)
            .on_conflict(teams::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(teams::dsl::teams
            .filter(teams::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id FROM teams ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id FROM teams ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, name, description, parent_team_id FROM teams ",
            "WHERE $1 <<% f_concat_teams_name_description(name, description) ",
            "ORDER BY strict_word_similarity($1, f_concat_teams_name_description(name, description)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::units;
        let query = units::dsl::units;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(units::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(units::table)
            .values(self)
            .on_conflict(units::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(units::dsl::units
            .filter(units::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
}
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
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
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::user_emails;
        let query = user_emails::dsl::user_emails;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(user_emails::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(user_emails::table)
            .values(self)
            .on_conflict(user_emails::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(user_emails::dsl::user_emails
            .filter(user_emails::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
#[derive(Queryable, Debug, Eq, PartialEq, Clone, Serialize, Deserialize, Identifiable, QueryableByName, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
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
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

impl User {
    /// Get all of the structs from the database.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of structs to retrieve.
    /// * `connection` - The connection to the database.
    ///
    pub fn all(
        limit: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::users;
        let query = users::dsl::users;
        if let Some(limit) = limit {
            query.limit(limit).load::<Self>(connection)
        } else {
            query.load::<Self>(connection)
        }
    }
    /// Insert the struct into the database.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(connection)
    }
    /// Insert the struct into the database or update it if it already exists.
    ///
    /// # Arguments
    /// * `connection` - The connection to the database.
    ///
    pub fn insert_or_update(
        &self,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .on_conflict(users::dsl::id)
            .do_update()
            .set(self)
            .get_result(connection)
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
        diesel::delete(users::dsl::users
            .filter(users::dsl::id.eq(self.id))
        )
        .execute(connection)
    }
    /// Get the struct from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the struct to get.
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at FROM users ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at FROM users ",
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
            return Self::all(Some(limit as i64), connection);
        }
        let similarity_query = concat!(
            "SELECT id, first_name, middle_name, last_name, created_at, updated_at FROM users ",
            "WHERE $1 <<% f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text) ",
            "ORDER BY strict_word_similarity($1, f_concat_users_name((first_name)::text, (middle_name)::text, (last_name)::text)) DESC LIMIT $2",
        );
        diesel::sql_query(similarity_query)
            .bind::<diesel::sql_types::Text, _>(query)
            .bind::<diesel::sql_types::Integer, _>(limit)
            .load(connection)
}
}
