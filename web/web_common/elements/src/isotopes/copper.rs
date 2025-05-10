//! Isotopes of the element Copper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Copper
pub enum CopperIsotope {
    /// Isotope Cu52 of Copper
    Cu52,
    /// Isotope Cu53 of Copper
    Cu53,
    /// Isotope Cu54 of Copper
    Cu54,
    /// Isotope Cu55 of Copper
    Cu55,
    /// Isotope Cu56 of Copper
    Cu56,
    /// Isotope Cu57 of Copper
    Cu57,
    /// Isotope Cu58 of Copper
    Cu58,
    /// Isotope Cu59 of Copper
    Cu59,
    /// Isotope Cu60 of Copper
    Cu60,
    /// Isotope Cu61 of Copper
    Cu61,
    /// Isotope Cu62 of Copper
    Cu62,
    /// Isotope Cu63 of Copper
    Cu63,
    /// Isotope Cu64 of Copper
    Cu64,
    /// Isotope Cu65 of Copper
    Cu65,
    /// Isotope Cu66 of Copper
    Cu66,
    /// Isotope Cu67 of Copper
    Cu67,
    /// Isotope Cu68 of Copper
    Cu68,
    /// Isotope Cu69 of Copper
    Cu69,
    /// Isotope Cu70 of Copper
    Cu70,
    /// Isotope Cu71 of Copper
    Cu71,
    /// Isotope Cu72 of Copper
    Cu72,
    /// Isotope Cu73 of Copper
    Cu73,
    /// Isotope Cu74 of Copper
    Cu74,
    /// Isotope Cu75 of Copper
    Cu75,
    /// Isotope Cu76 of Copper
    Cu76,
    /// Isotope Cu77 of Copper
    Cu77,
    /// Isotope Cu78 of Copper
    Cu78,
    /// Isotope Cu79 of Copper
    Cu79,
    /// Isotope Cu80 of Copper
    Cu80,
    /// Isotope Cu81 of Copper
    Cu81,
    /// Isotope Cu82 of Copper
    Cu82,
}
impl super::RelativeAtomicMass for CopperIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cu52 => 51.99671f64,
            Self::Cu53 => 52.98459f64,
            Self::Cu54 => 53.97666f64,
            Self::Cu55 => 54.96604f64,
            Self::Cu56 => 55.95895f64,
            Self::Cu57 => 56.9492125f64,
            Self::Cu58 => 57.94453305f64,
            Self::Cu59 => 58.93949748f64,
            Self::Cu60 => 59.9373645f64,
            Self::Cu61 => 60.9334576f64,
            Self::Cu62 => 61.93259541f64,
            Self::Cu63 => 62.92959772f64,
            Self::Cu64 => 63.92976434f64,
            Self::Cu65 => 64.9277897f64,
            Self::Cu66 => 65.92886903f64,
            Self::Cu67 => 66.9277303f64,
            Self::Cu68 => 67.9296109f64,
            Self::Cu69 => 68.9294293f64,
            Self::Cu70 => 69.9323921f64,
            Self::Cu71 => 70.9326768f64,
            Self::Cu72 => 71.9358203f64,
            Self::Cu73 => 72.9366744f64,
            Self::Cu74 => 73.9398749f64,
            Self::Cu75 => 74.9415226f64,
            Self::Cu76 => 75.945275f64,
            Self::Cu77 => 76.94792f64,
            Self::Cu78 => 77.95223f64,
            Self::Cu79 => 78.95502f64,
            Self::Cu80 => 79.96089f64,
            Self::Cu81 => 80.96587f64,
            Self::Cu82 => 81.97244f64,
        }
    }
}
impl super::ElementVariant for CopperIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cu
    }
}
impl super::MassNumber for CopperIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cu52 => 52u16,
            Self::Cu53 => 53u16,
            Self::Cu54 => 54u16,
            Self::Cu55 => 55u16,
            Self::Cu56 => 56u16,
            Self::Cu57 => 57u16,
            Self::Cu58 => 58u16,
            Self::Cu59 => 59u16,
            Self::Cu60 => 60u16,
            Self::Cu61 => 61u16,
            Self::Cu62 => 62u16,
            Self::Cu63 => 63u16,
            Self::Cu64 => 64u16,
            Self::Cu65 => 65u16,
            Self::Cu66 => 66u16,
            Self::Cu67 => 67u16,
            Self::Cu68 => 68u16,
            Self::Cu69 => 69u16,
            Self::Cu70 => 70u16,
            Self::Cu71 => 71u16,
            Self::Cu72 => 72u16,
            Self::Cu73 => 73u16,
            Self::Cu74 => 74u16,
            Self::Cu75 => 75u16,
            Self::Cu76 => 76u16,
            Self::Cu77 => 77u16,
            Self::Cu78 => 78u16,
            Self::Cu79 => 79u16,
            Self::Cu80 => 80u16,
            Self::Cu81 => 81u16,
            Self::Cu82 => 82u16,
        }
    }
}
impl super::IsotopicComposition for CopperIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cu63 => Some(0.6915f64),
            Self::Cu65 => Some(0.3085f64),
            Self::Cu52
            | Self::Cu53
            | Self::Cu54
            | Self::Cu55
            | Self::Cu56
            | Self::Cu57
            | Self::Cu58
            | Self::Cu59
            | Self::Cu60
            | Self::Cu61
            | Self::Cu62
            | Self::Cu64
            | Self::Cu66
            | Self::Cu67
            | Self::Cu68
            | Self::Cu69
            | Self::Cu70
            | Self::Cu71
            | Self::Cu72
            | Self::Cu73
            | Self::Cu74
            | Self::Cu75
            | Self::Cu76
            | Self::Cu77
            | Self::Cu78
            | Self::Cu79
            | Self::Cu80
            | Self::Cu81
            | Self::Cu82 => None,
        }
    }
}
impl super::MostAbundantIsotope for CopperIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cu63
    }
}
impl TryFrom<u16> for CopperIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            52u16 => Ok(Self::Cu52),
            53u16 => Ok(Self::Cu53),
            54u16 => Ok(Self::Cu54),
            55u16 => Ok(Self::Cu55),
            56u16 => Ok(Self::Cu56),
            57u16 => Ok(Self::Cu57),
            58u16 => Ok(Self::Cu58),
            59u16 => Ok(Self::Cu59),
            60u16 => Ok(Self::Cu60),
            61u16 => Ok(Self::Cu61),
            62u16 => Ok(Self::Cu62),
            63u16 => Ok(Self::Cu63),
            64u16 => Ok(Self::Cu64),
            65u16 => Ok(Self::Cu65),
            66u16 => Ok(Self::Cu66),
            67u16 => Ok(Self::Cu67),
            68u16 => Ok(Self::Cu68),
            69u16 => Ok(Self::Cu69),
            70u16 => Ok(Self::Cu70),
            71u16 => Ok(Self::Cu71),
            72u16 => Ok(Self::Cu72),
            73u16 => Ok(Self::Cu73),
            74u16 => Ok(Self::Cu74),
            75u16 => Ok(Self::Cu75),
            76u16 => Ok(Self::Cu76),
            77u16 => Ok(Self::Cu77),
            78u16 => Ok(Self::Cu78),
            79u16 => Ok(Self::Cu79),
            80u16 => Ok(Self::Cu80),
            81u16 => Ok(Self::Cu81),
            82u16 => Ok(Self::Cu82),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cu, value)),
        }
    }
}
impl std::fmt::Display for CopperIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cu52 => write!(f, "Cu52"),
            Self::Cu53 => write!(f, "Cu53"),
            Self::Cu54 => write!(f, "Cu54"),
            Self::Cu55 => write!(f, "Cu55"),
            Self::Cu56 => write!(f, "Cu56"),
            Self::Cu57 => write!(f, "Cu57"),
            Self::Cu58 => write!(f, "Cu58"),
            Self::Cu59 => write!(f, "Cu59"),
            Self::Cu60 => write!(f, "Cu60"),
            Self::Cu61 => write!(f, "Cu61"),
            Self::Cu62 => write!(f, "Cu62"),
            Self::Cu63 => write!(f, "Cu63"),
            Self::Cu64 => write!(f, "Cu64"),
            Self::Cu65 => write!(f, "Cu65"),
            Self::Cu66 => write!(f, "Cu66"),
            Self::Cu67 => write!(f, "Cu67"),
            Self::Cu68 => write!(f, "Cu68"),
            Self::Cu69 => write!(f, "Cu69"),
            Self::Cu70 => write!(f, "Cu70"),
            Self::Cu71 => write!(f, "Cu71"),
            Self::Cu72 => write!(f, "Cu72"),
            Self::Cu73 => write!(f, "Cu73"),
            Self::Cu74 => write!(f, "Cu74"),
            Self::Cu75 => write!(f, "Cu75"),
            Self::Cu76 => write!(f, "Cu76"),
            Self::Cu77 => write!(f, "Cu77"),
            Self::Cu78 => write!(f, "Cu78"),
            Self::Cu79 => write!(f, "Cu79"),
            Self::Cu80 => write!(f, "Cu80"),
            Self::Cu81 => write!(f, "Cu81"),
            Self::Cu82 => write!(f, "Cu82"),
        }
    }
}
