//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    ItemCategoryRelationships,
    DiscreteUnits,
    SamplingProcedures,
    Locations,
    ProjectStates,
    SampleTaxa,
    Teams,
    ItemLocations,
    UserEmails,
    Documents,
    SpectraCollection,
    Samples,
    ProcedureContinuousRequirements,
    SampledIndividuals,
    Taxa,
    Spectra,
    ManufacturedItemCategories,
    Procedures,
    ProjectRequirements,
    ContinuousUnits,
    ItemCategoryUnits,
    SampleStates,
    Users,
    DerivedSamples,
    Roles,
    ItemContinuousQuantities,
    Organizations,
    Items,
    Projects,
    DocumentFormats,
    LoginProviders,
    ItemCategories,
    Notifications,
    SampledIndividualTaxa,
    ContainerHorizontalRules,
    ItemDiscreteQuantities,
    PrimaryUserEmails,
    PublicUsers,
    ContainerVerticalRules,
    ItemUnits,
    Units,
    ProcedureDiscreteRequirements,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::DiscreteUnits => "discrete_units",
            Table::SamplingProcedures => "sampling_procedures",
            Table::Locations => "locations",
            Table::ProjectStates => "project_states",
            Table::SampleTaxa => "sample_taxa",
            Table::Teams => "teams",
            Table::ItemLocations => "item_locations",
            Table::UserEmails => "user_emails",
            Table::Documents => "documents",
            Table::SpectraCollection => "spectra_collection",
            Table::Samples => "samples",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Taxa => "taxa",
            Table::Spectra => "spectra",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Procedures => "procedures",
            Table::ProjectRequirements => "project_requirements",
            Table::ContinuousUnits => "continuous_units",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SampleStates => "sample_states",
            Table::Users => "users",
            Table::DerivedSamples => "derived_samples",
            Table::Roles => "roles",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Organizations => "organizations",
            Table::Items => "items",
            Table::Projects => "projects",
            Table::DocumentFormats => "document_formats",
            Table::LoginProviders => "login_providers",
            Table::ItemCategories => "item_categories",
            Table::Notifications => "notifications",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::PublicUsers => "public_users",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemUnits => "item_units",
            Table::Units => "units",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
