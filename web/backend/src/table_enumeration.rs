//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::models::*;
use crate::nested_models::*;
use crate::views::*;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use crate::new_variants::InsertRow;
use crate::update_variants::UpdateRow;

/// Trait providing the search method for the Table enum.
pub trait SearchableTable {
    /// Search the table by the query using the similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
    /// Search the table by the query using the word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn word_similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
    /// Search the table by the query using the strict_word_similarity method from PostgreSQL.
    ///
    /// # Arguments
    /// * `query` - The string to search for.
    /// * `limit` - The maximum number of results, by default `10`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized vector of the rows of the table, using bincode.
    fn strict_word_similarity_search(
         &self,
         query: &str,
         limit: Option<i32>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
}

impl SearchableTable for web_common::database::Table {
    fn similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContainerHorizontalRules => unimplemented!("Table `container_horizontal_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContainerVerticalRules => unimplemented!("Table `container_vertical_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContinuousUnits => unimplemented!("Table `continuous_units` does not have a GIN similarity index."),
            web_common::database::Table::Countries => Country::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DiscreteUnits => unimplemented!("Table `discrete_units` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => DocumentFormat::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Documents => unimplemented!("Table `documents` does not have a GIN similarity index."),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategories => NestedItemCategory::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategoryRelationships => unimplemented!("Table `item_category_relationships` does not have a GIN similarity index."),
            web_common::database::Table::ItemCategoryUnits => unimplemented!("Table `item_category_units` does not have a GIN similarity index."),
            web_common::database::Table::ItemLocations => unimplemented!("Table `item_locations` does not have a GIN similarity index."),
            web_common::database::Table::ItemUnits => unimplemented!("Table `item_units` does not have a GIN similarity index."),
            web_common::database::Table::Items => unimplemented!("Table `items` does not have a GIN similarity index."),
            web_common::database::Table::Locations => unimplemented!("Table `locations` does not have a GIN similarity index."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::ManufacturedItemCategories => unimplemented!("Table `manufactured_item_categories` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => Organization::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have a GIN similarity index."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => unimplemented!("Table `sampled_individuals` does not have a GIN similarity index."),
            web_common::database::Table::Samples => unimplemented!("Table `samples` does not have a GIN similarity index."),
            web_common::database::Table::SamplingProcedures => NestedSamplingProcedure::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Units => Unit::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
        }
    }
    fn word_similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContainerHorizontalRules => unimplemented!("Table `container_horizontal_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContainerVerticalRules => unimplemented!("Table `container_vertical_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContinuousUnits => unimplemented!("Table `continuous_units` does not have a GIN similarity index."),
            web_common::database::Table::Countries => Country::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DiscreteUnits => unimplemented!("Table `discrete_units` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => DocumentFormat::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Documents => unimplemented!("Table `documents` does not have a GIN similarity index."),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategories => NestedItemCategory::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategoryRelationships => unimplemented!("Table `item_category_relationships` does not have a GIN similarity index."),
            web_common::database::Table::ItemCategoryUnits => unimplemented!("Table `item_category_units` does not have a GIN similarity index."),
            web_common::database::Table::ItemLocations => unimplemented!("Table `item_locations` does not have a GIN similarity index."),
            web_common::database::Table::ItemUnits => unimplemented!("Table `item_units` does not have a GIN similarity index."),
            web_common::database::Table::Items => unimplemented!("Table `items` does not have a GIN similarity index."),
            web_common::database::Table::Locations => unimplemented!("Table `locations` does not have a GIN similarity index."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::ManufacturedItemCategories => unimplemented!("Table `manufactured_item_categories` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => Organization::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have a GIN similarity index."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => unimplemented!("Table `sampled_individuals` does not have a GIN similarity index."),
            web_common::database::Table::Samples => unimplemented!("Table `samples` does not have a GIN similarity index."),
            web_common::database::Table::SamplingProcedures => NestedSamplingProcedure::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Units => Unit::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
        }
    }
    fn strict_word_similarity_search(&self, query: &str, limit: Option<i32>, connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContainerHorizontalRules => unimplemented!("Table `container_horizontal_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContainerVerticalRules => unimplemented!("Table `container_vertical_rules` does not have a GIN similarity index."),
            web_common::database::Table::ContinuousUnits => unimplemented!("Table `continuous_units` does not have a GIN similarity index."),
            web_common::database::Table::Countries => Country::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => unimplemented!("Table `derived_samples` does not have a GIN similarity index."),
            web_common::database::Table::DiscreteUnits => unimplemented!("Table `discrete_units` does not have a GIN similarity index."),
            web_common::database::Table::DocumentFormats => DocumentFormat::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Documents => unimplemented!("Table `documents` does not have a GIN similarity index."),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategories => NestedItemCategory::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategoryRelationships => unimplemented!("Table `item_category_relationships` does not have a GIN similarity index."),
            web_common::database::Table::ItemCategoryUnits => unimplemented!("Table `item_category_units` does not have a GIN similarity index."),
            web_common::database::Table::ItemLocations => unimplemented!("Table `item_locations` does not have a GIN similarity index."),
            web_common::database::Table::ItemUnits => unimplemented!("Table `item_units` does not have a GIN similarity index."),
            web_common::database::Table::Items => unimplemented!("Table `items` does not have a GIN similarity index."),
            web_common::database::Table::Locations => unimplemented!("Table `locations` does not have a GIN similarity index."),
            web_common::database::Table::LoginProviders => unimplemented!("Table `login_providers` does not have a GIN similarity index."),
            web_common::database::Table::ManufacturedItemCategories => unimplemented!("Table `manufactured_item_categories` does not have a GIN similarity index."),
            web_common::database::Table::Notifications => unimplemented!("Table `notifications` does not have a GIN similarity index."),
            web_common::database::Table::Organizations => Organization::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Roles => unimplemented!("Table `roles` does not have a GIN similarity index."),
            web_common::database::Table::SampleBioOttTaxonItems => unimplemented!("Table `sample_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampleStates => NestedSampleState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unimplemented!("Table `sampled_individual_bio_ott_taxon_items` does not have a GIN similarity index."),
            web_common::database::Table::SampledIndividuals => unimplemented!("Table `sampled_individuals` does not have a GIN similarity index."),
            web_common::database::Table::Samples => unimplemented!("Table `samples` does not have a GIN similarity index."),
            web_common::database::Table::SamplingProcedures => NestedSamplingProcedure::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Spectra => unimplemented!("Table `spectra` does not have a GIN similarity index."),
            web_common::database::Table::SpectraCollections => unimplemented!("Table `spectra_collections` does not have a GIN similarity index."),
            web_common::database::Table::TeamStates => NestedTeamState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Units => Unit::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => unimplemented!("Table `user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Users => User::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
        }
    }
}
/// Trait providing the get method for the Table enum.
pub trait IdentifiableTable {
    /// Get the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A serialized version of the row of the table, using bincode.
    fn get(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl IdentifiableTable for web_common::database::Table {

    fn get(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => bincode::serialize(&NestedBioOttRank::get(primary_key.into(), connection)?)?,
            web_common::database::Table::BioOttTaxonItems => bincode::serialize(&NestedBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Colors => bincode::serialize(&Color::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ContainerHorizontalRules => bincode::serialize(&NestedContainerHorizontalRule::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ContainerVerticalRules => bincode::serialize(&NestedContainerVerticalRule::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ContinuousUnits => bincode::serialize(&ContinuousUnit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Countries => bincode::serialize(&Country::get(primary_key.into(), connection)?)?,
            web_common::database::Table::DerivedSamples => bincode::serialize(&NestedDerivedSample::get(primary_key.into(), connection)?)?,
            web_common::database::Table::DiscreteUnits => bincode::serialize(&DiscreteUnit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::DocumentFormats => bincode::serialize(&DocumentFormat::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Documents => bincode::serialize(&NestedDocument::get(primary_key.into(), connection)?)?,
            web_common::database::Table::FontAwesomeIcons => bincode::serialize(&FontAwesomeIcon::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ItemCategories => bincode::serialize(&NestedItemCategory::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ItemCategoryRelationships => bincode::serialize(&NestedItemCategoryRelationship::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ItemCategoryUnits => bincode::serialize(&NestedItemCategoryUnit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ItemLocations => bincode::serialize(&NestedItemLocation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ItemUnits => bincode::serialize(&NestedItemUnit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Items => bincode::serialize(&NestedItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Locations => bincode::serialize(&NestedLocation::get(primary_key.into(), connection)?)?,
            web_common::database::Table::LoginProviders => bincode::serialize(&NestedLoginProvider::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ManufacturedItemCategories => bincode::serialize(&NestedManufacturedItemCategory::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Notifications => bincode::serialize(&NestedNotification::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Organizations => bincode::serialize(&Organization::get(primary_key.into(), connection)?)?,
            web_common::database::Table::PrimaryUserEmails => bincode::serialize(&PrimaryUserEmail::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Procedures => bincode::serialize(&NestedProcedure::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectRequirements => bincode::serialize(&NestedProjectRequirement::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectStates => bincode::serialize(&NestedProjectState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Projects => bincode::serialize(&NestedProject::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Roles => bincode::serialize(&Role::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleBioOttTaxonItems => bincode::serialize(&NestedSampleBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleStates => bincode::serialize(&NestedSampleState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => bincode::serialize(&NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividuals => bincode::serialize(&NestedSampledIndividual::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Samples => bincode::serialize(&NestedSample::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SamplingProcedures => bincode::serialize(&NestedSamplingProcedure::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Spectra => bincode::serialize(&NestedSpectra::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SpectraCollections => bincode::serialize(&NestedSpectraCollection::get(primary_key.into(), connection)?)?,
            web_common::database::Table::TeamStates => bincode::serialize(&NestedTeamState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Teams => bincode::serialize(&NestedTeam::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Units => bincode::serialize(&Unit::get(primary_key.into(), connection)?)?,
            web_common::database::Table::UserEmails => bincode::serialize(&NestedUserEmail::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Users => bincode::serialize(&User::get(primary_key.into(), connection)?)?,
        })
    }
}
/// Trait providing the delete method for the Table enum.
pub trait DeletableTable {
    /// Delete the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The number of rows deleted.
    fn delete(
         &self,
         primary_key: web_common::database::operations::PrimaryKey,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<usize, web_common::api::ApiError>;
}

impl DeletableTable for web_common::database::Table {

    fn delete(
        &self,
        primary_key: web_common::database::operations::PrimaryKey,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<usize, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => BioOttRank::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::BioOttTaxonItems => BioOttTaxonItem::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Colors => Color::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ContainerHorizontalRules => ContainerHorizontalRule::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ContainerVerticalRules => ContainerVerticalRule::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ContinuousUnits => ContinuousUnit::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Countries => Country::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::DerivedSamples => DerivedSample::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::DiscreteUnits => DiscreteUnit::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::DocumentFormats => DocumentFormat::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Documents => Document::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ItemCategories => ItemCategory::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ItemCategoryRelationships => ItemCategoryRelationship::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ItemCategoryUnits => ItemCategoryUnit::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ItemLocations => ItemLocation::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ItemUnits => ItemUnit::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Items => Item::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Locations => Location::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::LoginProviders => LoginProvider::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ManufacturedItemCategories => ManufacturedItemCategory::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Notifications => Notification::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Organizations => Organization::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::PrimaryUserEmails => PrimaryUserEmail::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Procedures => Procedure::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ProjectRequirements => ProjectRequirement::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::ProjectStates => ProjectState::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Projects => Project::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Roles => Role::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SampleBioOttTaxonItems => SampleBioOttTaxonItem::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SampleStates => SampleState::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => SampledIndividualBioOttTaxonItem::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SampledIndividuals => SampledIndividual::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Samples => Sample::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SamplingProcedures => SamplingProcedure::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Spectra => Spectra::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::SpectraCollections => SpectraCollection::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::TeamStates => TeamState::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Teams => Team::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Units => Unit::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::UserEmails => UserEmail::delete_by_id(primary_key.into(), connection)?,
            web_common::database::Table::Users => User::delete_by_id(primary_key.into(), connection)?,
        })
    }
}
/// Trait providing the all method for the Table enum.
pub trait AllTable {
    /// Get all the rows from the table.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    fn all(
         &self,
         limit: Option<i64>,
         offset: Option<i64>,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<Vec<u8>>, web_common::api::ApiError>;
}

impl AllTable for web_common::database::Table {

    fn all(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<Vec<u8>>, web_common::api::ApiError> {
        match self {
            web_common::database::Table::BioOttRanks => NestedBioOttRank::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::BioOttTaxonItems => NestedBioOttTaxonItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Colors => Color::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContainerHorizontalRules => NestedContainerHorizontalRule::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContainerVerticalRules => NestedContainerVerticalRule::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ContinuousUnits => ContinuousUnit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Countries => Country::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DerivedSamples => NestedDerivedSample::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DiscreteUnits => DiscreteUnit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::DocumentFormats => DocumentFormat::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Documents => NestedDocument::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::FontAwesomeIcons => FontAwesomeIcon::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategories => NestedItemCategory::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategoryRelationships => NestedItemCategoryRelationship::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemCategoryUnits => NestedItemCategoryUnit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemLocations => NestedItemLocation::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ItemUnits => NestedItemUnit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Items => NestedItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Locations => NestedLocation::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::LoginProviders => NestedLoginProvider::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ManufacturedItemCategories => NestedManufacturedItemCategory::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Notifications => NestedNotification::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Organizations => Organization::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => PrimaryUserEmail::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Procedures => NestedProcedure::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectRequirements => NestedProjectRequirement::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectStates => NestedProjectState::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Roles => Role::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleBioOttTaxonItems => NestedSampleBioOttTaxonItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleStates => NestedSampleState::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => NestedSampledIndividualBioOttTaxonItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividuals => NestedSampledIndividual::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Samples => NestedSample::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SamplingProcedures => NestedSamplingProcedure::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Spectra => NestedSpectra::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SpectraCollections => NestedSpectraCollection::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::TeamStates => NestedTeamState::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Teams => NestedTeam::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Units => Unit::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::UserEmails => NestedUserEmail::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Users => User::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
        }
    }
}
/// Trait providing the insert method for the Table enum.
pub trait InsertableTable {
    /// Insert a new row into the table.
    ///
    /// # Arguments
    /// * `row` - The bincode-serialized row of the table.
    /// * `user_id` - The id of the user inserting the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn insert(
         &self,
         row: Vec<u8>,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl InsertableTable for web_common::database::Table {

    fn insert(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ContainerHorizontalRules => {
                let row: web_common::database::NewContainerHorizontalRule = bincode::deserialize::<web_common::database::NewContainerHorizontalRule>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::ContainerHorizontalRule = <web_common::database::NewContainerHorizontalRule as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedContainerHorizontalRule::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContainerVerticalRules => {
                let row: web_common::database::NewContainerVerticalRule = bincode::deserialize::<web_common::database::NewContainerVerticalRule>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::ContainerVerticalRule = <web_common::database::NewContainerVerticalRule as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedContainerVerticalRule::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContinuousUnits => unreachable!("Table `continuous_units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::DerivedSamples => todo!("Insert not implemented for derived_samples."),
            web_common::database::Table::DiscreteUnits => unreachable!("Table `discrete_units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Documents => unreachable!("Table `documents` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ItemCategories => {
                let row: web_common::database::NewItemCategory = bincode::deserialize::<web_common::database::NewItemCategory>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::ItemCategory = <web_common::database::NewItemCategory as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedItemCategory::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemCategoryRelationships => unreachable!("Table `item_category_relationships` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ItemCategoryUnits => unreachable!("Table `item_category_units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ItemLocations => unreachable!("Table `item_locations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ItemUnits => unreachable!("Table `item_units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Items => unreachable!("Table `items` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Locations => unreachable!("Table `locations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::ManufacturedItemCategories => unreachable!("Table `manufactured_item_categories` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::PrimaryUserEmails => unreachable!("Table `primary_user_emails` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Procedures => {
                let row: web_common::database::NewProcedure = bincode::deserialize::<web_common::database::NewProcedure>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Procedure = <web_common::database::NewProcedure as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProcedure::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectRequirements => {
                let row: web_common::database::NewProjectRequirement = bincode::deserialize::<web_common::database::NewProjectRequirement>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::ProjectRequirement = <web_common::database::NewProjectRequirement as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProjectRequirement::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::NewProject = bincode::deserialize::<web_common::database::NewProject>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Project = <web_common::database::NewProject as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Roles => unreachable!("Table `roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleBioOttTaxonItems => todo!("Insert not implemented for sample_bio_ott_taxon_items."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => todo!("Insert not implemented for sampled_individual_bio_ott_taxon_items."),
            web_common::database::Table::SampledIndividuals => {
                let row: web_common::database::NewSampledIndividual = bincode::deserialize::<web_common::database::NewSampledIndividual>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::SampledIndividual = <web_common::database::NewSampledIndividual as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampledIndividual::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Sample = <web_common::database::NewSample as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplingProcedures => {
                let row: web_common::database::NewSamplingProcedure = bincode::deserialize::<web_common::database::NewSamplingProcedure>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::SamplingProcedure = <web_common::database::NewSamplingProcedure as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSamplingProcedure::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SpectraCollections => todo!("Insert not implemented for spectra_collections."),
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::NewTeam = bincode::deserialize::<web_common::database::NewTeam>(&row).map_err(web_common::api::ApiError::from)?;
                let inserted_row: crate::models::Team = <web_common::database::NewTeam as InsertRow>::insert(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(inserted_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Units => unreachable!("Table `units` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::UserEmails => unreachable!("Table `user_emails` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Users => unreachable!("Table `users` is not insertable as it does not have a known column associated to a creator user id."),
})
    }
}
/// Trait providing the update method for the Table enum.
pub trait UpdatableTable {
    /// Update a row in the table.
    ///
    /// # Arguments
    /// * `row` - The bincode-serialized row of the table.
    /// * `user_id` - The id of the user updating the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn update(
         &self,
         row: Vec<u8>,
         user_id: i32,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl UpdatableTable for web_common::database::Table {

    fn update(
        &self,
        row: Vec<u8>,
        user_id: i32,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => unreachable!("Table `bio_ott_ranks` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::BioOttTaxonItems => unreachable!("Table `bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Colors => unreachable!("Table `colors` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ContainerHorizontalRules => {
                let row: web_common::database::UpdateContainerHorizontalRule = bincode::deserialize::<web_common::database::UpdateContainerHorizontalRule>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::ContainerHorizontalRule = <web_common::database::UpdateContainerHorizontalRule as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedContainerHorizontalRule::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContainerVerticalRules => {
                let row: web_common::database::UpdateContainerVerticalRule = bincode::deserialize::<web_common::database::UpdateContainerVerticalRule>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::ContainerVerticalRule = <web_common::database::UpdateContainerVerticalRule as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedContainerVerticalRule::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContinuousUnits => unreachable!("Table `continuous_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Countries => unreachable!("Table `countries` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DerivedSamples => todo!("Update not implemented for derived_samples."),
            web_common::database::Table::DiscreteUnits => unreachable!("Table `discrete_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Documents => unreachable!("Table `documents` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ItemCategories => {
                let row: web_common::database::UpdateItemCategory = bincode::deserialize::<web_common::database::UpdateItemCategory>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::ItemCategory = <web_common::database::UpdateItemCategory as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedItemCategory::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemCategoryRelationships => unreachable!("Table `item_category_relationships` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ItemCategoryUnits => unreachable!("Table `item_category_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ItemLocations => unreachable!("Table `item_locations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ItemUnits => unreachable!("Table `item_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Items => unreachable!("Table `items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Locations => unreachable!("Table `locations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::LoginProviders => unreachable!("Table `login_providers` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ManufacturedItemCategories => unreachable!("Table `manufactured_item_categories` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Notifications => unreachable!("Table `notifications` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Organizations => unreachable!("Table `organizations` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::PrimaryUserEmails => unreachable!("Table `primary_user_emails` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Procedures => {
                let row: web_common::database::UpdateProcedure = bincode::deserialize::<web_common::database::UpdateProcedure>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Procedure = <web_common::database::UpdateProcedure as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProcedure::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectRequirements => {
                let row: web_common::database::UpdateProjectRequirement = bincode::deserialize::<web_common::database::UpdateProjectRequirement>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::ProjectRequirement = <web_common::database::UpdateProjectRequirement as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProjectRequirement::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::UpdateProject = bincode::deserialize::<web_common::database::UpdateProject>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Project = <web_common::database::UpdateProject as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Roles => unreachable!("Table `roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleBioOttTaxonItems => todo!("Update not implemented for sample_bio_ott_taxon_items."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => todo!("Update not implemented for sampled_individual_bio_ott_taxon_items."),
            web_common::database::Table::SampledIndividuals => {
                let row: web_common::database::NewSampledIndividual = bincode::deserialize::<web_common::database::NewSampledIndividual>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::SampledIndividual = <web_common::database::NewSampledIndividual as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSampledIndividual::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Samples => {
                let row: web_common::database::NewSample = bincode::deserialize::<web_common::database::NewSample>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Sample = <web_common::database::NewSample as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSample::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplingProcedures => {
                let row: web_common::database::NewSamplingProcedure = bincode::deserialize::<web_common::database::NewSamplingProcedure>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::SamplingProcedure = <web_common::database::NewSamplingProcedure as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedSamplingProcedure::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollections => todo!("Update not implemented for spectra_collections."),
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Teams => {
                let row: web_common::database::UpdateTeam = bincode::deserialize::<web_common::database::UpdateTeam>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Team = <web_common::database::UpdateTeam as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedTeam::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Units => unreachable!("Table `units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UserEmails => unreachable!("Table `user_emails` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Users => {
                let row: web_common::database::UpdateUser = bincode::deserialize::<web_common::database::UpdateUser>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::User = <web_common::database::UpdateUser as UpdateRow>::update(row, user_id, connection)?;
                 bincode::serialize(&updated_row).map_err(web_common::api::ApiError::from)?
            },
})
    }
}
/// Trait providing the from_flat_str method for the Table enum.
pub trait FromFlatStrTable {
    /// Convert a JSON serialized row of the flat variant of the table to the richest struct.
    ///
    /// # Arguments
    /// * `row` - The JSON serialized row of the table.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    fn from_flat_str(
         &self,
         row: &str,
         connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
) -> Result<Vec<u8>, web_common::api::ApiError>;
}

impl FromFlatStrTable for web_common::database::Table {

    fn from_flat_str(
        &self,
        row: &str,
        connection: &mut PooledConnection<ConnectionManager<diesel::prelude::PgConnection>>
    ) -> Result<Vec<u8>, web_common::api::ApiError> {
        Ok(match self {
            web_common::database::Table::BioOttRanks => {
                let flat_row: crate::models::BioOttRank = serde_json::from_str::<crate::models::BioOttRank>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedBioOttRank::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::BioOttTaxonItems => {
                let flat_row: crate::models::BioOttTaxonItem = serde_json::from_str::<crate::models::BioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Colors => bincode::serialize(&serde_json::from_str::<crate::models::Color>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::ContainerHorizontalRules => {
                let flat_row: crate::models::ContainerHorizontalRule = serde_json::from_str::<crate::models::ContainerHorizontalRule>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedContainerHorizontalRule::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContainerVerticalRules => {
                let flat_row: crate::models::ContainerVerticalRule = serde_json::from_str::<crate::models::ContainerVerticalRule>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedContainerVerticalRule::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ContinuousUnits => bincode::serialize(&serde_json::from_str::<crate::models::ContinuousUnit>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::Countries => bincode::serialize(&serde_json::from_str::<crate::models::Country>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::DerivedSamples => {
                let flat_row: crate::models::DerivedSample = serde_json::from_str::<crate::models::DerivedSample>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedDerivedSample::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::DiscreteUnits => bincode::serialize(&serde_json::from_str::<crate::models::DiscreteUnit>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::DocumentFormats => bincode::serialize(&serde_json::from_str::<crate::models::DocumentFormat>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::Documents => {
                let flat_row: crate::models::Document = serde_json::from_str::<crate::models::Document>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedDocument::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::FontAwesomeIcons => bincode::serialize(&serde_json::from_str::<crate::models::FontAwesomeIcon>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::ItemCategories => {
                let flat_row: crate::models::ItemCategory = serde_json::from_str::<crate::models::ItemCategory>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItemCategory::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemCategoryRelationships => {
                let flat_row: crate::models::ItemCategoryRelationship = serde_json::from_str::<crate::models::ItemCategoryRelationship>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItemCategoryRelationship::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemCategoryUnits => {
                let flat_row: crate::models::ItemCategoryUnit = serde_json::from_str::<crate::models::ItemCategoryUnit>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItemCategoryUnit::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemLocations => {
                let flat_row: crate::models::ItemLocation = serde_json::from_str::<crate::models::ItemLocation>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItemLocation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ItemUnits => {
                let flat_row: crate::models::ItemUnit = serde_json::from_str::<crate::models::ItemUnit>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItemUnit::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Items => {
                let flat_row: crate::models::Item = serde_json::from_str::<crate::models::Item>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Locations => {
                let flat_row: crate::models::Location = serde_json::from_str::<crate::models::Location>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedLocation::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::LoginProviders => {
                let flat_row: crate::models::LoginProvider = serde_json::from_str::<crate::models::LoginProvider>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedLoginProvider::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ManufacturedItemCategories => {
                let flat_row: crate::models::ManufacturedItemCategory = serde_json::from_str::<crate::models::ManufacturedItemCategory>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedManufacturedItemCategory::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Notifications => {
                let flat_row: crate::models::Notification = serde_json::from_str::<crate::models::Notification>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedNotification::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Organizations => bincode::serialize(&serde_json::from_str::<crate::models::Organization>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::PrimaryUserEmails => bincode::serialize(&serde_json::from_str::<crate::models::PrimaryUserEmail>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::Procedures => {
                let flat_row: crate::models::Procedure = serde_json::from_str::<crate::models::Procedure>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProcedure::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectRequirements => {
                let flat_row: crate::models::ProjectRequirement = serde_json::from_str::<crate::models::ProjectRequirement>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectRequirement::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::ProjectStates => {
                let flat_row: crate::models::ProjectState = serde_json::from_str::<crate::models::ProjectState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProjectState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Projects => {
                let flat_row: crate::models::Project = serde_json::from_str::<crate::models::Project>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedProject::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Roles => bincode::serialize(&serde_json::from_str::<crate::models::Role>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::SampleBioOttTaxonItems => {
                let flat_row: crate::models::SampleBioOttTaxonItem = serde_json::from_str::<crate::models::SampleBioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampleBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampleStates => {
                let flat_row: crate::models::SampleState = serde_json::from_str::<crate::models::SampleState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampleState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividualBioOttTaxonItems => {
                let flat_row: crate::models::SampledIndividualBioOttTaxonItem = serde_json::from_str::<crate::models::SampledIndividualBioOttTaxonItem>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividualBioOttTaxonItem::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SampledIndividuals => {
                let flat_row: crate::models::SampledIndividual = serde_json::from_str::<crate::models::SampledIndividual>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSampledIndividual::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Samples => {
                let flat_row: crate::models::Sample = serde_json::from_str::<crate::models::Sample>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSample::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SamplingProcedures => {
                let flat_row: crate::models::SamplingProcedure = serde_json::from_str::<crate::models::SamplingProcedure>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSamplingProcedure::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Spectra => {
                let flat_row: crate::models::Spectra = serde_json::from_str::<crate::models::Spectra>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectra::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::SpectraCollections => {
                let flat_row: crate::models::SpectraCollection = serde_json::from_str::<crate::models::SpectraCollection>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedSpectraCollection::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::TeamStates => {
                let flat_row: crate::models::TeamState = serde_json::from_str::<crate::models::TeamState>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeamState::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Teams => {
                let flat_row: crate::models::Team = serde_json::from_str::<crate::models::Team>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedTeam::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Units => bincode::serialize(&serde_json::from_str::<crate::models::Unit>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
            web_common::database::Table::UserEmails => {
                let flat_row: crate::models::UserEmail = serde_json::from_str::<crate::models::UserEmail>(row).map_err(web_common::api::ApiError::from)?;
                let richest_row = crate::nested_models::NestedUserEmail::from_flat(flat_row, connection)?;
                 bincode::serialize(&richest_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::Users => bincode::serialize(&serde_json::from_str::<crate::models::User>(row).map_err(web_common::api::ApiError::from)?).map_err(web_common::api::ApiError::from)?,
        })
    }
}
