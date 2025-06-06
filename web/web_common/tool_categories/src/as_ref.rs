//! Submodule providing implementations of the `AsRef` trait for the
//! [`ToolCategory`] enumeration.

impl AsRef<str> for crate::ToolCategory {
    fn as_ref(&self) -> &str {
        match self {
            Self::Camera => "Camera",
            Self::InsectTrap => "InsectTrap",
            Self::CuttingTool => "CuttingTool",
            Self::Wrapper => "Wrapper",
            Self::EmpiricalMeasurementTool => "EmpiricalMeasurementTool",
            Self::PreciseManipulationTool => "PreciseManipulationTool",
            Self::BreakingBeads => "BreakingBeads",
            Self::Gloves => "Gloves",
            Self::LiquidDispenser => "LiquidDispenser",
            Self::PaperTowels => "PaperTowels",
            Self::GraduatedCylinder => "GraduatedCylinder",
            Self::Pipette => "Pipette",
            Self::PipetteTip => "PipetteTip",
            Self::PipettingContainer => "PipettingContainer",
        }
    }
}
