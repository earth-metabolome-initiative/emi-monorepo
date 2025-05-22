#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

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
	diesel(sql_type = crate::diesel_impls::InstrumentCategory)
)]
/// Enum representing different instrument categories
pub enum InstrumentCategory {
    /// Mass Spectrometer
    MassSpectrometer,
    /// Weight Scale
    WeightScale,
    /// Freeze Dryer
    FreezeDryer,
    /// Ball Mill
    BallMill,
    /// Centrifuge
    Centrifuge,
    /// Dispenser
    Dispenser,
    /// Shaker
    Shaker,
}

impl InstrumentCategory {
    #[must_use]
    /// Returns the name of the instrument category
    pub fn name(&self) -> &'static str {
        match self {
            InstrumentCategory::MassSpectrometer => "Mass Spectrometer",
            InstrumentCategory::WeightScale => "Weight Scale",
            InstrumentCategory::FreezeDryer => "Freeze Dryer",
            InstrumentCategory::BallMill => "Ball Mill",
            InstrumentCategory::Centrifuge => "Centrifuge",
            InstrumentCategory::Dispenser => "Dispenser",
            InstrumentCategory::Shaker => "Shaker",
        }
    }

    #[must_use]
    /// Returns the description of the instrument category
    pub fn description(&self) -> &'static str {
        match self {
            InstrumentCategory::MassSpectrometer => {
                "An analytical technique used to measure the mass-to-charge ratio of ions."
            }
            InstrumentCategory::WeightScale => "A device used to measure the weight of an object.",
            InstrumentCategory::FreezeDryer => {
                "A device used to remove moisture from a sample by freezing it and then reducing the surrounding pressure."
            }
            InstrumentCategory::BallMill => "A device used to grind materials with beads.",
            InstrumentCategory::Centrifuge => {
                "A device that uses centrifugal force to separate components of a mixture based on density."
            }
            InstrumentCategory::Dispenser => {
                "A device used to dispense a specific amount of liquid or powder."
            }
            InstrumentCategory::Shaker => "A device used to mix or agitate a sample.",
        }
    }

    #[must_use]
    /// Returns the icon of the instrument category
    pub fn icon(&self) -> &'static str {
        match self {
            InstrumentCategory::MassSpectrometer | InstrumentCategory::Centrifuge => "microscope",
            InstrumentCategory::WeightScale => "weight-scale",
            InstrumentCategory::FreezeDryer => "snowflake",
            InstrumentCategory::BallMill => "mortar-pestle",
            InstrumentCategory::Dispenser => "eye-dropper",
            InstrumentCategory::Shaker => "vial",
        }
    }
}
