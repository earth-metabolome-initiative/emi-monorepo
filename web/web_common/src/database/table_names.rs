//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    PrimaryUserEmails,
    SamplingProcedures,
    Taxa,
    Documents,
    SampleTaxa,
    DiscreteUnits,
    ContainerHorizontalRules,
    ItemUnits,
    ItemDiscreteQuantities,
    Notifications,
    DerivedSamples,
    ManufacturedItemCategories,
    Organizations,
    ProcedureContinuousRequirements,
    SampleStates,
    Roles,
    ContinuousUnits,
    SampledIndividualTaxa,
    Locations,
    ItemCategoryRelationships,
    Items,
    Samples,
    LoginProviders,
    Units,
    Procedures,
    SpectraCollection,
    Teams,
    Users,
    UserEmails,
    Spectra,
    ItemCategoryUnits,
    PublicUsers,
    ItemContinuousQuantities,
    SampledIndividuals,
    ProcedureDiscreteRequirements,
    Projects,
    ItemLocations,
    ProjectStates,
    ProjectRequirements,
    DocumentFormats,
    ContainerVerticalRules,
    ItemCategories,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::SamplingProcedures => "sampling_procedures",
            Table::Taxa => "taxa",
            Table::Documents => "documents",
            Table::SampleTaxa => "sample_taxa",
            Table::DiscreteUnits => "discrete_units",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ItemUnits => "item_units",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::Notifications => "notifications",
            Table::DerivedSamples => "derived_samples",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Organizations => "organizations",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::SampleStates => "sample_states",
            Table::Roles => "roles",
            Table::ContinuousUnits => "continuous_units",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::Locations => "locations",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::Items => "items",
            Table::Samples => "samples",
            Table::LoginProviders => "login_providers",
            Table::Units => "units",
            Table::Procedures => "procedures",
            Table::SpectraCollection => "spectra_collection",
            Table::Teams => "teams",
            Table::Users => "users",
            Table::UserEmails => "user_emails",
            Table::Spectra => "spectra",
            Table::ItemCategoryUnits => "item_category_units",
            Table::PublicUsers => "public_users",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::Projects => "projects",
            Table::ItemLocations => "item_locations",
            Table::ProjectStates => "project_states",
            Table::ProjectRequirements => "project_requirements",
            Table::DocumentFormats => "document_formats",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemCategories => "item_categories",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
