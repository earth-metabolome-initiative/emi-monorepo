//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Documents,
    Colors,
    ItemDiscreteQuantities,
    DiscreteUnits,
    Spectra,
    PrimaryUserEmails,
    FontAwesomeIcons,
    Items,
    DocumentFormats,
    ItemCategories,
    Projects,
    ContinuousUnits,
    ProcedureDiscreteRequirements,
    ManufacturedItemCategories,
    BioOttTaxonItems,
    Notifications,
    Procedures,
    SamplingProcedures,
    SpectraCollection,
    BioOttRanks,
    Locations,
    Samples,
    SampledIndividualBioOttTaxonItems,
    Teams,
    ItemLocations,
    ProjectStates,
    ItemUnits,
    ProcedureContinuousRequirements,
    ContainerVerticalRules,
    ItemCategoryRelationships,
    LoginProviders,
    SampledIndividuals,
    ItemCategoryUnits,
    SampleStates,
    Units,
    PublicUsers,
    ContainerHorizontalRules,
    Roles,
    ItemContinuousQuantities,
    DerivedSamples,
    ProjectRequirements,
    SampleBioOttTaxonItems,
    Users,
    UserEmails,
    Organizations,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Documents => "documents",
            Table::Colors => "colors",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::DiscreteUnits => "discrete_units",
            Table::Spectra => "spectra",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::FontAwesomeIcons => "font_awesome_icons",
            Table::Items => "items",
            Table::DocumentFormats => "document_formats",
            Table::ItemCategories => "item_categories",
            Table::Projects => "projects",
            Table::ContinuousUnits => "continuous_units",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::BioOttTaxonItems => "bio_ott_taxon_items",
            Table::Notifications => "notifications",
            Table::Procedures => "procedures",
            Table::SamplingProcedures => "sampling_procedures",
            Table::SpectraCollection => "spectra_collection",
            Table::BioOttRanks => "bio_ott_ranks",
            Table::Locations => "locations",
            Table::Samples => "samples",
            Table::SampledIndividualBioOttTaxonItems => "sampled_individual_bio_ott_taxon_items",
            Table::Teams => "teams",
            Table::ItemLocations => "item_locations",
            Table::ProjectStates => "project_states",
            Table::ItemUnits => "item_units",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::LoginProviders => "login_providers",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SampleStates => "sample_states",
            Table::Units => "units",
            Table::PublicUsers => "public_users",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::Roles => "roles",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::DerivedSamples => "derived_samples",
            Table::ProjectRequirements => "project_requirements",
            Table::SampleBioOttTaxonItems => "sample_bio_ott_taxon_items",
            Table::Users => "users",
            Table::UserEmails => "user_emails",
            Table::Organizations => "organizations",
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
            "documents" => Ok(Table::Documents),
            "colors" => Ok(Table::Colors),
            "item_discrete_quantities" => Ok(Table::ItemDiscreteQuantities),
            "discrete_units" => Ok(Table::DiscreteUnits),
            "spectra" => Ok(Table::Spectra),
            "primary_user_emails" => Ok(Table::PrimaryUserEmails),
            "font_awesome_icons" => Ok(Table::FontAwesomeIcons),
            "items" => Ok(Table::Items),
            "document_formats" => Ok(Table::DocumentFormats),
            "item_categories" => Ok(Table::ItemCategories),
            "projects" => Ok(Table::Projects),
            "continuous_units" => Ok(Table::ContinuousUnits),
            "procedure_discrete_requirements" => Ok(Table::ProcedureDiscreteRequirements),
            "manufactured_item_categories" => Ok(Table::ManufacturedItemCategories),
            "bio_ott_taxon_items" => Ok(Table::BioOttTaxonItems),
            "notifications" => Ok(Table::Notifications),
            "procedures" => Ok(Table::Procedures),
            "sampling_procedures" => Ok(Table::SamplingProcedures),
            "spectra_collection" => Ok(Table::SpectraCollection),
            "bio_ott_ranks" => Ok(Table::BioOttRanks),
            "locations" => Ok(Table::Locations),
            "samples" => Ok(Table::Samples),
            "sampled_individual_bio_ott_taxon_items" => Ok(Table::SampledIndividualBioOttTaxonItems),
            "teams" => Ok(Table::Teams),
            "item_locations" => Ok(Table::ItemLocations),
            "project_states" => Ok(Table::ProjectStates),
            "item_units" => Ok(Table::ItemUnits),
            "procedure_continuous_requirements" => Ok(Table::ProcedureContinuousRequirements),
            "container_vertical_rules" => Ok(Table::ContainerVerticalRules),
            "item_category_relationships" => Ok(Table::ItemCategoryRelationships),
            "login_providers" => Ok(Table::LoginProviders),
            "sampled_individuals" => Ok(Table::SampledIndividuals),
            "item_category_units" => Ok(Table::ItemCategoryUnits),
            "sample_states" => Ok(Table::SampleStates),
            "units" => Ok(Table::Units),
            "public_users" => Ok(Table::PublicUsers),
            "container_horizontal_rules" => Ok(Table::ContainerHorizontalRules),
            "roles" => Ok(Table::Roles),
            "item_continuous_quantities" => Ok(Table::ItemContinuousQuantities),
            "derived_samples" => Ok(Table::DerivedSamples),
            "project_requirements" => Ok(Table::ProjectRequirements),
            "sample_bio_ott_taxon_items" => Ok(Table::SampleBioOttTaxonItems),
            "users" => Ok(Table::Users),
            "user_emails" => Ok(Table::UserEmails),
            "organizations" => Ok(Table::Organizations),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
