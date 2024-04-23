//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    ItemCategoryUnits,
    PublicUser,
    ProcedureContinuousRequirements,
    ItemUnits,
    SampleTaxa,
    Organizations,
    SamplingProcedures,
    SampledIndividuals,
    Units,
    DiscreteUnits,
    ProjectStates,
    Teams,
    LoginProviders,
    DocumentFormats,
    ItemLocations,
    Notifications,
    ContainerHorizontalRules,
    ItemContinuousQuantities,
    Procedures,
    Users,
    Projects,
    Spectra,
    SampledIndividualTaxa,
    DerivedSamples,
    ContinuousUnits,
    Samples,
    SampleStates,
    Taxa,
    ItemCategoryRelationships,
    ItemDiscreteQuantities,
    ItemCategories,
    ManufacturedItemCategories,
    Locations,
    ProcedureDiscreteRequirements,
    UserEmails,
    Roles,
    SpectraCollection,
    ProjectRequirements,
    Documents,
    Items,
    PrimaryUserEmails,
    ContainerVerticalRules,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::ItemCategoryUnits => "item_category_units",
            Table::PublicUser => "public_user",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ItemUnits => "item_units",
            Table::SampleTaxa => "sample_taxa",
            Table::Organizations => "organizations",
            Table::SamplingProcedures => "sampling_procedures",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Units => "units",
            Table::DiscreteUnits => "discrete_units",
            Table::ProjectStates => "project_states",
            Table::Teams => "teams",
            Table::LoginProviders => "login_providers",
            Table::DocumentFormats => "document_formats",
            Table::ItemLocations => "item_locations",
            Table::Notifications => "notifications",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Procedures => "procedures",
            Table::Users => "users",
            Table::Projects => "projects",
            Table::Spectra => "spectra",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::DerivedSamples => "derived_samples",
            Table::ContinuousUnits => "continuous_units",
            Table::Samples => "samples",
            Table::SampleStates => "sample_states",
            Table::Taxa => "taxa",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ItemCategories => "item_categories",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Locations => "locations",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::UserEmails => "user_emails",
            Table::Roles => "roles",
            Table::SpectraCollection => "spectra_collection",
            Table::ProjectRequirements => "project_requirements",
            Table::Documents => "documents",
            Table::Items => "items",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ContainerVerticalRules => "container_vertical_rules",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
