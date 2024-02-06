use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible formula settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormulaV5 {
    /// If the formula is enabled
    Enabled,

    /// The version for `formula`
    Version,

    /// The help for `formula`
    Help,
}

impl ToString for FormulaV5 {
    fn to_string(&self) -> String {
        match self {
            FormulaV5::Enabled => Self::parameter_set_name().to_string(),
            FormulaV5::Help => "--help".to_string(),
            FormulaV5::Version => "--version".to_string(),
        }
    }
}

impl IntoDefault for FormulaV5 {
    fn into_default(self) -> Self {
        match self {
            FormulaV5::Enabled => FormulaV5::Enabled,
            FormulaV5::Help => FormulaV5::Help,
            FormulaV5::Version => FormulaV5::Version,
        }
    }
}

impl Enablable for FormulaV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, FormulaV5::Enabled)
    }

    fn enabler() -> Self {
        FormulaV5::Enabled
    }
}

impl NamedParametersSet for FormulaV5 {
    fn parameter_set_name() -> &'static str {
        "formula"
    }
}
