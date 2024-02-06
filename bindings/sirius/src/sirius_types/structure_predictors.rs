use std::fmt::Display;

/// The structure predictors. For now, only CSI_FINGERID is supported.
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub enum StructurePredictors {
    #[default]
    /// The CSI_FINGERID structure predictor
    CsiFingerId,
}

impl Display for StructurePredictors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructurePredictors::CsiFingerId => write!(f, "CSI_FINGERID"),
        }
    }
}

impl<'a> TryFrom<&'a str> for StructurePredictors {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "CSI_FINGERID" => Ok(StructurePredictors::CsiFingerId),
            _ => Err(format!("Unknown structure predictor: {}", s)),
        }
    }
}

impl TryFrom<String> for StructurePredictors {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        StructurePredictors::try_from(s.as_str())
    }
}
