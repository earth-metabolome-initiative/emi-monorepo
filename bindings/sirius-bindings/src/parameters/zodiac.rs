use std::fmt::Display;

use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible zodiac settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZodiacV5 {
    /// If the zodiac is enabled
    Enabled,

    /// The version of the zodiac
    Version,

    /// The help for the zodiac
    Help,
}

impl Display for ZodiacV5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZodiacV5::Enabled => write!(f, "{}", Self::parameter_set_name()),
            ZodiacV5::Help => write!(f, "--help"),
            ZodiacV5::Version => write!(f, "--version"),
        }
    }
}

impl IntoDefault for ZodiacV5 {
    fn into_default(self) -> Self {
        match self {
            ZodiacV5::Enabled => ZodiacV5::Enabled,
            ZodiacV5::Help => ZodiacV5::Help,
            ZodiacV5::Version => ZodiacV5::Version,
        }
    }
}

impl Enablable for ZodiacV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, ZodiacV5::Enabled)
    }

    fn enabler() -> Self {
        ZodiacV5::Enabled
    }
}

impl NamedParametersSet for ZodiacV5 {
    fn parameter_set_name() -> &'static str {
        "zodiac"
    }
}
