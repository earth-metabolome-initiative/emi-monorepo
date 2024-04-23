//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    SampleTaxa,
    ProjectStates,
    ItemDiscreteQuantities,
    UserEmails,
    LoginProviders,
    Units,
    Locations,
    ContinuousUnits,
    ProcedureContinuousRequirements,
    SpectraCollection,
    Samples,
    DiscreteUnits,
    DocumentFormats,
    PrimaryUserEmails,
    Projects,
    Roles,
    Taxa,
    Users,
    ItemLocations,
    Spectra,
    Documents,
    ItemUnits,
    ContainerVerticalRules,
    ItemCategoryUnits,
    ItemCategoryRelationships,
    Notifications,
    ProjectRequirements,
    ProcedureDiscreteRequirements,
    Teams,
    Items,
    ItemCategories,
    SampledIndividualTaxa,
    ItemContinuousQuantities,
    ManufacturedItemCategories,
    Procedures,
    PublicUser,
    SampledIndividuals,
    Organizations,
    ContainerHorizontalRules,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::SampleTaxa => "sample_taxa",
            Table::ProjectStates => "project_states",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::UserEmails => "user_emails",
            Table::LoginProviders => "login_providers",
            Table::Units => "units",
            Table::Locations => "locations",
            Table::ContinuousUnits => "continuous_units",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::SpectraCollection => "spectra_collection",
            Table::Samples => "samples",
            Table::DiscreteUnits => "discrete_units",
            Table::DocumentFormats => "document_formats",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Projects => "projects",
            Table::Roles => "roles",
            Table::Taxa => "taxa",
            Table::Users => "users",
            Table::ItemLocations => "item_locations",
            Table::Spectra => "spectra",
            Table::Documents => "documents",
            Table::ItemUnits => "item_units",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemCategoryUnits => "item_category_units",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::Notifications => "notifications",
            Table::ProjectRequirements => "project_requirements",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::Teams => "teams",
            Table::Items => "items",
            Table::ItemCategories => "item_categories",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::Procedures => "procedures",
            Table::PublicUser => "public_user",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Organizations => "organizations",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
