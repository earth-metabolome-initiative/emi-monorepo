use std::fmt::Display;

/// Allow or forbid recalibration
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum ForbidRecalibration {
    /// The default recalibration setting : ALLOWED
    #[default]
    Allowed,

    /// Forbid recalibration
    Forbidden,
}

impl Display for ForbidRecalibration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForbidRecalibration::Allowed => write!(f, "ALLOWED"),
            ForbidRecalibration::Forbidden => write!(f, "FORBIDDEN"),
        }
    }
}

impl<'a> TryFrom<&'a str> for ForbidRecalibration {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "ALLOWED" => Ok(ForbidRecalibration::Allowed),
            "FORBIDDEN" => Ok(ForbidRecalibration::Forbidden),
            _ => Err(format!("Unknown value: {}", s)),
        }
    }
}

impl TryFrom<String> for ForbidRecalibration {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        ForbidRecalibration::try_from(s.as_str())
    }
}
