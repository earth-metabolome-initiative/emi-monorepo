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
	diesel(sql_type = crate::diesel_impls::ToolCategory)
)]
/// Enum representing the tool categories.
pub enum ToolCategory {
    /// Device to capture images or videos
    Camera,
    /// A device used to capture insects for study or control.
    InsectTrap,
    /// A tool used to cut materials, such as scissors, scalpels or knives.
    CuttingTool,
    /// Something used to wrap or package items, such as plastic wrap, foil or
    /// paper.
    Wrapper,
    /// A tool designed for precise manipulation of small objects, such as
    /// tweezers or forceps.
    PreciseManipulationTool,
    /// Beads employed to facilitate the breaking of a material.
    BreakingBeads,
    /// Protective coverings for hands, often made of latex or nitrile.
    Gloves,
    /// A chemical agent, often ethanol, used to avoid contamination among
    /// samples.
    HandCleaningAgent,
    /// Absorbent paper used for drying hands or cleaning surfaces.
    PaperTowels,
    /// A device used to measure the volume of liquids, such as graduated
    /// cylinders or pipettes.
    VolumeMeasuringTool,
}

impl ToolCategory {
    #[must_use]
    /// Returns the name of the tool category.
    pub fn name(&self) -> &'static str {
        match self {
            ToolCategory::Camera => "Camera",
            ToolCategory::InsectTrap => "Insect Trap",
            ToolCategory::CuttingTool => "Cutting Tool",
            ToolCategory::Wrapper => "Wrapper",
            ToolCategory::PreciseManipulationTool => "Precise Manipulation Tool",
            ToolCategory::BreakingBeads => "Breaking Beads",
            ToolCategory::Gloves => "Gloves",
            ToolCategory::HandCleaningAgent => "Hand Cleaning Agent",
            ToolCategory::PaperTowels => "Paper Towels",
            ToolCategory::VolumeMeasuringTool => "Volume Measuring Tool",
        }
    }

    #[must_use]
    /// Returns the description of the tool category.
    pub fn description(&self) -> &'static str {
        match self {
            ToolCategory::Camera => "Device to capture images or videos",
            ToolCategory::InsectTrap => "A device used to capture insects for study or control.",
            ToolCategory::CuttingTool => {
                "A tool used to cut materials, such as scissors, scalpels or knives."
            }
            ToolCategory::Wrapper => {
                "Something used to wrap or package items, such as plastic wrap, foil or paper."
            }
            ToolCategory::PreciseManipulationTool => {
                "A tool designed for precise manipulation of small objects, such as tweezers or forceps."
            }
            ToolCategory::BreakingBeads => {
                "Beads employed to facilitate the breaking of a material."
            }
            ToolCategory::Gloves => {
                "Protective coverings for hands, often made of latex or nitrile."
            }
            ToolCategory::HandCleaningAgent => {
                "A chemical agent, often ethanol, used to avoid contamination among samples."
            }
            ToolCategory::PaperTowels => {
                "Absorbent paper used for drying hands or cleaning surfaces."
            }
            ToolCategory::VolumeMeasuringTool => {
                "A device used to measure the volume of liquids, such as graduated cylinders or pipettes."
            }
        }
    }

    #[must_use]
    /// Returns the icon of the tool category.
    pub fn icon(&self) -> &'static str {
        match self {
            ToolCategory::Camera => "camera",
            ToolCategory::InsectTrap => "mosquito-net",
            ToolCategory::CuttingTool => "scissors",
            ToolCategory::Wrapper => "sheet-plastic",
            ToolCategory::PreciseManipulationTool => "spoon",
            ToolCategory::BreakingBeads => "bowling-ball",
            ToolCategory::Gloves => "hand",
            ToolCategory::HandCleaningAgent => "pump-soap",
            ToolCategory::PaperTowels => "box-tissue",
            ToolCategory::VolumeMeasuringTool => "flask",
        }
    }
}
