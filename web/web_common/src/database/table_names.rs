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
            "teams" => Ok(Table::Teams),
            "units" => Ok(Table::Units),
            "user_emails" => Ok(Table::UserEmails),
            "users" => Ok(Table::Users),
            table_name => Err(format!("Unknown table name: {}", table_name)),
        }
    }
}
