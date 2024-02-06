use std::fmt::Display;

/// The possible instruments for the mass spectrometry
#[cfg_attr(feature = "fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Instruments {
    /// The default instrument
    #[default]
    Default,

    /// The qtof instrument
    Qtof,

    /// The orbitrap instrument
    Orbitrap,

    /// The fticr instrument
    Fticr,
}

impl Display for Instruments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruments::Default => write!(f, "default"),
            Instruments::Qtof => write!(f, "qtof"),
            Instruments::Orbitrap => write!(f, "orbitrap"),
            Instruments::Fticr => write!(f, "fticr"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Instruments {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "default" => Ok(Instruments::Default),
            "qtof" => Ok(Instruments::Qtof),
            "orbitrap" => Ok(Instruments::Orbitrap),
            "fticr" => Ok(Instruments::Fticr),
            _ => Err(format!("Unknown instrument: {}", s)),
        }
    }
}

impl TryFrom<String> for Instruments {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Instruments::try_from(s.as_str())
    }
}
