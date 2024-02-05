use std::fmt::Display;

#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompoundQuality {
    #[default]
    Unknown,
    Good,
    LowIntensity,
    NoMS1Peak,
    FewPeaks,
    Chimeric,
    NotMonoisotopicPeak,
    PoorlyExplained,
}

impl Display for CompoundQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundQuality::Unknown => write!(f, "UNKNOWN"),
            CompoundQuality::Good => write!(f, "Good"),
            CompoundQuality::LowIntensity => write!(f, "LowIntensity"),
            CompoundQuality::NoMS1Peak => write!(f, "NoMS1Peak"),
            CompoundQuality::FewPeaks => write!(f, "FewPeaks"),
            CompoundQuality::Chimeric => write!(f, "Chimeric"),
            CompoundQuality::NotMonoisotopicPeak => write!(f, "NotMonoisotopicPeak"),
            CompoundQuality::PoorlyExplained => write!(f, "PoorlyExplained"),
        }
    }
}

impl<'a> TryFrom<&'a str> for CompoundQuality {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "UNKNOWN" => Ok(CompoundQuality::Unknown),
            "Good" => Ok(CompoundQuality::Good),
            "LowIntensity" => Ok(CompoundQuality::LowIntensity),
            "NoMS1Peak" => Ok(CompoundQuality::NoMS1Peak),
            "FewPeaks" => Ok(CompoundQuality::FewPeaks),
            "Chimeric" => Ok(CompoundQuality::Chimeric),
            "NotMonoisotopicPeak" => Ok(CompoundQuality::NotMonoisotopicPeak),
            "PoorlyExplained" => Ok(CompoundQuality::PoorlyExplained),
            _ => Err(format!("Unknown compound quality: {}", s)),
        }
    }
}

impl TryFrom<String> for CompoundQuality {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        CompoundQuality::try_from(s.as_str())
    }
}
