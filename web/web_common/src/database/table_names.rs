//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Users,
    ItemCategories,
    ItemDiscreteQuantities,
    ItemUnits,
    ItemContinuousQuantities,
    FontAwesomeIcons,
    SamplingProcedures,
    ProjectRequirements,
    BioOttTaxonItems,
    UserEmails,
    ItemCategoryRelationships,
    BioOttRanks,
    DocumentFormats,
    SampleBioOttTaxonItems,
    Locations,
    Documents,
    ContainerVerticalRules,
    ManufacturedItemCategories,
    DerivedSamples,
    ItemLocations,
    Roles,
    DiscreteUnits,
    SampledIndividuals,
    Units,
    PrimaryUserEmails,
    Procedures,
    SampledIndividualBioOttTaxonItems,
    Projects,
    Colors,
    SampleStates,
    ContinuousUnits,
    Spectra,
    ProcedureDiscreteRequirements,
    ItemCategoryUnits,
    SpectraCollection,
    Samples,
    LoginProviders,
    ContainerHorizontalRules,
    PublicUsers,
    ProcedureContinuousRequirements,
    Organizations,
    ProjectStates,
    Teams,
    Notifications,
    Items,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Users => "users",
            Table::ItemCategories => "item_categories",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ItemUnits => "item_units",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::SamplingProcedures => "sampling_procedures",
            Table::ProjectRequirements => "project_requirements",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::UserEmails => "user_emails",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::BioOttRanks => "bio_ott_ranks",
            Table::DocumentFormats => "document_formats",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::Locations => "locations",
            Table::Documents => "documents",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::DerivedSamples => "derived_samples",
            Table::ItemLocations => "item_locations",
            Table::Roles => "roles",
            Table::DiscreteUnits => "discrete_units",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Units => "units",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Procedures => "procedures",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::Projects => "projects",
            Table::Colors => "colors",
            Table::SampleStates => "sample_states",
            Table::ContinuousUnits => "continuous_units",
            Table::Spectra => "spectra",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SpectraCollection => "spectra_collection",
            Table::Samples => "samples",
            Table::LoginProviders => "login_providers",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::PublicUsers => "public_users",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::Organizations => "organizations",
            Table::ProjectStates => "project_states",
            Table::Teams => "teams",
            Table::Notifications => "notifications",
            Table::Items => "items",
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
            "users" => Ok(Table::Users),
            "item_categories" => Ok(Table::ItemCategories),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "item_units" => Ok(Table::ItemUnits),
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "user_emails" => Ok(Table::UserEmails),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "document_formats" => Ok(Table::DocumentFormats),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "locations" => Ok(Table::Locations),
            "documents" => Ok(Table::Documents),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "derived_samples" => Ok(Table::DerivedSamples),
            "item_locations" => Ok(Table::ItemLocations),
            "roles" => Ok(Table::Roles),
            "discrete_units" => Ok(Table::DiscreteUnits),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "units" => Ok(Table::Units),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "procedures" => Ok(Table::Procedures),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "projects" => Ok(Table::Projects),
            "colors" => Ok(Table::Colors),
            "sample_states" => Ok(Table::SampleStates),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "spectra" => Ok(Table::Spectra),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "spectra_collection" => Ok(Table::SpectraCollection),
            "samples" => Ok(Table::Samples),
            "login_providers" => Ok(Table::LoginProviders),
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "public_users" => Ok(Table::PublicUsers),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            "organizations" => Ok(Table::Organizations),
            "project_states" => Ok(Table::ProjectStates),
            "teams" => Ok(Table::Teams),
            "notifications" => Ok(Table::Notifications),
            "items" => Ok(Table::Items),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
