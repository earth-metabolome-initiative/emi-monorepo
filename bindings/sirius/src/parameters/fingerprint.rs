use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FingerprintV5 {
    Enabled,
    Version,
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
        match self {
            FingerprintV5::Enabled => true,
            _ => false,
        }
    }

    fn enabler() -> Self {
        FingerprintV5::Enabled
    }
}

impl NamedParametersSet for FingerprintV5 {
    fn parameter_set_name() -> &'static str {
        "Fingerprint"
    }
}
