//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Taxa,
    ContinuousUnits,
    Organizations,
    ContainerHorizontalRules,
    Locations,
    ItemContinuousQuantities,
    DocumentFormats,
    SpectraCollection,
    Procedures,
    ItemDiscreteQuantities,
    DerivedSamples,
    Documents,
    PrimaryUserEmails,
    Samples,
    SamplingProcedures,
    Projects,
    Roles,
    ItemCategories,
    ItemCategoryRelationships,
    UserEmails,
    PublicUsers,
    ProcedureDiscreteRequirements,
    SampledIndividuals,
    ProcedureContinuousRequirements,
    Teams,
    LoginProviders,
    SampleTaxa,
    Units,
    ItemUnits,
    ManufacturedItemCategories,
    Spectra,
    DiscreteUnits,
    ContainerVerticalRules,
    ProjectStates,
    ProjectRequirements,
    SampleStates,
    ItemCategoryUnits,
    SampledIndividualTaxa,
    ItemLocations,
    Notifications,
    Items,
    Users,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Taxa => "taxa",
            Table::ContinuousUnits => "continuous_units",
            Table::Organizations => "organizations",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::Locations => "locations",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::DocumentFormats => "document_formats",
            Table::SpectraCollection => "spectra_collection",
            Table::Procedures => "procedures",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::DerivedSamples => "derived_samples",
            Table::Documents => "documents",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Samples => "samples",
            Table::SamplingProcedures => "sampling_procedures",
            Table::Projects => "projects",
            Table::Roles => "roles",
            Table::ItemCategories => "item_categories",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::UserEmails => "user_emails",
            Table::PublicUsers => "public_users",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::Teams => "teams",
            Table::LoginProviders => "login_providers",
            Table::SampleTaxa => "sample_taxa",
            Table::Units => "units",
            Table::ItemUnits => "item_units",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Spectra => "spectra",
            Table::DiscreteUnits => "discrete_units",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ProjectStates => "project_states",
            Table::ProjectRequirements => "project_requirements",
            Table::SampleStates => "sample_states",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ItemLocations => "item_locations",
            Table::Notifications => "notifications",
            Table::Items => "items",
            Table::Users => "users",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
