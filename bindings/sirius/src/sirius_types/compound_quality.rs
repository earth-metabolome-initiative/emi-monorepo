use std::fmt::Display;

/// The possible compound qualities if known
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompoundQuality {
    /// The default compound quality: unknown
    #[default]
    Unknown,

    /// Is good
    Good,

    /// Is low intensity
    LowIntensity,

    /// Has no MS1 peak
    NoMS1Peak,

    /// Has few peaks
    FewPeaks,

    /// Is chimeric
    Chimeric,

    /// Is not a monoisotopic peak
    NotMonoisotopicPeak,

    /// Is poorly explained
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
