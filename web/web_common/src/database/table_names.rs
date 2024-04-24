//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Procedures,
    PrimaryUserEmails,
    Documents,
    ItemUnits,
    LoginProviders,
    Locations,
    Samples,
    SamplingProcedures,
    SampleStates,
    ContainerHorizontalRules,
    DocumentFormats,
    ItemDiscreteQuantities,
    ItemLocations,
    Users,
    Projects,
    Spectra,
    Items,
    SampledIndividualTaxa,
    DerivedSamples,
    ItemCategoryUnits,
    ItemContinuousQuantities,
    ContinuousUnits,
    ProcedureDiscreteRequirements,
    Organizations,
    DiscreteUnits,
    ContainerVerticalRules,
    ItemCategories,
    Notifications,
    SpectraCollection,
    ItemCategoryRelationships,
    ProcedureContinuousRequirements,
    Units,
    UserEmails,
    PublicUsers,
    ManufacturedItemCategories,
    SampleTaxa,
    Taxa,
    Roles,
    ProjectRequirements,
    ProjectStates,
    SampledIndividuals,
    Teams,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Procedures => "procedures",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Documents => "documents",
            Table::ItemUnits => "item_units",
            Table::LoginProviders => "login_providers",
            Table::Locations => "locations",
            Table::Samples => "samples",
            Table::SamplingProcedures => "sampling_procedures",
            Table::SampleStates => "sample_states",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::DocumentFormats => "document_formats",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ItemLocations => "item_locations",
            Table::Users => "users",
            Table::Projects => "projects",
            Table::Spectra => "spectra",
            Table::Items => "items",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::DerivedSamples => "derived_samples",
            Table::ItemCategoryUnits => "item_category_units",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ContinuousUnits => "continuous_units",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::Organizations => "organizations",
            Table::DiscreteUnits => "discrete_units",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemCategories => "item_categories",
            Table::Notifications => "notifications",
            Table::SpectraCollection => "spectra_collection",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::Units => "units",
            Table::UserEmails => "user_emails",
            Table::PublicUsers => "public_users",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::SampleTaxa => "sample_taxa",
            Table::Taxa => "taxa",
            Table::Roles => "roles",
            Table::ProjectRequirements => "project_requirements",
            Table::ProjectStates => "project_states",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Teams => "teams",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
