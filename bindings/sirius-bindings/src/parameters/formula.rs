use std::fmt::Display;

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

impl Display for FormulaV5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaV5::Enabled => write!(f, "{}", Self::parameter_set_name()),
            FormulaV5::Help => write!(f, "--help"),
            FormulaV5::Version => write!(f, "--version"),
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
