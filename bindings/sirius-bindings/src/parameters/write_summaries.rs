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

impl ToString for WriteSummariesV5 {
    fn to_string(&self) -> String {
        match self {
            WriteSummariesV5::Enabled => Self::parameter_set_name().to_string(),
            WriteSummariesV5::Help => "--help".to_string(),
            WriteSummariesV5::Version => "--version".to_string(),
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
