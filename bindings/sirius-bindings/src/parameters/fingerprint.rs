use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible fingerprint settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FingerprintV5 {
    /// If the fingerprint is enabled
    Enabled,

    /// The version for `fingerprint`
    Version,

    /// The help for `fingerprint`
    Help,
}

impl ToString for FingerprintV5 {
    fn to_string(&self) -> String {
        match self {
            FingerprintV5::Enabled => Self::parameter_set_name().to_string(),
            FingerprintV5::Help => "--help".to_string(),
            FingerprintV5::Version => "--version".to_string(),
        }
    }
}

impl IntoDefault for FingerprintV5 {
    fn into_default(self) -> Self {
        match self {
            FingerprintV5::Enabled => FingerprintV5::Enabled,
            FingerprintV5::Help => FingerprintV5::Help,
            FingerprintV5::Version => FingerprintV5::Version,
        }
    }
}

impl Enablable for FingerprintV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, FingerprintV5::Enabled)
    }

    fn enabler() -> Self {
        FingerprintV5::Enabled
    }
}

impl NamedParametersSet for FingerprintV5 {
    fn parameter_set_name() -> &'static str {
        "fingerprint"
    }
}
