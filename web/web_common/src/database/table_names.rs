//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Projects,
    ContainerVerticalRules,
    UserEmails,
    Users,
    Locations,
    Taxa,
    Items,
    Spectra,
    ItemLocations,
    Units,
    ItemContinuousQuantities,
    SampleStates,
    SpectraCollection,
    Notifications,
    PrimaryUserEmails,
    Documents,
    Samples,
    SamplingProcedures,
    DocumentFormats,
    LoginProviders,
    SampledIndividualTaxa,
    Teams,
    SampleTaxa,
    PublicUsers,
    ProjectRequirements,
    ItemCategoryUnits,
    Procedures,
    DerivedSamples,
    DiscreteUnits,
    Roles,
    ItemUnits,
    ItemDiscreteQuantities,
    SampledIndividuals,
    Organizations,
    ContinuousUnits,
    ItemCategoryRelationships,
    ProcedureContinuousRequirements,
    ProjectStates,
    ManufacturedItemCategories,
    ProcedureDiscreteRequirements,
    ItemCategories,
    ContainerHorizontalRules,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Projects => "projects",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::UserEmails => "user_emails",
            Table::Users => "users",
            Table::Locations => "locations",
            Table::Taxa => "taxa",
            Table::Items => "items",
            Table::Spectra => "spectra",
            Table::ItemLocations => "item_locations",
            Table::Units => "units",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::SampleStates => "sample_states",
            Table::SpectraCollection => "spectra_collection",
            Table::Notifications => "notifications",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Documents => "documents",
            Table::Samples => "samples",
            Table::SamplingProcedures => "sampling_procedures",
            Table::DocumentFormats => "document_formats",
            Table::LoginProviders => "login_providers",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::Teams => "teams",
            Table::SampleTaxa => "sample_taxa",
            Table::PublicUsers => "public_users",
            Table::ProjectRequirements => "project_requirements",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Procedures => "procedures",
            Table::DerivedSamples => "derived_samples",
            Table::DiscreteUnits => "discrete_units",
            Table::Roles => "roles",
            Table::ItemUnits => "item_units",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Organizations => "organizations",
            Table::ContinuousUnits => "continuous_units",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ProjectStates => "project_states",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ItemCategories => "item_categories",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
