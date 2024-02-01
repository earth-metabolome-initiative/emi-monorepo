use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum BasePeak {
    #[default]
    NotPrecursor,
}

impl Display for BasePeak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasePeak::NotPrecursor => write!(f, "NOT_PRECURSOR"),
        }
    }
}

impl<'a> TryFrom<&'a str> for BasePeak {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "NOT_PRECURSOR" => Ok(BasePeak::NotPrecursor),
            _ => Err(format!("Unknown base peak: {}", s)),
        }
    }
}

impl TryFrom<String> for BasePeak {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        BasePeak::try_from(s.as_str())
    }
}
