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
    /// Any tool allowing for empirically measure the length of a material,
    /// the volume of a liquid, or the mass of an object.
    EmpiricalMeasurementTool,
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
    /// A device used to measure the volume of liquids.
    GraduatedCylinder,
    /// A pipette or similar device used to measure and transfer liquids.
    Pipette,
    /// A pipette tip used to avoid contamination when transferring liquids.
    PipetteTip,
    /// A temporary container used for pipetting without risking contamination.
    PipettingContainer,
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
            ToolCategory::EmpiricalMeasurementTool => "Empirical Measurement Tool",
            ToolCategory::PreciseManipulationTool => "Precise Manipulation Tool",
            ToolCategory::BreakingBeads => "Breaking Beads",
            ToolCategory::Gloves => "Gloves",
            ToolCategory::HandCleaningAgent => "Hand Cleaning Agent",
            ToolCategory::PaperTowels => "Paper Towels",
            ToolCategory::GraduatedCylinder => "Graduated Cylinder",
            ToolCategory::Pipette => "Pipette",
            ToolCategory::PipetteTip => "Pipette Tip",
            ToolCategory::PipettingContainer => "Pipetting Container",
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
            ToolCategory::EmpiricalMeasurementTool => {
                "Any tool allowing for empirically measure the length of a material, \
                 the volume of a liquid, or the mass of an object, which can include rulers, \
                    measuring cups, scales, or even eye-sight."
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
            ToolCategory::GraduatedCylinder => "A device used to measure the volume of liquids.",
            ToolCategory::Pipette => {
                "A pipette or similar device used to measure and transfer liquids."
            }
            ToolCategory::PipetteTip => {
                "A pipette tip used to avoid contamination when transferring liquids."
            }
            ToolCategory::PipettingContainer => {
                "A temporary container used for pipetting without risking contamination."
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
            ToolCategory::EmpiricalMeasurementTool => "ruler",
            ToolCategory::PreciseManipulationTool => "spoon",
            ToolCategory::BreakingBeads => "bowling-ball",
            ToolCategory::Gloves => "hand",
            ToolCategory::HandCleaningAgent => "pump-soap",
            ToolCategory::PaperTowels => "box-tissue",
            ToolCategory::GraduatedCylinder => "flask",
            ToolCategory::PipetteTip | ToolCategory::Pipette => "syringe",
            ToolCategory::PipettingContainer => "fill-drip",
        }
    }
}
