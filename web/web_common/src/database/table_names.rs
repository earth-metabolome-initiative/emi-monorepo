//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    Roles,
    ItemUnits,
    Samples,
    ContainerHorizontalRules,
    SpectraCollection,
    Items,
    PrimaryUserEmails,
    Organizations,
    ContinuousUnits,
    DocumentFormats,
    DiscreteUnits,
    SampledIndividuals,
    Users,
    ManufacturedItemCategories,
    ItemLocations,
    SampleTaxa,
    Taxa,
    Projects,
    UserEmails,
    Teams,
    Spectra,
    ProcedureDiscreteRequirements,
    PublicUser,
    ItemCategoryRelationships,
    ItemCategoryUnits,
    SampledIndividualTaxa,
    Documents,
    ProcedureContinuousRequirements,
    LoginProviders,
    ItemCategories,
    ProjectRequirements,
    Procedures,
    ItemContinuousQuantities,
    Units,
    Locations,
    ContainerVerticalRules,
    Notifications,
    ItemDiscreteQuantities,
    ProjectStates,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::Roles => "roles",
            Table::ItemUnits => "item_units",
            Table::Samples => "samples",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::SpectraCollection => "spectra_collection",
            Table::Items => "items",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::Organizations => "organizations",
            Table::ContinuousUnits => "continuous_units",
            Table::DocumentFormats => "document_formats",
            Table::DiscreteUnits => "discrete_units",
            Table::SampledIndividuals => "sampled_individuals",
            Table::Users => "users",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ItemLocations => "item_locations",
            Table::SampleTaxa => "sample_taxa",
            Table::Taxa => "taxa",
            Table::Projects => "projects",
            Table::UserEmails => "user_emails",
            Table::Teams => "teams",
            Table::Spectra => "spectra",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::PublicUser => "public_user",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ItemCategoryUnits => "item_category_units",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::Documents => "documents",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::LoginProviders => "login_providers",
            Table::ItemCategories => "item_categories",
            Table::ProjectRequirements => "project_requirements",
            Table::Procedures => "procedures",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Units => "units",
            Table::Locations => "locations",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::Notifications => "notifications",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::ProjectStates => "project_states",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
