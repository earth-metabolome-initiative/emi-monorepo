use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZodiacV5 {
    Enabled,
    Version,
    Help,
}

impl ToString for ZodiacV5 {
    fn to_string(&self) -> String {
        match self {
            ZodiacV5::Enabled => Self::parameter_set_name().to_string(),
            ZodiacV5::Help => "--help".to_string(),
            ZodiacV5::Version => "--version".to_string(),
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
