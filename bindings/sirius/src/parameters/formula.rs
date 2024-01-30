use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormulaV5 {
    Enabled,
    Version,
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
        match self {
            FormulaV5::Enabled => true,
            _ => false,
        }
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
