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
    ItemContinuousQuantities,
    ItemDiscreteQuantities,
    ItemLocations,
    ItemUnits,
    Items,
    Locations,
    LoginProviders,
    ManufacturedItemCategories,
    Notifications,
    Organizations,
    PrimaryUserEmails,
    ProcedureContinuousRequirements,
    ProcedureDiscreteRequirements,
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
    SpectraCollection,
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
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ItemLocations => "item_locations",
            Table::ItemUnits => "item_units",
            Table::Items => "items",
            Table::Locations => "locations",
            Table::LoginProviders => "login_providers",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Notifications => "notifications",
            Table::Organizations => "organizations",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
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
            Table::SpectraCollection => "spectra_collection",
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
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "item_locations" => Ok(Table::ItemLocations),
            "item_units" => Ok(Table::ItemUnits),
            "items" => Ok(Table::Items),
            "locations" => Ok(Table::Locations),
            "login_providers" => Ok(Table::LoginProviders),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "notifications" => Ok(Table::Notifications),
            "organizations" => Ok(Table::Organizations),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
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
            "spectra_collection" => Ok(Table::SpectraCollection),
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
            Table::ItemContinuousQuantities => {
                crate::database::ItemContinuousQuantity::delete_from_id(primary_key.into(), connection).await
            },
            Table::ItemDiscreteQuantities => {
                crate::database::ItemDiscreteQuantity::delete_from_id(primary_key.into(), connection).await
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
            Table::ProcedureContinuousRequirements => {
                crate::database::ProcedureContinuousRequirement::delete_from_id(primary_key.into(), connection).await
            },
            Table::ProcedureDiscreteRequirements => {
                crate::database::ProcedureDiscreteRequirement::delete_from_id(primary_key.into(), connection).await
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
            Table::SpectraCollection => {
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
            Table::ItemContinuousQuantities => crate::database::NestedItemContinuousQuantity::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemDiscreteQuantities => crate::database::NestedItemDiscreteQuantity::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemLocations => crate::database::NestedItemLocation::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ItemUnits => crate::database::NestedItemUnit::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Items => crate::database::NestedItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Locations => crate::database::NestedLocation::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::LoginProviders => crate::database::LoginProvider::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ManufacturedItemCategories => crate::database::NestedManufacturedItemCategory::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Notifications => crate::database::NestedNotification::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Organizations => crate::database::NestedOrganization::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::PrimaryUserEmails => crate::database::PrimaryUserEmail::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProcedureContinuousRequirements => crate::database::NestedProcedureContinuousRequirement::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProcedureDiscreteRequirements => crate::database::NestedProcedureDiscreteRequirement::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Procedures => crate::database::NestedProcedure::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectRequirements => crate::database::NestedProjectRequirement::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::ProjectStates => crate::database::ProjectState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Projects => crate::database::NestedProject::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::PublicUsers => crate::database::NestedPublicUser::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Roles => crate::database::Role::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampleStates => crate::database::SampleState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SampledIndividuals => crate::database::SampledIndividual::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Samples => crate::database::NestedSample::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SamplingProcedures => crate::database::NestedSamplingProcedure::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::Spectra => crate::database::NestedSpectra::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::SpectraCollection => crate::database::NestedSpectraCollection::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
            Table::TeamStates => crate::database::TeamState::get(primary_key.into(), connection).await?.map(|row| bincode::serialize(&row)).transpose()?,
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
            Table::ItemContinuousQuantities => crate::database::NestedItemContinuousQuantity::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemDiscreteQuantities => crate::database::NestedItemDiscreteQuantity::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemLocations => crate::database::NestedItemLocation::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ItemUnits => crate::database::NestedItemUnit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Items => crate::database::NestedItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Locations => crate::database::NestedLocation::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::LoginProviders => crate::database::LoginProvider::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ManufacturedItemCategories => crate::database::NestedManufacturedItemCategory::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Notifications => crate::database::NestedNotification::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Organizations => crate::database::NestedOrganization::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::PrimaryUserEmails => crate::database::PrimaryUserEmail::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProcedureContinuousRequirements => crate::database::NestedProcedureContinuousRequirement::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProcedureDiscreteRequirements => crate::database::NestedProcedureDiscreteRequirement::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Procedures => crate::database::NestedProcedure::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectRequirements => crate::database::NestedProjectRequirement::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::ProjectStates => crate::database::ProjectState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Projects => crate::database::NestedProject::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::PublicUsers => crate::database::NestedPublicUser::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Roles => crate::database::Role::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleBioOttTaxonItems => crate::database::NestedSampleBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampleStates => crate::database::SampleState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividualBioOttTaxonItems => crate::database::NestedSampledIndividualBioOttTaxonItem::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SampledIndividuals => crate::database::SampledIndividual::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Samples => crate::database::NestedSample::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SamplingProcedures => crate::database::NestedSamplingProcedure::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Spectra => crate::database::NestedSpectra::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::SpectraCollection => crate::database::NestedSpectraCollection::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::TeamStates => crate::database::TeamState::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Teams => crate::database::NestedTeam::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Units => crate::database::Unit::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::UserEmails => crate::database::NestedUserEmail::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
            Table::Users => crate::database::User::all(limit, offset, connection).await?.into_iter().map(|row| bincode::serialize(&row).map_err(crate::api::ApiError::from)).collect(),
        }
    }
}
