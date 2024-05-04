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
            web_common::database::Table::Organizations => NestedOrganization::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PublicUsers => NestedPublicUser::similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
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
            web_common::database::Table::Organizations => NestedOrganization::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PublicUsers => NestedPublicUser::word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
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
            web_common::database::Table::Organizations => NestedOrganization::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => unimplemented!("Table `primary_user_emails` does not have a GIN similarity index."),
            web_common::database::Table::Procedures => unimplemented!("Table `procedures` does not have a GIN similarity index."),
            web_common::database::Table::ProjectRequirements => unimplemented!("Table `project_requirements` does not have a GIN similarity index."),
            web_common::database::Table::ProjectStates => NestedProjectState::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PublicUsers => NestedPublicUser::strict_word_similarity_search(query, limit, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
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
            web_common::database::Table::Organizations => bincode::serialize(&NestedOrganization::get(primary_key.into(), connection)?)?,
            web_common::database::Table::PrimaryUserEmails => bincode::serialize(&PrimaryUserEmail::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Procedures => bincode::serialize(&NestedProcedure::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectRequirements => bincode::serialize(&NestedProjectRequirement::get(primary_key.into(), connection)?)?,
            web_common::database::Table::ProjectStates => bincode::serialize(&NestedProjectState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Projects => bincode::serialize(&NestedProject::get(primary_key.into(), connection)?)?,
            web_common::database::Table::PublicUsers => bincode::serialize(&NestedPublicUser::get(primary_key.into(), connection)?)?,
            web_common::database::Table::Roles => bincode::serialize(&Role::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleBioOttTaxonItems => bincode::serialize(&NestedSampleBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampleStates => bincode::serialize(&NestedSampleState::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividualBioOttTaxonItems => bincode::serialize(&NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection)?)?,
            web_common::database::Table::SampledIndividuals => bincode::serialize(&SampledIndividual::get(primary_key.into(), connection)?)?,
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
            web_common::database::Table::PublicUsers => PublicUser::delete_by_id(primary_key.into(), connection)?,
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
            web_common::database::Table::Organizations => NestedOrganization::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PrimaryUserEmails => PrimaryUserEmail::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Procedures => NestedProcedure::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectRequirements => NestedProjectRequirement::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::ProjectStates => NestedProjectState::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Projects => NestedProject::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::PublicUsers => NestedPublicUser::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::Roles => Role::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleBioOttTaxonItems => NestedSampleBioOttTaxonItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampleStates => NestedSampleState::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => NestedSampledIndividualBioOttTaxonItem::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
            web_common::database::Table::SampledIndividuals => SampledIndividual::all(limit, offset, connection)?.iter().map(|row| bincode::serialize(row).map_err(web_common::api::ApiError::from)).collect(),
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
            web_common::database::Table::PublicUsers => unreachable!("Table `public_users` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampleBioOttTaxonItems => todo!("Insert not implemented for sample_bio_ott_taxon_items."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not insertable as it does not have a known column associated to a creator user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => todo!("Insert not implemented for sampled_individual_bio_ott_taxon_items."),
            web_common::database::Table::SampledIndividuals => unreachable!("Table `sampled_individuals` is not insertable as it does not have a known column associated to a creator user id."),
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
            web_common::database::Table::ContainerHorizontalRules => unreachable!("Table `container_horizontal_rules` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ContainerVerticalRules => unreachable!("Table `container_vertical_rules` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ContinuousUnits => unreachable!("Table `continuous_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DerivedSamples => unreachable!("Table `derived_samples` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DiscreteUnits => unreachable!("Table `discrete_units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::DocumentFormats => unreachable!("Table `document_formats` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Documents => unreachable!("Table `documents` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::FontAwesomeIcons => unreachable!("Table `font_awesome_icons` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ItemCategories => unreachable!("Table `item_categories` is not updatable as it does not have a known column associated to an updater user id."),
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
            web_common::database::Table::Procedures => unreachable!("Table `procedures` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectRequirements => unreachable!("Table `project_requirements` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::ProjectStates => unreachable!("Table `project_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Projects => {
                let row: web_common::database::UpdateProject = bincode::deserialize::<web_common::database::UpdateProject>(&row).map_err(web_common::api::ApiError::from)?;
                let updated_row: crate::models::Project = <web_common::database::UpdateProject as UpdateRow>::update(row, user_id, connection)?;
                let nested_row = crate::nested_models::NestedProject::from_flat(updated_row, connection)?;
                 bincode::serialize(&nested_row).map_err(web_common::api::ApiError::from)?
            },
            web_common::database::Table::PublicUsers => unreachable!("Table `public_users` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Roles => unreachable!("Table `roles` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleBioOttTaxonItems => unreachable!("Table `sample_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampleStates => unreachable!("Table `sample_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividualBioOttTaxonItems => unreachable!("Table `sampled_individual_bio_ott_taxon_items` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SampledIndividuals => unreachable!("Table `sampled_individuals` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Samples => unreachable!("Table `samples` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SamplingProcedures => unreachable!("Table `sampling_procedures` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Spectra => unreachable!("Table `spectra` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::SpectraCollections => unreachable!("Table `spectra_collections` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::TeamStates => unreachable!("Table `team_states` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Teams => unreachable!("Table `teams` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Units => unreachable!("Table `units` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::UserEmails => unreachable!("Table `user_emails` is not updatable as it does not have a known column associated to an updater user id."),
            web_common::database::Table::Users => unreachable!("Table `users` is not updatable as it does not have a known column associated to an updater user id."),
})
    }
}
