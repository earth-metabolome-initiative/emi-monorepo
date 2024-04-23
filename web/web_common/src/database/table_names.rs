//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    ManufacturedItemCategories,
    ContainerHorizontalRules,
    SampleStates,
    ItemCategoryRelationships,
    Procedures,
    ItemDiscreteQuantities,
    UserEmails,
    Notifications,
    SampledIndividualTaxa,
    Taxa,
    SampledIndividuals,
    ProjectRequirements,
    ProjectStates,
    Items,
    Locations,
    Projects,
    Units,
    Users,
    ItemCategoryUnits,
    Teams,
    ItemLocations,
    ItemUnits,
    SpectraCollection,
    Roles,
    ItemContinuousQuantities,
    ContinuousUnits,
    ItemCategories,
    LoginProviders,
    ProcedureContinuousRequirements,
    ContainerVerticalRules,
    Samples,
    DiscreteUnits,
    Documents,
    Spectra,
    Organizations,
    DocumentFormats,
    SampleTaxa,
    ProcedureDiscreteRequirements,
    PublicUser,
    PrimaryUserEmails,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::SampleStates => "sample_states",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::Procedures => "procedures",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::UserEmails => "user_emails",
            Table::Notifications => "notifications",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::Taxa => "taxa",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ProjectRequirements => "project_requirements",
            Table::ProjectStates => "project_states",
            Table::Items => "items",
            Table::Locations => "locations",
            Table::Projects => "projects",
            Table::Units => "units",
            Table::Users => "users",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Teams => "teams",
            Table::ItemLocations => "item_locations",
            Table::ItemUnits => "item_units",
            Table::SpectraCollection => "spectra_collection",
            Table::Roles => "roles",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ContinuousUnits => "continuous_units",
            Table::ItemCategories => "item_categories",
            Table::LoginProviders => "login_providers",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::Samples => "samples",
            Table::DiscreteUnits => "discrete_units",
            Table::Documents => "documents",
            Table::Spectra => "spectra",
            Table::Organizations => "organizations",
            Table::DocumentFormats => "document_formats",
            Table::SampleTaxa => "sample_taxa",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::PublicUser => "public_user",
            Table::PrimaryUserEmails => "primary_user_emails",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
