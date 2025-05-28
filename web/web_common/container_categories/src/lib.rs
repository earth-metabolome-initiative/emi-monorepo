#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "diesel_pgrx", derive(diesel_pgrx::DieselPGRX))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresType))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[
    cfg_attr(feature = "diesel", diesel(sql_type = crate::diesel_impls::ContainerCategory))]
#[cfg_attr(feature = "pgrx", pg_binary_protocol)]
/// Enum representing different instrument categories
pub enum ContainerCategory {
    /// A container which may be used for samples or mixtures.
    Bottle {
        /// The volume of the bottle in liters
        liters: f64,
    },
    /// A sample container, such as falcon tube that contains solid sample
    SampleContainer,
    /// A rack for sample containers, that can contain multiple sample
    /// containers. Open.
    SampleContainerRack,
    /// A box for sample containers, or sample container racks. Closed.
    ContainerBox,
}

impl ContainerCategory {
    #[must_use]
    /// Returns the name of the instrument category
    pub fn name(&self) -> &'static str {
        match self {
            ContainerCategory::Bottle { .. } => "Bottle",
            ContainerCategory::SampleContainer => "Sample Container",
            ContainerCategory::SampleContainerRack => "Sample Container Rack",
            ContainerCategory::ContainerBox => "Container Box",
        }
    }

    #[must_use]
    /// Returns the description of the instrument category
    pub fn description(&self) -> &'static str {
        match self {
            ContainerCategory::SampleContainer => {
                "A sample container, such as falcon tube that contains solid sample."
            }
            ContainerCategory::Bottle { .. } => {
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
            ContainerCategory::SampleContainer => "vial",
            ContainerCategory::Bottle { .. } => "flask-vial",
            ContainerCategory::SampleContainerRack => "vials",
            ContainerCategory::ContainerBox => "box",
        }
    }
}
