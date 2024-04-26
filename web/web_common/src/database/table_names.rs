//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    ItemCategoryUnits,
    Documents,
    Units,
    SampleStates,
    ItemContinuousQuantities,
    SamplingProcedures,
    DocumentFormats,
    ProjectStates,
    Samples,
    Items,
    UserEmails,
    FontAwesomeIcons,
    DerivedSamples,
    Spectra,
    Organizations,
    SampledIndividuals,
    BioOttRanks,
    ManufacturedItemCategories,
    ContainerHorizontalRules,
    Procedures,
    Notifications,
    DiscreteUnits,
    Colors,
    PrimaryUserEmails,
    ProjectRequirements,
    SampledIndividualBioOttTaxonItems,
    PublicUsers,
    ProcedureDiscreteRequirements,
    Roles,
    Projects,
    Users,
    SampleBioOttTaxonItems,
    ContinuousUnits,
    ItemUnits,
    ItemLocations,
    ProcedureContinuousRequirements,
    Locations,
    LoginProviders,
    ItemCategoryRelationships,
    ItemCategories,
    ItemDiscreteQuantities,
    Teams,
    ContainerVerticalRules,
    BioOttTaxonItems,
    SpectraCollection,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::ItemCategoryUnits => "item_category_units",
            Table::Documents => "documents",
            Table::Units => "units",
            Table::SampleStates => "sample_states",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::SamplingProcedures => "sampling_procedures",
            Table::DocumentFormats => "document_formats",
            Table::ProjectStates => "project_states",
            Table::Samples => "samples",
            Table::Items => "items",
            Table::UserEmails => "user_emails",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::DerivedSamples => "derived_samples",
            Table::Spectra => "spectra",
            Table::Organizations => "organizations",
            Table::SampledIndividuals => "sampled_individuals",
            Table::BioOttRanks => "bio_ott_ranks",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::Procedures => "procedures",
            Table::Notifications => "notifications",
            Table::DiscreteUnits => "discrete_units",
            Table::Colors => "colors",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ProjectRequirements => "project_requirements",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::PublicUsers => "public_users",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::Roles => "roles",
            Table::Projects => "projects",
            Table::Users => "users",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::ContinuousUnits => "continuous_units",
            Table::ItemUnits => "item_units",
            Table::ItemLocations => "item_locations",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::Locations => "locations",
            Table::LoginProviders => "login_providers",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemCategories => "item_categories",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::Teams => "teams",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::SpectraCollection => "spectra_collection",
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
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "documents" => Ok(Table::Documents),
            "units" => Ok(Table::Units),
            "sample_states" => Ok(Table::SampleStates),
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "document_formats" => Ok(Table::DocumentFormats),
            "project_states" => Ok(Table::ProjectStates),
            "samples" => Ok(Table::Samples),
            "items" => Ok(Table::Items),
            "user_emails" => Ok(Table::UserEmails),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "derived_samples" => Ok(Table::DerivedSamples),
            "spectra" => Ok(Table::Spectra),
            "organizations" => Ok(Table::Organizations),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "procedures" => Ok(Table::Procedures),
            "notifications" => Ok(Table::Notifications),
            "discrete_units" => Ok(Table::DiscreteUnits),
            "colors" => Ok(Table::Colors),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "public_users" => Ok(Table::PublicUsers),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
            "roles" => Ok(Table::Roles),
            "projects" => Ok(Table::Projects),
            "users" => Ok(Table::Users),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "item_units" => Ok(Table::ItemUnits),
            "item_locations" => Ok(Table::ItemLocations),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            "locations" => Ok(Table::Locations),
            "login_providers" => Ok(Table::LoginProviders),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "item_categories" => Ok(Table::ItemCategories),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "teams" => Ok(Table::Teams),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "spectra_collection" => Ok(Table::SpectraCollection),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
