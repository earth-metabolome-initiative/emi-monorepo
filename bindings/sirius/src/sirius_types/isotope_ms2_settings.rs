use std::fmt::Display;

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum IsotopeMS2Settings {
    #[default]
    Ignore,
}

impl Display for IsotopeMS2Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IsotopeMS2Settings::Ignore => write!(f, "IGNORE"),
        }
    }
}

impl<'a> TryFrom<&'a str> for IsotopeMS2Settings {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "IGNORE" => Ok(IsotopeMS2Settings::Ignore),
            _ => Err(format!("Unknown adduct settings enforced: {}", s)),
        }
    }
}

impl TryFrom<String> for IsotopeMS2Settings {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        IsotopeMS2Settings::try_from(s.as_str())
    }
}
