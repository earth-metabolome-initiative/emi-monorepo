//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use serde::Deserialize;
use serde::Serialize;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Table {
    PublicUser,
    ItemCategoryRelationships,
    ContinuousUnits,
    ProcedureDiscreteRequirements,
    Teams,
    Spectra,
    Procedures,
    Users,
    LoginProviders,
    DocumentFormats,
    Taxa,
    ItemUnits,
    ProjectRequirements,
    ContainerHorizontalRules,
    ItemLocations,
    ProcedureContinuousRequirements,
    DiscreteUnits,
    SampledIndividualTaxa,
    SampledIndividuals,
    UserEmails,
    ItemContinuousQuantities,
    Notifications,
    SampleTaxa,
    Items,
    Organizations,
    Samples,
    ItemCategories,
    Units,
    ManufacturedItemCategories,
    ItemDiscreteQuantities,
    Projects,
    Documents,
    SpectraCollection,
    Locations,
    Roles,
    PrimaryUserEmails,
    ContainerVerticalRules,
    ItemCategoryUnits,
    ProjectStates,
}

impl AsRef<str> for Table {
    fn as_ref(&self) -> &str {
        match self {
            Table::PublicUser => "public_user",
            Table::ItemCategoryRelationships => "item_category_relationships",
            Table::ContinuousUnits => "continuous_units",
            Table::ProcedureDiscreteRequirements => "procedure_discrete_requirements",
            Table::Teams => "teams",
            Table::Spectra => "spectra",
            Table::Procedures => "procedures",
            Table::Users => "users",
            Table::LoginProviders => "login_providers",
            Table::DocumentFormats => "document_formats",
            Table::Taxa => "taxa",
            Table::ItemUnits => "item_units",
            Table::ProjectRequirements => "project_requirements",
            Table::ContainerHorizontalRules => "container_horizontal_rules",
            Table::ItemLocations => "item_locations",
            Table::ProcedureContinuousRequirements => "procedure_continuous_requirements",
            Table::DiscreteUnits => "discrete_units",
            Table::SampledIndividualTaxa => "sampled_individual_taxa",
            Table::SampledIndividuals => "sampled_individuals",
            Table::UserEmails => "user_emails",
            Table::ItemContinuousQuantities => "item_continuous_quantities",
            Table::Notifications => "notifications",
            Table::SampleTaxa => "sample_taxa",
            Table::Items => "items",
            Table::Organizations => "organizations",
            Table::Samples => "samples",
            Table::ItemCategories => "item_categories",
            Table::Units => "units",
            Table::ManufacturedItemCategories => "manufactured_item_categories",
            Table::ItemDiscreteQuantities => "item_discrete_quantities",
            Table::Projects => "projects",
            Table::Documents => "documents",
            Table::SpectraCollection => "spectra_collection",
            Table::Locations => "locations",
            Table::Roles => "roles",
            Table::PrimaryUserEmails => "primary_user_emails",
            Table::ContainerVerticalRules => "container_vertical_rules",
            Table::ItemCategoryUnits => "item_category_units",
            Table::ProjectStates => "project_states",
        }
    }
}
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
