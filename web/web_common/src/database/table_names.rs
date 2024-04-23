//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    DocumentFormats,
    ItemLocations,
    Procedures,
    PrimaryUserEmails,
    ProcedureDiscreteRequirements,
    ProjectStates,
    Spectra,
    ContainerVerticalRules,
    ItemDiscreteQuantities,
    SamplingProcedures,
    Teams,
    ItemCategoryUnits,
    SampleStates,
    Taxa,
    Notifications,
    ItemUnits,
    PublicUser,
    Documents,
    Roles,
    Units,
    SampleTaxa,
    Users,
    Samples,
    Organizations,
    Locations,
    SampledIndividuals,
    Items,
    LoginProviders,
    DiscreteUnits,
    SampledIndividualTaxa,
    ContainerHorizontalRules,
    ManufacturedItemCategories,
    ItemContinuousQuantities,
    ItemCategoryRelationships,
    ProjectRequirements,
    Projects,
    ItemCategories,
    ContinuousUnits,
    ProcedureContinuousRequirements,
    DerivedSamples,
    SpectraCollection,
    UserEmails,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::DocumentFormats => "document_formats",
            Table::ItemLocations => "item_locations",
            Table::Procedures => "procedures",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ProjectStates => "project_states",
            Table::Spectra => "spectra",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::SamplingProcedures => "sampling_procedures",
            Table::Teams => "teams",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SampleStates => "sample_states",
            Table::Taxa => "taxa",
            Table::Notifications => "notifications",
            Table::ItemUnits => "item_units",
            Table::PublicUser => "public_user",
            Table::Documents => "documents",
            Table::Roles => "roles",
            Table::Units => "units",
            Table::SampleTaxa => "sample_taxa",
            Table::Users => "users",
            Table::Samples => "samples",
            Table::Organizations => "organizations",
            Table::Locations => "locations",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Items => "items",
            Table::LoginProviders => "login_providers",
            Table::DiscreteUnits => "discrete_units",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ProjectRequirements => "project_requirements",
            Table::Projects => "projects",
            Table::ItemCategories => "item_categories",
            Table::ContinuousUnits => "continuous_units",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::DerivedSamples => "derived_samples",
            Table::SpectraCollection => "spectra_collection",
            Table::UserEmails => "user_emails",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
