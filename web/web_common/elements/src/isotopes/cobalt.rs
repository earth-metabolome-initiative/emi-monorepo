//! Isotopes of the element Cobalt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Cobalt
pub enum CobaltIsotope {
    /// Isotope Co47 of Cobalt
    Co47,
    /// Isotope Co48 of Cobalt
    Co48,
    /// Isotope Co49 of Cobalt
    Co49,
    /// Isotope Co50 of Cobalt
    Co50,
    /// Isotope Co51 of Cobalt
    Co51,
    /// Isotope Co52 of Cobalt
    Co52,
    /// Isotope Co53 of Cobalt
    Co53,
    /// Isotope Co54 of Cobalt
    Co54,
    /// Isotope Co55 of Cobalt
    Co55,
    /// Isotope Co56 of Cobalt
    Co56,
    /// Isotope Co57 of Cobalt
    Co57,
    /// Isotope Co58 of Cobalt
    Co58,
    /// Isotope Co59 of Cobalt
    Co59,
    /// Isotope Co60 of Cobalt
    Co60,
    /// Isotope Co61 of Cobalt
    Co61,
    /// Isotope Co62 of Cobalt
    Co62,
    /// Isotope Co63 of Cobalt
    Co63,
    /// Isotope Co64 of Cobalt
    Co64,
    /// Isotope Co65 of Cobalt
    Co65,
    /// Isotope Co66 of Cobalt
    Co66,
    /// Isotope Co67 of Cobalt
    Co67,
    /// Isotope Co68 of Cobalt
    Co68,
    /// Isotope Co69 of Cobalt
    Co69,
    /// Isotope Co70 of Cobalt
    Co70,
    /// Isotope Co71 of Cobalt
    Co71,
    /// Isotope Co72 of Cobalt
    Co72,
    /// Isotope Co73 of Cobalt
    Co73,
    /// Isotope Co74 of Cobalt
    Co74,
    /// Isotope Co75 of Cobalt
    Co75,
    /// Isotope Co76 of Cobalt
    Co76,
}
impl super::RelativeAtomicMass for CobaltIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Co47 => 47.01057f64,
            Self::Co48 => 48.00093f64,
            Self::Co49 => 48.98891f64,
            Self::Co50 => 49.98091f64,
            Self::Co51 => 50.970647f64,
            Self::Co52 => 51.96351f64,
            Self::Co53 => 52.9542041f64,
            Self::Co54 => 53.94845987f64,
            Self::Co55 => 54.9419972f64,
            Self::Co56 => 55.9398388f64,
            Self::Co57 => 56.93629057f64,
            Self::Co58 => 57.9357521f64,
            Self::Co59 => 58.93319429f64,
            Self::Co60 => 59.9338163f64,
            Self::Co61 => 60.93247662f64,
            Self::Co62 => 61.934059f64,
            Self::Co63 => 62.9336f64,
            Self::Co64 => 63.935811f64,
            Self::Co65 => 64.9364621f64,
            Self::Co66 => 65.939443f64,
            Self::Co67 => 66.9406096f64,
            Self::Co68 => 67.94426f64,
            Self::Co69 => 68.94614f64,
            Self::Co70 => 69.94963f64,
            Self::Co71 => 70.95237f64,
            Self::Co72 => 71.95729f64,
            Self::Co73 => 72.96039f64,
            Self::Co74 => 73.96515f64,
            Self::Co75 => 74.96876f64,
            Self::Co76 => 75.97413f64,
        }
    }
}
impl super::ElementVariant for CobaltIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Co
    }
}
impl super::MassNumber for CobaltIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Co47 => 47u16,
            Self::Co48 => 48u16,
            Self::Co49 => 49u16,
            Self::Co50 => 50u16,
            Self::Co51 => 51u16,
            Self::Co52 => 52u16,
            Self::Co53 => 53u16,
            Self::Co54 => 54u16,
            Self::Co55 => 55u16,
            Self::Co56 => 56u16,
            Self::Co57 => 57u16,
            Self::Co58 => 58u16,
            Self::Co59 => 59u16,
            Self::Co60 => 60u16,
            Self::Co61 => 61u16,
            Self::Co62 => 62u16,
            Self::Co63 => 63u16,
            Self::Co64 => 64u16,
            Self::Co65 => 65u16,
            Self::Co66 => 66u16,
            Self::Co67 => 67u16,
            Self::Co68 => 68u16,
            Self::Co69 => 69u16,
            Self::Co70 => 70u16,
            Self::Co71 => 71u16,
            Self::Co72 => 72u16,
            Self::Co73 => 73u16,
            Self::Co74 => 74u16,
            Self::Co75 => 75u16,
            Self::Co76 => 76u16,
        }
    }
}
impl super::IsotopicComposition for CobaltIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Co59 => Some(1f64),
            Self::Co47
            | Self::Co48
            | Self::Co49
            | Self::Co50
            | Self::Co51
            | Self::Co52
            | Self::Co53
            | Self::Co54
            | Self::Co55
            | Self::Co56
            | Self::Co57
            | Self::Co58
            | Self::Co60
            | Self::Co61
            | Self::Co62
            | Self::Co63
            | Self::Co64
            | Self::Co65
            | Self::Co66
            | Self::Co67
            | Self::Co68
            | Self::Co69
            | Self::Co70
            | Self::Co71
            | Self::Co72
            | Self::Co73
            | Self::Co74
            | Self::Co75
            | Self::Co76 => None,
        }
    }
}
impl super::MostAbundantIsotope for CobaltIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Co59
    }
}
impl TryFrom<u16> for CobaltIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            47u16 => Ok(Self::Co47),
            48u16 => Ok(Self::Co48),
            49u16 => Ok(Self::Co49),
            50u16 => Ok(Self::Co50),
            51u16 => Ok(Self::Co51),
            52u16 => Ok(Self::Co52),
            53u16 => Ok(Self::Co53),
            54u16 => Ok(Self::Co54),
            55u16 => Ok(Self::Co55),
            56u16 => Ok(Self::Co56),
            57u16 => Ok(Self::Co57),
            58u16 => Ok(Self::Co58),
            59u16 => Ok(Self::Co59),
            60u16 => Ok(Self::Co60),
            61u16 => Ok(Self::Co61),
            62u16 => Ok(Self::Co62),
            63u16 => Ok(Self::Co63),
            64u16 => Ok(Self::Co64),
            65u16 => Ok(Self::Co65),
            66u16 => Ok(Self::Co66),
            67u16 => Ok(Self::Co67),
            68u16 => Ok(Self::Co68),
            69u16 => Ok(Self::Co69),
            70u16 => Ok(Self::Co70),
            71u16 => Ok(Self::Co71),
            72u16 => Ok(Self::Co72),
            73u16 => Ok(Self::Co73),
            74u16 => Ok(Self::Co74),
            75u16 => Ok(Self::Co75),
            76u16 => Ok(Self::Co76),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Co, value)),
        }
    }
}
impl std::fmt::Display for CobaltIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Co47 => write!(f, "Co47"),
            Self::Co48 => write!(f, "Co48"),
            Self::Co49 => write!(f, "Co49"),
            Self::Co50 => write!(f, "Co50"),
            Self::Co51 => write!(f, "Co51"),
            Self::Co52 => write!(f, "Co52"),
            Self::Co53 => write!(f, "Co53"),
            Self::Co54 => write!(f, "Co54"),
            Self::Co55 => write!(f, "Co55"),
            Self::Co56 => write!(f, "Co56"),
            Self::Co57 => write!(f, "Co57"),
            Self::Co58 => write!(f, "Co58"),
            Self::Co59 => write!(f, "Co59"),
            Self::Co60 => write!(f, "Co60"),
            Self::Co61 => write!(f, "Co61"),
            Self::Co62 => write!(f, "Co62"),
            Self::Co63 => write!(f, "Co63"),
            Self::Co64 => write!(f, "Co64"),
            Self::Co65 => write!(f, "Co65"),
            Self::Co66 => write!(f, "Co66"),
            Self::Co67 => write!(f, "Co67"),
            Self::Co68 => write!(f, "Co68"),
            Self::Co69 => write!(f, "Co69"),
            Self::Co70 => write!(f, "Co70"),
            Self::Co71 => write!(f, "Co71"),
            Self::Co72 => write!(f, "Co72"),
            Self::Co73 => write!(f, "Co73"),
            Self::Co74 => write!(f, "Co74"),
            Self::Co75 => write!(f, "Co75"),
            Self::Co76 => write!(f, "Co76"),
        }
    }
}
