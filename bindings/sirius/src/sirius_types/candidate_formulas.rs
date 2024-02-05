use std::fmt::Display;

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum CandidateFormulas {
    #[default]
    Comma,
}

impl Display for CandidateFormulas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateFormulas::Comma => write!(f, ","),
        }
    }
}

impl<'a> TryFrom<&'a str> for CandidateFormulas {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "," => Ok(CandidateFormulas::Comma),
            _ => Err(format!("Unknown adduct settings enforced: {}", s)),
        }
    }
}

impl TryFrom<String> for CandidateFormulas {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        CandidateFormulas::try_from(s.as_str())
    }
}
