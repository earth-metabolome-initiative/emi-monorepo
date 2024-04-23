//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    SampleTaxa,
    ProjectRequirements,
    PublicUser,
    Users,
    Documents,
    ProcedureDiscreteRequirements,
    ItemCategoryUnits,
    Items,
    SampledIndividuals,
    ItemDiscreteQuantities,
    ContainerVerticalRules,
    Locations,
    ProjectStates,
    Projects,
    UserEmails,
    SpectraCollection,
    ContinuousUnits,
    Samples,
    LoginProviders,
    Spectra,
    DiscreteUnits,
    ItemCategoryRelationships,
    ItemLocations,
    ManufacturedItemCategories,
    ItemUnits,
    SampleStates,
    Units,
    ContainerHorizontalRules,
    DocumentFormats,
    PrimaryUserEmails,
    Teams,
    Procedures,
    SampledIndividualTaxa,
    ItemCategories,
    Taxa,
    Organizations,
    Notifications,
    ItemContinuousQuantities,
    Roles,
    ProcedureContinuousRequirements,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::SampleTaxa => "sample_taxa",
            Table::ProjectRequirements => "project_requirements",
            Table::PublicUser => "public_user",
            Table::Users => "users",
            Table::Documents => "documents",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Items => "items",
            Table::SampledIndividuals => "sampled_individuals",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::Locations => "locations",
            Table::ProjectStates => "project_states",
            Table::Projects => "projects",
            Table::UserEmails => "user_emails",
            Table::SpectraCollection => "spectra_collection",
            Table::ContinuousUnits => "continuous_units",
            Table::Samples => "samples",
            Table::LoginProviders => "login_providers",
            Table::Spectra => "spectra",
            Table::DiscreteUnits => "discrete_units",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemLocations => "item_locations",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ItemUnits => "item_units",
            Table::SampleStates => "sample_states",
            Table::Units => "units",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::DocumentFormats => "document_formats",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Teams => "teams",
            Table::Procedures => "procedures",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ItemCategories => "item_categories",
            Table::Taxa => "taxa",
            Table::Organizations => "organizations",
            Table::Notifications => "notifications",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Roles => "roles",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
