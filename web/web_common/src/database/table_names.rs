//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Organizations,
    UserEmails,
    ItemCategories,
    ContinuousUnits,
    DocumentFormats,
    ItemCategoryUnits,
    Teams,
    PublicUser,
    SampleTaxa,
    Samples,
    SampledIndividualTaxa,
    ItemCategoryRelationships,
    SampledIndividuals,
    ProcedureContinuousRequirements,
    Roles,
    Units,
    ContainerHorizontalRules,
    Items,
    Users,
    Spectra,
    ItemContinuousQuantities,
    LoginProviders,
    Procedures,
    ManufacturedItemCategories,
    Documents,
    SpectraCollection,
    Taxa,
    Locations,
    ProjectRequirements,
    ContainerVerticalRules,
    ProjectStates,
    ItemUnits,
    ItemDiscreteQuantities,
    Projects,
    ItemLocations,
    SampleStates,
    Notifications,
    DiscreteUnits,
    PrimaryUserEmails,
    ProcedureDiscreteRequirements,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Organizations => "organizations",
            Table::UserEmails => "user_emails",
            Table::ItemCategories => "item_categories",
            Table::ContinuousUnits => "continuous_units",
            Table::DocumentFormats => "document_formats",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Teams => "teams",
            Table::PublicUser => "public_user",
            Table::SampleTaxa => "sample_taxa",
            Table::Samples => "samples",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::Roles => "roles",
            Table::Units => "units",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::Items => "items",
            Table::Users => "users",
            Table::Spectra => "spectra",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::LoginProviders => "login_providers",
            Table::Procedures => "procedures",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Documents => "documents",
            Table::SpectraCollection => "spectra_collection",
            Table::Taxa => "taxa",
            Table::Locations => "locations",
            Table::ProjectRequirements => "project_requirements",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ProjectStates => "project_states",
            Table::ItemUnits => "item_units",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::Projects => "projects",
            Table::ItemLocations => "item_locations",
            Table::SampleStates => "sample_states",
            Table::Notifications => "notifications",
            Table::DiscreteUnits => "discrete_units",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
