//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    DiscreteUnits,
    ItemDiscreteQuantities,
    DocumentFormats,
    UserEmails,
    Items,
    Projects,
    ProjectStates,
    ItemUnits,
    ContainerHorizontalRules,
    ContinuousUnits,
    Organizations,
    LoginProviders,
    BioOttRanks,
    BioOttTaxonItems,
    ManufacturedItemCategories,
    SpectraCollection,
    ItemCategoryUnits,
    SamplingProcedures,
    PublicUsers,
    PrimaryUserEmails,
    ItemCategories,
    Samples,
    Roles,
    Documents,
    FontAwesomeIcons,
    ContainerVerticalRules,
    DerivedSamples,
    Units,
    Locations,
    SampledIndividualBioOttTaxonItems,
    ItemCategoryRelationships,
    ItemLocations,
    Teams,
    Users,
    Notifications,
    ProcedureDiscreteRequirements,
    ProjectRequirements,
    Spectra,
    ItemContinuousQuantities,
    Procedures,
    SampleStates,
    SampleBioOttTaxonItems,
    Colors,
    SampledIndividuals,
    ProcedureContinuousRequirements,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::DiscreteUnits => "discrete_units",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::DocumentFormats => "document_formats",
            Table::UserEmails => "user_emails",
            Table::Items => "items",
            Table::Projects => "projects",
            Table::ProjectStates => "project_states",
            Table::ItemUnits => "item_units",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ContinuousUnits => "continuous_units",
            Table::Organizations => "organizations",
            Table::LoginProviders => "login_providers",
            Table::BioOttRanks => "bio_ott_ranks",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::SpectraCollection => "spectra_collection",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SamplingProcedures => "sampling_procedures",
            Table::PublicUsers => "public_users",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ItemCategories => "item_categories",
            Table::Samples => "samples",
            Table::Roles => "roles",
            Table::Documents => "documents",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::DerivedSamples => "derived_samples",
            Table::Units => "units",
            Table::Locations => "locations",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemLocations => "item_locations",
            Table::Teams => "teams",
            Table::Users => "users",
            Table::Notifications => "notifications",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ProjectRequirements => "project_requirements",
            Table::Spectra => "spectra",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Procedures => "procedures",
            Table::SampleStates => "sample_states",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::Colors => "colors",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
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
            "discrete_units" => Ok(Table::DiscreteUnits),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "document_formats" => Ok(Table::DocumentFormats),
            "user_emails" => Ok(Table::UserEmails),
            "items" => Ok(Table::Items),
            "projects" => Ok(Table::Projects),
            "project_states" => Ok(Table::ProjectStates),
            "item_units" => Ok(Table::ItemUnits),
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "organizations" => Ok(Table::Organizations),
            "login_providers" => Ok(Table::LoginProviders),
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "spectra_collection" => Ok(Table::SpectraCollection),
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "public_users" => Ok(Table::PublicUsers),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "item_categories" => Ok(Table::ItemCategories),
            "samples" => Ok(Table::Samples),
            "roles" => Ok(Table::Roles),
            "documents" => Ok(Table::Documents),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "derived_samples" => Ok(Table::DerivedSamples),
            "units" => Ok(Table::Units),
            "locations" => Ok(Table::Locations),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "item_locations" => Ok(Table::ItemLocations),
            "teams" => Ok(Table::Teams),
            "users" => Ok(Table::Users),
            "notifications" => Ok(Table::Notifications),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "spectra" => Ok(Table::Spectra),
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "procedures" => Ok(Table::Procedures),
            "sample_states" => Ok(Table::SampleStates),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "colors" => Ok(Table::Colors),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
