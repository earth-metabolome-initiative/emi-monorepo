//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    BioOttRanks,
    BioOttTaxonItems,
    Colors,
    ContainerHorizontalRules,
    ContainerVerticalRules,
    ContinuousUnits,
    DerivedSamples,
    DiscreteUnits,
    DocumentFormats,
    Documents,
    FontAwesomeIcons,
    ItemCategories,
    ItemCategoryRelationships,
    ItemCategoryUnits,
    ItemLocations,
    ItemUnits,
    Items,
    Locations,
    LoginProviders,
    ManufacturedItemCategories,
    Notifications,
    Organizations,
    PrimaryUserEmails,
    Procedures,
    ProjectRequirements,
    ProjectStates,
    Projects,
    PublicUsers,
    Roles,
    SampleBioOttTaxonItems,
    SampleStates,
    SampledIndividualBioOttTaxonItems,
    SampledIndividuals,
    Samples,
    SamplingProcedures,
    Spectra,
    SpectraCollections,
    TeamStates,
    Teams,
    Units,
    UserEmails,
    Users,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::BioOttRanks => "bio_ott_ranks",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::Colors => "colors",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ContinuousUnits => "continuous_units",
            Table::DerivedSamples => "derived_samples",
            Table::DiscreteUnits => "discrete_units",
            Table::DocumentFormats => "document_formats",
            Table::Documents => "documents",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::ItemCategories => "item_categories",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemCategoryUnits => "item_category_units",
            Table::ItemLocations => "item_locations",
            Table::ItemUnits => "item_units",
            Table::Items => "items",
            Table::Locations => "locations",
            Table::LoginProviders => "login_providers",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Notifications => "notifications",
            Table::Organizations => "organizations",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Procedures => "procedures",
            Table::ProjectRequirements => "project_requirements",
            Table::ProjectStates => "project_states",
            Table::Projects => "projects",
            Table::PublicUsers => "public_users",
            Table::Roles => "roles",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::SampleStates => "sample_states",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Samples => "samples",
            Table::SamplingProcedures => "sampling_procedures",
            Table::Spectra => "spectra",
            Table::SpectraCollections => "spectra_collections",
            Table::TeamStates => "team_states",
            Table::Teams => "teams",
            Table::Units => "units",
            Table::UserEmails => "user_emails",
            Table::Users => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl From<Table> for String {
    fn from(table: Table) -> Self {
        table.to_string()
    }
}
impl std::convert::TryFrom<&str> for Table {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "colors" => Ok(Table::Colors),
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "derived_samples" => Ok(Table::DerivedSamples),
            "discrete_units" => Ok(Table::DiscreteUnits),
            "document_formats" => Ok(Table::DocumentFormats),
            "documents" => Ok(Table::Documents),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "item_categories" => Ok(Table::ItemCategories),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "item_locations" => Ok(Table::ItemLocations),
            "item_units" => Ok(Table::ItemUnits),
            "items" => Ok(Table::Items),
            "locations" => Ok(Table::Locations),
            "login_providers" => Ok(Table::LoginProviders),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "notifications" => Ok(Table::Notifications),
            "organizations" => Ok(Table::Organizations),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "procedures" => Ok(Table::Procedures),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "project_states" => Ok(Table::ProjectStates),
            "projects" => Ok(Table::Projects),
            "public_users" => Ok(Table::PublicUsers),
            "roles" => Ok(Table::Roles),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "sample_states" => Ok(Table::SampleStates),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "samples" => Ok(Table::Samples),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "spectra" => Ok(Table::Spectra),
            "spectra_collections" => Ok(Table::SpectraCollections),
            "team_states" => Ok(Table::TeamStates),
            "teams" => Ok(Table::Teams),
            "units" => Ok(Table::Units),
            "user_emails" => Ok(Table::UserEmails),
            "users" => Ok(Table::Users),
            table_name => Err(format!("Unknown table name: {table_name}")),
        }
    }
}
impl std::convert::TryFrom<String> for Table {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
#[cfg(feature = "frontend")]
impl Table {
    /// Delete the row from the table.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The number of rows deleted.
    pub async fn delete<C>(
        &self,
        primary_key: crate::database::operations::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<usize, gluesql::prelude::Error> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                crate::database::BioOttRank::delete_from_id(primary_key.into(), connection).await
            },
            Table::BioOttTaxonItems => {
                crate::database::BioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::Colors => {
                crate::database::Color::delete_from_id(primary_key.into(), connection).await
            },
            Table::ContainerHorizontalRules => {
                crate::database::ContainerHorizontalRule::delete_from_id(primary_key.into(), connection).await
            },
            Table::ContainerVerticalRules => {
                crate::database::ContainerVerticalRule::delete_from_id(primary_key.into(), connection).await
            },
            Table::ContinuousUnits => {
                crate::database::ContinuousUnit::delete_from_id(primary_key.into(), connection).await
            },
            Table::DerivedSamples => {
                crate::database::DerivedSample::delete_from_id(primary_key.into(), connection).await
            },
            Table::DiscreteUnits => {
                crate::database::DiscreteUnit::delete_from_id(primary_key.into(), connection).await
            },
            Table::DocumentFormats => {
                crate::database::DocumentFormat::delete_from_id(primary_key.into(), connection).await
            },
            Table::Documents => {
                crate::database::Document::delete_from_id(primary_key.into(), connection).await
            },
            Table::FontAwesomeIcons => {
                crate::database::FontAwesomeIcon::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemCategories => {
                crate::database::ItemCategory::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemCategoryRelationships => {
                crate::database::ItemCategoryRelationship::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemCategoryUnits => {
                crate::database::ItemCategoryUnit::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemLocations => {
                crate::database::ItemLocation::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemUnits => {
                crate::database::ItemUnit::delete_from_id(primary_key.into(), connection).await
            },
            Table::Items => {
                crate::database::Item::delete_from_id(primary_key.into(), connection).await
            },
            Table::Locations => {
                crate::database::Location::delete_from_id(primary_key.into(), connection).await
            },
            Table::LoginProviders => {
                crate::database::LoginProvider::delete_from_id(primary_key.into(), connection).await
            },
            Table::ManufacturedItemCategories => {
                crate::database::ManufacturedItemCategory::delete_from_id(primary_key.into(), connection).await
            },
            Table::Notifications => {
                crate::database::Notification::delete_from_id(primary_key.into(), connection).await
            },
            Table::Organizations => {
                crate::database::Organization::delete_from_id(primary_key.into(), connection).await
            },
            Table::PrimaryUserEmails => {
                crate::database::PrimaryUserEmail::delete_from_id(primary_key.into(), connection).await
            },
            Table::Procedures => {
                crate::database::Procedure::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProjectRequirements => {
                crate::database::ProjectRequirement::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProjectStates => {
                crate::database::ProjectState::delete_from_id(primary_key.into(), connection).await
            },
            Table::Projects => {
                crate::database::Project::delete_from_id(primary_key.into(), connection).await
            },
            Table::PublicUsers => {
                crate::database::PublicUser::delete_from_id(primary_key.into(), connection).await
            },
            Table::Roles => {
                crate::database::Role::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampleBioOttTaxonItems => {
                crate::database::SampleBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampleStates => {
                crate::database::SampleState::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividualBioOttTaxonItems => {
                crate::database::SampledIndividualBioOttTaxonItem::delete_from_id(primary_key.into(), connection).await
            },
            Table::SampledIndividuals => {
                crate::database::SampledIndividual::delete_from_id(primary_key.into(), connection).await
            },
            Table::Samples => {
                crate::database::Sample::delete_from_id(primary_key.into(), connection).await
            },
            Table::SamplingProcedures => {
                crate::database::SamplingProcedure::delete_from_id(primary_key.into(), connection).await
            },
            Table::Spectra => {
                crate::database::Spectra::delete_from_id(primary_key.into(), connection).await
            },
            Table::SpectraCollections => {
                crate::database::SpectraCollection::delete_from_id(primary_key.into(), connection).await
            },
            Table::TeamStates => {
                crate::database::TeamState::delete_from_id(primary_key.into(), connection).await
            },
            Table::Teams => {
                crate::database::Team::delete_from_id(primary_key.into(), connection).await
            },
            Table::Units => {
                crate::database::Unit::delete_from_id(primary_key.into(), connection).await
            },
            Table::UserEmails => {
                crate::database::UserEmail::delete_from_id(primary_key.into(), connection).await
            },
            Table::Users => {
                crate::database::User::delete_from_id(primary_key.into(), connection).await
            },
        }
    }
    /// Get the row from the table by the primary key.
    ///
    /// # Arguments
    /// * `primary_key` - The primary key of the row.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The row of the table.
    pub async fn get<C>(
        &self,
        primary_key: crate::database::operations::PrimaryKey,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Option<Vec<u8>>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => crate::database::NestedBioOttRank::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::BioOttTaxonItems => crate::database::NestedBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Colors => crate::database::Color::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ContainerHorizontalRules => crate::database::NestedContainerHorizontalRule::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ContainerVerticalRules => crate::database::NestedContainerVerticalRule::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ContinuousUnits => crate::database::ContinuousUnit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::DerivedSamples => crate::database::NestedDerivedSample::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::DiscreteUnits => crate::database::DiscreteUnit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::DocumentFormats => crate::database::DocumentFormat::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Documents => crate::database::NestedDocument::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::FontAwesomeIcons => crate::database::FontAwesomeIcon::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemCategories => crate::database::NestedItemCategory::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemCategoryRelationships => crate::database::NestedItemCategoryRelationship::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemCategoryUnits => crate::database::NestedItemCategoryUnit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemLocations => crate::database::NestedItemLocation::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemUnits => crate::database::NestedItemUnit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Items => crate::database::NestedItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Locations => crate::database::NestedLocation::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::LoginProviders => crate::database::NestedLoginProvider::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ManufacturedItemCategories => crate::database::NestedManufacturedItemCategory::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Notifications => crate::database::NestedNotification::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Organizations => crate::database::NestedOrganization::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::PrimaryUserEmails => crate::database::PrimaryUserEmail::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Procedures => crate::database::NestedProcedure::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectRequirements => crate::database::NestedProjectRequirement::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectStates => crate::database::NestedProjectState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Projects => crate::database::NestedProject::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::PublicUsers => crate::database::NestedPublicUser::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Roles => crate::database::Role::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleStates => crate::database::NestedSampleState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividuals => crate::database::SampledIndividual::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Samples => crate::database::NestedSample::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SamplingProcedures => crate::database::NestedSamplingProcedure::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Spectra => crate::database::NestedSpectra::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SpectraCollections => crate::database::NestedSpectraCollection::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::TeamStates => crate::database::NestedTeamState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Teams => crate::database::NestedTeam::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Units => crate::database::Unit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::UserEmails => crate::database::NestedUserEmail::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Users => crate::database::User::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
        })
    }
    /// Get all the rows from the table.
    ///
    /// # Arguments
    /// * `limit` - The maximum number of rows to return.
    /// * `offset` - The number of rows to skip. By default `0`.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// A vector of the rows of the table.
    pub async fn all<C>(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<Vec<u8>>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => crate::database::NestedBioOttRank::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::BioOttTaxonItems => crate::database::NestedBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Colors => crate::database::Color::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ContainerHorizontalRules => crate::database::NestedContainerHorizontalRule::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ContainerVerticalRules => crate::database::NestedContainerVerticalRule::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ContinuousUnits => crate::database::ContinuousUnit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DerivedSamples => crate::database::NestedDerivedSample::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DiscreteUnits => crate::database::DiscreteUnit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::DocumentFormats => crate::database::DocumentFormat::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Documents => crate::database::NestedDocument::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::FontAwesomeIcons => crate::database::FontAwesomeIcon::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemCategories => crate::database::NestedItemCategory::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemCategoryRelationships => crate::database::NestedItemCategoryRelationship::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemCategoryUnits => crate::database::NestedItemCategoryUnit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemLocations => crate::database::NestedItemLocation::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemUnits => crate::database::NestedItemUnit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Items => crate::database::NestedItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Locations => crate::database::NestedLocation::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::LoginProviders => crate::database::NestedLoginProvider::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ManufacturedItemCategories => crate::database::NestedManufacturedItemCategory::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Notifications => crate::database::NestedNotification::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Organizations => crate::database::NestedOrganization::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::PrimaryUserEmails => crate::database::PrimaryUserEmail::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Procedures => crate::database::NestedProcedure::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectRequirements => crate::database::NestedProjectRequirement::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectStates => crate::database::NestedProjectState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Projects => crate::database::NestedProject::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::PublicUsers => crate::database::NestedPublicUser::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Roles => crate::database::Role::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleStates => crate::database::NestedSampleState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividuals => crate::database::SampledIndividual::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Samples => crate::database::NestedSample::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplingProcedures => crate::database::NestedSamplingProcedure::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Spectra => crate::database::NestedSpectra::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollections => crate::database::NestedSpectraCollection::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamStates => crate::database::NestedTeamState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Teams => crate::database::NestedTeam::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Units => crate::database::Unit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::UserEmails => crate::database::NestedUserEmail::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Users => crate::database::User::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
        }
    }
    /// Insert a new row into the table.
    ///
    /// # Arguments
    /// * `new_row` - The bincode-serialized row of the table.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    pub async fn insert<C>(
        &self,
        new_row: Vec<u8>,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Insert not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("Insert not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("Insert not implemented for colors."),
            Table::ContainerHorizontalRules => unimplemented!("Insert not implemented for container_horizontal_rules in frontend as it does not have a UUID primary key."),
            Table::ContainerVerticalRules => unimplemented!("Insert not implemented for container_vertical_rules in frontend as it does not have a UUID primary key."),
            Table::ContinuousUnits => unimplemented!("Insert not implemented for continuous_units."),
            Table::DerivedSamples => unimplemented!("Insert not implemented for derived_samples in frontend as it does not have a UUID primary key."),
            Table::DiscreteUnits => unimplemented!("Insert not implemented for discrete_units."),
            Table::DocumentFormats => unimplemented!("Insert not implemented for document_formats."),
            Table::Documents => unimplemented!("Insert not implemented for documents."),
            Table::FontAwesomeIcons => unimplemented!("Insert not implemented for font_awesome_icons."),
            Table::ItemCategories => unimplemented!("Insert not implemented for item_categories in frontend as it does not have a UUID primary key."),
            Table::ItemCategoryRelationships => unimplemented!("Insert not implemented for item_category_relationships."),
            Table::ItemCategoryUnits => unimplemented!("Insert not implemented for item_category_units."),
            Table::ItemLocations => unimplemented!("Insert not implemented for item_locations."),
            Table::ItemUnits => unimplemented!("Insert not implemented for item_units."),
            Table::Items => unimplemented!("Insert not implemented for items."),
            Table::Locations => unimplemented!("Insert not implemented for locations."),
            Table::LoginProviders => unimplemented!("Insert not implemented for login_providers."),
            Table::ManufacturedItemCategories => unimplemented!("Insert not implemented for manufactured_item_categories."),
            Table::Notifications => unimplemented!("Insert not implemented for notifications."),
            Table::Organizations => unimplemented!("Insert not implemented for organizations."),
            Table::PrimaryUserEmails => unimplemented!("Insert not implemented for primary_user_emails."),
            Table::Procedures => unimplemented!("Insert not implemented for procedures in frontend as it does not have a UUID primary key."),
            Table::ProjectRequirements => unimplemented!("Insert not implemented for project_requirements in frontend as it does not have a UUID primary key."),
            Table::ProjectStates => unimplemented!("Insert not implemented for project_states."),
            Table::Projects => unimplemented!("Insert not implemented for projects in frontend as it does not have a UUID primary key."),
            Table::PublicUsers => unimplemented!("Insert not implemented for public_users."),
            Table::Roles => unimplemented!("Insert not implemented for roles."),
            Table::SampleBioOttTaxonItems => todo!("Insert not implemented for sample_bio_ott_taxon_items."),
            Table::SampleStates => unimplemented!("Insert not implemented for sample_states."),
            Table::SampledIndividualBioOttTaxonItems => todo!("Insert not implemented for sampled_individual_bio_ott_taxon_items."),
            Table::SampledIndividuals => unimplemented!("Insert not implemented for sampled_individuals."),
            Table::Samples => {
                let new_row: super::NewSample = bincode::deserialize::<super::NewSample>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::Sample = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedSample::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::SamplingProcedures => {
                let new_row: super::NewSamplingProcedure = bincode::deserialize::<super::NewSamplingProcedure>(&new_row).map_err(crate::api::ApiError::from)?;
                let inserted_row: super::SamplingProcedure = new_row.insert(user_id, connection).await?;
                let nested_row = super::NestedSamplingProcedure::from_flat(inserted_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::Spectra => unimplemented!("Insert not implemented for spectra."),
            Table::SpectraCollections => unimplemented!("Insert not implemented for spectra_collections in frontend as it does not have a UUID primary key."),
            Table::TeamStates => unimplemented!("Insert not implemented for team_states."),
            Table::Teams => unimplemented!("Insert not implemented for teams in frontend as it does not have a UUID primary key."),
            Table::Units => unimplemented!("Insert not implemented for units."),
            Table::UserEmails => unimplemented!("Insert not implemented for user_emails."),
            Table::Users => unimplemented!("Insert not implemented for users."),
})
    }
    /// Update a row in the table.
    ///
    /// # Arguments
    /// * `update_row` - The bincode-serialized row of the table.
    /// * `user_id` - The user ID of the user performing the operation.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// The bincode-serialized row of the table.
    pub async fn update<C>(
        &self,
        update_row: Vec<u8>,
        user_id: i32,
        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<Vec<u8>, crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        Ok(match self {
            Table::BioOttRanks => unimplemented!("Update not implemented for bio_ott_ranks."),
            Table::BioOttTaxonItems => unimplemented!("Update not implemented for bio_ott_taxon_items."),
            Table::Colors => unimplemented!("Update not implemented for colors."),
            Table::ContainerHorizontalRules => unimplemented!("Update not implemented for container_horizontal_rules."),
            Table::ContainerVerticalRules => unimplemented!("Update not implemented for container_vertical_rules."),
            Table::ContinuousUnits => unimplemented!("Update not implemented for continuous_units."),
            Table::DerivedSamples => unimplemented!("Update not implemented for derived_samples."),
            Table::DiscreteUnits => unimplemented!("Update not implemented for discrete_units."),
            Table::DocumentFormats => unimplemented!("Update not implemented for document_formats."),
            Table::Documents => unimplemented!("Update not implemented for documents."),
            Table::FontAwesomeIcons => unimplemented!("Update not implemented for font_awesome_icons."),
            Table::ItemCategories => unimplemented!("Update not implemented for item_categories."),
            Table::ItemCategoryRelationships => unimplemented!("Update not implemented for item_category_relationships."),
            Table::ItemCategoryUnits => unimplemented!("Update not implemented for item_category_units."),
            Table::ItemLocations => unimplemented!("Update not implemented for item_locations."),
            Table::ItemUnits => unimplemented!("Update not implemented for item_units."),
            Table::Items => unimplemented!("Update not implemented for items."),
            Table::Locations => unimplemented!("Update not implemented for locations."),
            Table::LoginProviders => unimplemented!("Update not implemented for login_providers."),
            Table::ManufacturedItemCategories => unimplemented!("Update not implemented for manufactured_item_categories."),
            Table::Notifications => unimplemented!("Update not implemented for notifications."),
            Table::Organizations => unimplemented!("Update not implemented for organizations."),
            Table::PrimaryUserEmails => unimplemented!("Update not implemented for primary_user_emails."),
            Table::Procedures => unimplemented!("Update not implemented for procedures."),
            Table::ProjectRequirements => unimplemented!("Update not implemented for project_requirements."),
            Table::ProjectStates => unimplemented!("Update not implemented for project_states."),
            Table::Projects => {
                let update_row: super::UpdateProject = bincode::deserialize::<super::UpdateProject>(&update_row).map_err(crate::api::ApiError::from)?;
                let id = update_row.id;
                update_row.update(user_id, connection).await?;
                let updated_row: super::Project = super::Project::get(id, connection).await?.unwrap();
                let nested_row = super::NestedProject::from_flat(updated_row, connection).await?;
                 bincode::serialize(&nested_row).map_err(crate::api::ApiError::from)?
            },
            Table::PublicUsers => unimplemented!("Update not implemented for public_users."),
            Table::Roles => unimplemented!("Update not implemented for roles."),
            Table::SampleBioOttTaxonItems => unimplemented!("Update not implemented for sample_bio_ott_taxon_items."),
            Table::SampleStates => unimplemented!("Update not implemented for sample_states."),
            Table::SampledIndividualBioOttTaxonItems => unimplemented!("Update not implemented for sampled_individual_bio_ott_taxon_items."),
            Table::SampledIndividuals => unimplemented!("Update not implemented for sampled_individuals."),
            Table::Samples => unimplemented!("Update not implemented for samples."),
            Table::SamplingProcedures => unimplemented!("Update not implemented for sampling_procedures."),
            Table::Spectra => unimplemented!("Update not implemented for spectra."),
            Table::SpectraCollections => unimplemented!("Update not implemented for spectra_collections."),
            Table::TeamStates => unimplemented!("Update not implemented for team_states."),
            Table::Teams => unimplemented!("Update not implemented for teams."),
            Table::Units => unimplemented!("Update not implemented for units."),
            Table::UserEmails => unimplemented!("Update not implemented for user_emails."),
            Table::Users => unimplemented!("Update not implemented for users."),
})
    }
    /// Update or insert a row into the table.
    ///
    /// # Arguments
    /// * `rows` - The bincode-serialized rows of the table.
    /// * `connection` - The database connection.
    ///
    /// # Returns
    /// An empty tuple.
    pub async fn update_or_insert<C>(
        &self,
        rows: Vec<Vec<u8>>,        connection: &mut gluesql::prelude::Glue<C>,
    ) -> Result<(), crate::api::ApiError> where
        C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    {
        match self {
            Table::BioOttRanks => {
                for row in rows {
                    let row: super::NestedBioOttRank = bincode::deserialize::<super::NestedBioOttRank>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::BioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedBioOttTaxonItem = bincode::deserialize::<super::NestedBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Colors => {
                for row in rows {
                    let row: super::Color = bincode::deserialize::<super::Color>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ContainerHorizontalRules => {
                for row in rows {
                    let row: super::NestedContainerHorizontalRule = bincode::deserialize::<super::NestedContainerHorizontalRule>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ContainerVerticalRules => {
                for row in rows {
                    let row: super::NestedContainerVerticalRule = bincode::deserialize::<super::NestedContainerVerticalRule>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ContinuousUnits => {
                for row in rows {
                    let row: super::ContinuousUnit = bincode::deserialize::<super::ContinuousUnit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::DerivedSamples => {
                for row in rows {
                    let row: super::NestedDerivedSample = bincode::deserialize::<super::NestedDerivedSample>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::DiscreteUnits => {
                for row in rows {
                    let row: super::DiscreteUnit = bincode::deserialize::<super::DiscreteUnit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::DocumentFormats => {
                for row in rows {
                    let row: super::DocumentFormat = bincode::deserialize::<super::DocumentFormat>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Documents => {
                for row in rows {
                    let row: super::NestedDocument = bincode::deserialize::<super::NestedDocument>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::FontAwesomeIcons => {
                for row in rows {
                    let row: super::FontAwesomeIcon = bincode::deserialize::<super::FontAwesomeIcon>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ItemCategories => {
                for row in rows {
                    let row: super::NestedItemCategory = bincode::deserialize::<super::NestedItemCategory>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ItemCategoryRelationships => {
                for row in rows {
                    let row: super::NestedItemCategoryRelationship = bincode::deserialize::<super::NestedItemCategoryRelationship>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ItemCategoryUnits => {
                for row in rows {
                    let row: super::NestedItemCategoryUnit = bincode::deserialize::<super::NestedItemCategoryUnit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ItemLocations => {
                for row in rows {
                    let row: super::NestedItemLocation = bincode::deserialize::<super::NestedItemLocation>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ItemUnits => {
                for row in rows {
                    let row: super::NestedItemUnit = bincode::deserialize::<super::NestedItemUnit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Items => {
                for row in rows {
                    let row: super::NestedItem = bincode::deserialize::<super::NestedItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Locations => {
                for row in rows {
                    let row: super::NestedLocation = bincode::deserialize::<super::NestedLocation>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::LoginProviders => {
                for row in rows {
                    let row: super::NestedLoginProvider = bincode::deserialize::<super::NestedLoginProvider>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ManufacturedItemCategories => {
                for row in rows {
                    let row: super::NestedManufacturedItemCategory = bincode::deserialize::<super::NestedManufacturedItemCategory>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Notifications => {
                for row in rows {
                    let row: super::NestedNotification = bincode::deserialize::<super::NestedNotification>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Organizations => {
                for row in rows {
                    let row: super::NestedOrganization = bincode::deserialize::<super::NestedOrganization>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::PrimaryUserEmails => {
                for row in rows {
                    let row: super::PrimaryUserEmail = bincode::deserialize::<super::PrimaryUserEmail>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Procedures => {
                for row in rows {
                    let row: super::NestedProcedure = bincode::deserialize::<super::NestedProcedure>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ProjectRequirements => {
                for row in rows {
                    let row: super::NestedProjectRequirement = bincode::deserialize::<super::NestedProjectRequirement>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::ProjectStates => {
                for row in rows {
                    let row: super::NestedProjectState = bincode::deserialize::<super::NestedProjectState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Projects => {
                for row in rows {
                    let row: super::NestedProject = bincode::deserialize::<super::NestedProject>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::PublicUsers => {
                for row in rows {
                    let row: super::NestedPublicUser = bincode::deserialize::<super::NestedPublicUser>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Roles => {
                for row in rows {
                    let row: super::Role = bincode::deserialize::<super::Role>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampleBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedSampleBioOttTaxonItem = bincode::deserialize::<super::NestedSampleBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampleStates => {
                for row in rows {
                    let row: super::NestedSampleState = bincode::deserialize::<super::NestedSampleState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividualBioOttTaxonItems => {
                for row in rows {
                    let row: super::NestedSampledIndividualBioOttTaxonItem = bincode::deserialize::<super::NestedSampledIndividualBioOttTaxonItem>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SampledIndividuals => {
                for row in rows {
                    let row: super::SampledIndividual = bincode::deserialize::<super::SampledIndividual>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Samples => {
                for row in rows {
                    let row: super::NestedSample = bincode::deserialize::<super::NestedSample>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SamplingProcedures => {
                for row in rows {
                    let row: super::NestedSamplingProcedure = bincode::deserialize::<super::NestedSamplingProcedure>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Spectra => {
                for row in rows {
                    let row: super::NestedSpectra = bincode::deserialize::<super::NestedSpectra>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::SpectraCollections => {
                for row in rows {
                    let row: super::NestedSpectraCollection = bincode::deserialize::<super::NestedSpectraCollection>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::TeamStates => {
                for row in rows {
                    let row: super::NestedTeamState = bincode::deserialize::<super::NestedTeamState>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Teams => {
                for row in rows {
                    let row: super::NestedTeam = bincode::deserialize::<super::NestedTeam>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Units => {
                for row in rows {
                    let row: super::Unit = bincode::deserialize::<super::Unit>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::UserEmails => {
                for row in rows {
                    let row: super::NestedUserEmail = bincode::deserialize::<super::NestedUserEmail>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
            Table::Users => {
                for row in rows {
                    let row: super::User = bincode::deserialize::<super::User>(&row).map_err(crate::api::ApiError::from)?;
                    row.update_or_insert(connection).await?;
                }
            },
        }
    Ok(())}
}
