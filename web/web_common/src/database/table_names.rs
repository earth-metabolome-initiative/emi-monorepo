//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Units,
    ItemCategories,
    ProjectRequirements,
    PublicUser,
    ItemContinuousQuantities,
    Samples,
    ItemLocations,
    ContainerHorizontalRules,
    Organizations,
    Roles,
    SpectraCollection,
    ProjectStates,
    UserEmails,
    PrimaryUserEmails,
    Users,
    Notifications,
    Items,
    Taxa,
    ItemCategoryRelationships,
    ItemCategoryUnits,
    Procedures,
    ItemUnits,
    SampledIndividuals,
    DiscreteUnits,
    Projects,
    ManufacturedItemCategories,
    LoginProviders,
    Teams,
    ContinuousUnits,
    Documents,
    ProcedureContinuousRequirements,
    ContainerVerticalRules,
    ItemDiscreteQuantities,
    SampleTaxa,
    Spectra,
    DocumentFormats,
    Locations,
    ProcedureDiscreteRequirements,
    SampledIndividualTaxa,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Units => "units",
            Table::ItemCategories => "item_categories",
            Table::ProjectRequirements => "project_requirements",
            Table::PublicUser => "public_user",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Samples => "samples",
            Table::ItemLocations => "item_locations",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::Organizations => "organizations",
            Table::Roles => "roles",
            Table::SpectraCollection => "spectra_collection",
            Table::ProjectStates => "project_states",
            Table::UserEmails => "user_emails",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Users => "users",
            Table::Notifications => "notifications",
            Table::Items => "items",
            Table::Taxa => "taxa",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemCategoryUnits => "item_category_units",
            Table::Procedures => "procedures",
            Table::ItemUnits => "item_units",
            Table::SampledIndividuals => "sampled_individuals",
            Table::DiscreteUnits => "discrete_units",
            Table::Projects => "projects",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::LoginProviders => "login_providers",
            Table::Teams => "teams",
            Table::ContinuousUnits => "continuous_units",
            Table::Documents => "documents",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::SampleTaxa => "sample_taxa",
            Table::Spectra => "spectra",
            Table::DocumentFormats => "document_formats",
            Table::Locations => "locations",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
