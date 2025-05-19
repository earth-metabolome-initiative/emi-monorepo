#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

#[cfg(feature = "pgrx")]
use pgrx::FromDatum;

mod as_ref;
pub mod diesel_impls;
mod display;
pub mod errors;
mod try_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::ContainerCategory)
)]
/// Enum representing different instrument categories
pub enum ContainerCategory {
    /// A container which may be used for samples or mixtures.
    Bottle,
    /// A rack for sample containers, such as leaves or roots
    SampleContainerRack,
    /// A box for sample containers, or sample container racks
    ContainerBox,
}

impl ContainerCategory {
    #[must_use]
    /// Returns the name of the instrument category
    pub fn name(&self) -> &'static str {
        match self {
            ContainerCategory::Bottle => "Bottle",
            ContainerCategory::SampleContainerRack => "Sample Container Rack",
            ContainerCategory::ContainerBox => "Container Box",
        }
    }

    #[must_use]
    /// Returns the description of the instrument category
    pub fn description(&self) -> &'static str {
        match self {
            ContainerCategory::Bottle => {
                "A container appropriate for samples, such as leafs or roots."
            }
            ContainerCategory::SampleContainerRack => {
                "A rack for sample containers, such as leafs or roots."
            }
            ContainerCategory::ContainerBox => {
                "A box for sample containers, or sample container racks"
            }
        }
    }

    #[must_use]
    /// Returns the icon of the instrument category
    pub fn icon(&self) -> &'static str {
        match self {
            ContainerCategory::Bottle => "FlaskVial",
            ContainerCategory::SampleContainerRack => "Vials",
            ContainerCategory::ContainerBox => "Box",
        }
    }
}
