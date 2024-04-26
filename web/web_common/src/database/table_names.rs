//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    ContainerHorizontalRules,
    SampledIndividualBioOttTaxonItems,
    Colors,
    PrimaryUserEmails,
    UserEmails,
    Units,
    SampleStates,
    ItemCategoryRelationships,
    Spectra,
    LoginProviders,
    BioOttRanks,
    ManufacturedItemCategories,
    Notifications,
    Users,
    ItemUnits,
    BioOttTaxonItems,
    ItemCategoryUnits,
    Locations,
    ItemContinuousQuantities,
    ItemLocations,
    Documents,
    ContainerVerticalRules,
    Projects,
    Organizations,
    ItemCategories,
    DocumentFormats,
    Items,
    Roles,
    SampleBioOttTaxonItems,
    SampledIndividuals,
    PublicUsers,
    ItemDiscreteQuantities,
    SamplingProcedures,
    DiscreteUnits,
    DerivedSamples,
    ProcedureContinuousRequirements,
    ContinuousUnits,
    Samples,
    ProcedureDiscreteRequirements,
    ProjectStates,
    Teams,
    ProjectRequirements,
    Procedures,
    FontAwesomeIcons,
    SpectraCollection,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::Colors => "colors",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::UserEmails => "user_emails",
            Table::Units => "units",
            Table::SampleStates => "sample_states",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::Spectra => "spectra",
            Table::LoginProviders => "login_providers",
            Table::BioOttRanks => "bio_ott_ranks",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Notifications => "notifications",
            Table::Users => "users",
            Table::ItemUnits => "item_units",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Locations => "locations",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ItemLocations => "item_locations",
            Table::Documents => "documents",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::Projects => "projects",
            Table::Organizations => "organizations",
            Table::ItemCategories => "item_categories",
            Table::DocumentFormats => "document_formats",
            Table::Items => "items",
            Table::Roles => "roles",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::SampledIndividuals => "sampled_individuals",
            Table::PublicUsers => "public_users",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::SamplingProcedures => "sampling_procedures",
            Table::DiscreteUnits => "discrete_units",
            Table::DerivedSamples => "derived_samples",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ContinuousUnits => "continuous_units",
            Table::Samples => "samples",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ProjectStates => "project_states",
            Table::Teams => "teams",
            Table::ProjectRequirements => "project_requirements",
            Table::Procedures => "procedures",
            Table::FontAwesomeIcons => "font_awesome_icons",
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
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "colors" => Ok(Table::Colors),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "user_emails" => Ok(Table::UserEmails),
            "units" => Ok(Table::Units),
            "sample_states" => Ok(Table::SampleStates),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "spectra" => Ok(Table::Spectra),
            "login_providers" => Ok(Table::LoginProviders),
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "notifications" => Ok(Table::Notifications),
            "users" => Ok(Table::Users),
            "item_units" => Ok(Table::ItemUnits),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "locations" => Ok(Table::Locations),
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "item_locations" => Ok(Table::ItemLocations),
            "documents" => Ok(Table::Documents),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "projects" => Ok(Table::Projects),
            "organizations" => Ok(Table::Organizations),
            "item_categories" => Ok(Table::ItemCategories),
            "document_formats" => Ok(Table::DocumentFormats),
            "items" => Ok(Table::Items),
            "roles" => Ok(Table::Roles),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "public_users" => Ok(Table::PublicUsers),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "discrete_units" => Ok(Table::DiscreteUnits),
            "derived_samples" => Ok(Table::DerivedSamples),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "samples" => Ok(Table::Samples),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
            "project_states" => Ok(Table::ProjectStates),
            "teams" => Ok(Table::Teams),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "procedures" => Ok(Table::Procedures),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "spectra_collection" => Ok(Table::SpectraCollection),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
