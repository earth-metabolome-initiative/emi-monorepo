use std::fmt::Display;

/// The possible adduct settings enforced
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum AdductSettingsEnforced {
    /// The default adduct settings enforced
    #[default]
    Comma,
}

impl Display for AdductSettingsEnforced {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdductSettingsEnforced::Comma => write!(f, ","),
        }
    }
}

impl<'a> TryFrom<&'a str> for AdductSettingsEnforced {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "," => Ok(AdductSettingsEnforced::Comma),
            _ => Err(format!("Unknown adduct settings enforced: {}", s)),
        }
    }
}

impl TryFrom<String> for AdductSettingsEnforced {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        AdductSettingsEnforced::try_from(s.as_str())
    }
}
