use std::fmt::Display;

use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible write summaries settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteSummariesV5 {
    /// If the write summaries is enabled
    Enabled,

    /// The version of the write summaries
    Version,

    /// The help for the write summaries
    Help,
}

impl Display for WriteSummariesV5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteSummariesV5::Enabled => write!(f, "{}", Self::parameter_set_name()),
            WriteSummariesV5::Help => write!(f, "--help"),
            WriteSummariesV5::Version => write!(f, "--version"),
        }
    }
}

impl IntoDefault for WriteSummariesV5 {
    fn into_default(self) -> Self {
        match self {
            WriteSummariesV5::Enabled => WriteSummariesV5::Enabled,
            WriteSummariesV5::Help => WriteSummariesV5::Help,
            WriteSummariesV5::Version => WriteSummariesV5::Version,
        }
    }
}

impl Enablable for WriteSummariesV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, WriteSummariesV5::Enabled)
    }

    fn enabler() -> Self {
        WriteSummariesV5::Enabled
    }
}

impl NamedParametersSet for WriteSummariesV5 {
    fn parameter_set_name() -> &'static str {
        "write-summaries"
    }
}
