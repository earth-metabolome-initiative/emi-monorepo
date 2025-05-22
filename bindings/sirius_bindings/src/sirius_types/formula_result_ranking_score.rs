use std::fmt::Display;

/// The possible formula result ranking score
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum FormulaResultRankingScore {
    /// The default formula result ranking score
    #[default]
    Auto,
}

impl Display for FormulaResultRankingScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaResultRankingScore::Auto => write!(f, "AUTO"),
        }
    }
}

impl<'a> TryFrom<&'a str> for FormulaResultRankingScore {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "AUTO" => Ok(FormulaResultRankingScore::Auto),
            _ => Err(format!("Unknown adduct settings enforced: {s}")),
        }
    }
}

impl TryFrom<String> for FormulaResultRankingScore {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        FormulaResultRankingScore::try_from(s.as_str())
    }
}
