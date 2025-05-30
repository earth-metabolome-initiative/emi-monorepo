//! Submodule providing implementations of the `TryFrom` trait for the
//! [`ToolCategory`] enumeration.

impl TryFrom<&str> for crate::ToolCategory {
    type Error = crate::errors::UnknownToolCategory;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "Camera" => Self::Camera,
            "InsectTrap" => Self::InsectTrap,
            "CuttingTool" => Self::CuttingTool,
            "Wrapper" => Self::Wrapper,
            "EmpiricalMeasurementTool" => Self::EmpiricalMeasurementTool,
            "PreciseManipulationTool" => Self::PreciseManipulationTool,
            "BreakingBeads" => Self::BreakingBeads,
            "Gloves" => Self::Gloves,
            "HandCleaningAgent" => Self::HandCleaningAgent,
            "PaperTowels" => Self::PaperTowels,
            "GraduatedCylinder" => Self::GraduatedCylinder,
            "Pipette" => Self::Pipette,
            "PipetteTip" => Self::PipetteTip,
            "PipettingContainer" => Self::PipettingContainer,
            _ => return Err(crate::errors::UnknownToolCategory::UnknownString(value.to_string())),
        })
    }
}

impl TryFrom<String> for crate::ToolCategory {
    type Error = crate::errors::UnknownToolCategory;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for crate::ToolCategory {
    type Error = crate::errors::UnknownToolCategory;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
