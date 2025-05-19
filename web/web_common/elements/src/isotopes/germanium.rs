//! Isotopes of the element Germanium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Germanium
pub enum GermaniumIsotope {
    /// Isotope Ge58 of Germanium
    Ge58,
    /// Isotope Ge59 of Germanium
    Ge59,
    /// Isotope Ge60 of Germanium
    Ge60,
    /// Isotope Ge61 of Germanium
    Ge61,
    /// Isotope Ge62 of Germanium
    Ge62,
    /// Isotope Ge63 of Germanium
    Ge63,
    /// Isotope Ge64 of Germanium
    Ge64,
    /// Isotope Ge65 of Germanium
    Ge65,
    /// Isotope Ge66 of Germanium
    Ge66,
    /// Isotope Ge67 of Germanium
    Ge67,
    /// Isotope Ge68 of Germanium
    Ge68,
    /// Isotope Ge69 of Germanium
    Ge69,
    /// Isotope Ge70 of Germanium
    Ge70,
    /// Isotope Ge71 of Germanium
    Ge71,
    /// Isotope Ge72 of Germanium
    Ge72,
    /// Isotope Ge73 of Germanium
    Ge73,
    /// Isotope Ge74 of Germanium
    Ge74,
    /// Isotope Ge75 of Germanium
    Ge75,
    /// Isotope Ge76 of Germanium
    Ge76,
    /// Isotope Ge77 of Germanium
    Ge77,
    /// Isotope Ge78 of Germanium
    Ge78,
    /// Isotope Ge79 of Germanium
    Ge79,
    /// Isotope Ge80 of Germanium
    Ge80,
    /// Isotope Ge81 of Germanium
    Ge81,
    /// Isotope Ge82 of Germanium
    Ge82,
    /// Isotope Ge83 of Germanium
    Ge83,
    /// Isotope Ge84 of Germanium
    Ge84,
    /// Isotope Ge85 of Germanium
    Ge85,
    /// Isotope Ge86 of Germanium
    Ge86,
    /// Isotope Ge87 of Germanium
    Ge87,
    /// Isotope Ge88 of Germanium
    Ge88,
    /// Isotope Ge89 of Germanium
    Ge89,
    /// Isotope Ge90 of Germanium
    Ge90,
}
impl super::RelativeAtomicMass for GermaniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ge58 => 57.99172f64,
            Self::Ge59 => 58.98249f64,
            Self::Ge60 => 59.97036f64,
            Self::Ge61 => 60.96379f64,
            Self::Ge62 => 61.95502f64,
            Self::Ge63 => 62.949628f64,
            Self::Ge64 => 63.9416899f64,
            Self::Ge65 => 64.9393681f64,
            Self::Ge66 => 65.9338621f64,
            Self::Ge67 => 66.9327339f64,
            Self::Ge68 => 67.9280953f64,
            Self::Ge69 => 68.9279645f64,
            Self::Ge70 => 69.92424875f64,
            Self::Ge71 => 70.92495233f64,
            Self::Ge72 => 71.922075826f64,
            Self::Ge73 => 72.923458956f64,
            Self::Ge74 => 73.921177761f64,
            Self::Ge75 => 74.92285837f64,
            Self::Ge76 => 75.921402726f64,
            Self::Ge77 => 76.923549843f64,
            Self::Ge78 => 77.9228529f64,
            Self::Ge79 => 78.92536f64,
            Self::Ge80 => 79.9253508f64,
            Self::Ge81 => 80.9288329f64,
            Self::Ge82 => 81.929774f64,
            Self::Ge83 => 82.9345391f64,
            Self::Ge84 => 83.9375751f64,
            Self::Ge85 => 84.9429697f64,
            Self::Ge86 => 85.94658f64,
            Self::Ge87 => 86.95268f64,
            Self::Ge88 => 87.95691f64,
            Self::Ge89 => 88.96379f64,
            Self::Ge90 => 89.96863f64,
        }
    }
}
impl super::ElementVariant for GermaniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ge
    }
}
impl super::MassNumber for GermaniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ge58 => 58u16,
            Self::Ge59 => 59u16,
            Self::Ge60 => 60u16,
            Self::Ge61 => 61u16,
            Self::Ge62 => 62u16,
            Self::Ge63 => 63u16,
            Self::Ge64 => 64u16,
            Self::Ge65 => 65u16,
            Self::Ge66 => 66u16,
            Self::Ge67 => 67u16,
            Self::Ge68 => 68u16,
            Self::Ge69 => 69u16,
            Self::Ge70 => 70u16,
            Self::Ge71 => 71u16,
            Self::Ge72 => 72u16,
            Self::Ge73 => 73u16,
            Self::Ge74 => 74u16,
            Self::Ge75 => 75u16,
            Self::Ge76 => 76u16,
            Self::Ge77 => 77u16,
            Self::Ge78 => 78u16,
            Self::Ge79 => 79u16,
            Self::Ge80 => 80u16,
            Self::Ge81 => 81u16,
            Self::Ge82 => 82u16,
            Self::Ge83 => 83u16,
            Self::Ge84 => 84u16,
            Self::Ge85 => 85u16,
            Self::Ge86 => 86u16,
            Self::Ge87 => 87u16,
            Self::Ge88 => 88u16,
            Self::Ge89 => 89u16,
            Self::Ge90 => 90u16,
        }
    }
}
impl super::IsotopicComposition for GermaniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ge70 => Some(0.2057f64),
            Self::Ge72 => Some(0.2745f64),
            Self::Ge73 => Some(0.0775f64),
            Self::Ge74 => Some(0.365f64),
            Self::Ge76 => Some(0.0773f64),
            Self::Ge58
            | Self::Ge59
            | Self::Ge60
            | Self::Ge61
            | Self::Ge62
            | Self::Ge63
            | Self::Ge64
            | Self::Ge65
            | Self::Ge66
            | Self::Ge67
            | Self::Ge68
            | Self::Ge69
            | Self::Ge71
            | Self::Ge75
            | Self::Ge77
            | Self::Ge78
            | Self::Ge79
            | Self::Ge80
            | Self::Ge81
            | Self::Ge82
            | Self::Ge83
            | Self::Ge84
            | Self::Ge85
            | Self::Ge86
            | Self::Ge87
            | Self::Ge88
            | Self::Ge89
            | Self::Ge90 => None,
        }
    }
}
impl super::MostAbundantIsotope for GermaniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ge74
    }
}
impl TryFrom<u16> for GermaniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            58u16 => Ok(Self::Ge58),
            59u16 => Ok(Self::Ge59),
            60u16 => Ok(Self::Ge60),
            61u16 => Ok(Self::Ge61),
            62u16 => Ok(Self::Ge62),
            63u16 => Ok(Self::Ge63),
            64u16 => Ok(Self::Ge64),
            65u16 => Ok(Self::Ge65),
            66u16 => Ok(Self::Ge66),
            67u16 => Ok(Self::Ge67),
            68u16 => Ok(Self::Ge68),
            69u16 => Ok(Self::Ge69),
            70u16 => Ok(Self::Ge70),
            71u16 => Ok(Self::Ge71),
            72u16 => Ok(Self::Ge72),
            73u16 => Ok(Self::Ge73),
            74u16 => Ok(Self::Ge74),
            75u16 => Ok(Self::Ge75),
            76u16 => Ok(Self::Ge76),
            77u16 => Ok(Self::Ge77),
            78u16 => Ok(Self::Ge78),
            79u16 => Ok(Self::Ge79),
            80u16 => Ok(Self::Ge80),
            81u16 => Ok(Self::Ge81),
            82u16 => Ok(Self::Ge82),
            83u16 => Ok(Self::Ge83),
            84u16 => Ok(Self::Ge84),
            85u16 => Ok(Self::Ge85),
            86u16 => Ok(Self::Ge86),
            87u16 => Ok(Self::Ge87),
            88u16 => Ok(Self::Ge88),
            89u16 => Ok(Self::Ge89),
            90u16 => Ok(Self::Ge90),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ge, value)),
        }
    }
}
impl std::fmt::Display for GermaniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ge58 => write!(f, "Ge58"),
            Self::Ge59 => write!(f, "Ge59"),
            Self::Ge60 => write!(f, "Ge60"),
            Self::Ge61 => write!(f, "Ge61"),
            Self::Ge62 => write!(f, "Ge62"),
            Self::Ge63 => write!(f, "Ge63"),
            Self::Ge64 => write!(f, "Ge64"),
            Self::Ge65 => write!(f, "Ge65"),
            Self::Ge66 => write!(f, "Ge66"),
            Self::Ge67 => write!(f, "Ge67"),
            Self::Ge68 => write!(f, "Ge68"),
            Self::Ge69 => write!(f, "Ge69"),
            Self::Ge70 => write!(f, "Ge70"),
            Self::Ge71 => write!(f, "Ge71"),
            Self::Ge72 => write!(f, "Ge72"),
            Self::Ge73 => write!(f, "Ge73"),
            Self::Ge74 => write!(f, "Ge74"),
            Self::Ge75 => write!(f, "Ge75"),
            Self::Ge76 => write!(f, "Ge76"),
            Self::Ge77 => write!(f, "Ge77"),
            Self::Ge78 => write!(f, "Ge78"),
            Self::Ge79 => write!(f, "Ge79"),
            Self::Ge80 => write!(f, "Ge80"),
            Self::Ge81 => write!(f, "Ge81"),
            Self::Ge82 => write!(f, "Ge82"),
            Self::Ge83 => write!(f, "Ge83"),
            Self::Ge84 => write!(f, "Ge84"),
            Self::Ge85 => write!(f, "Ge85"),
            Self::Ge86 => write!(f, "Ge86"),
            Self::Ge87 => write!(f, "Ge87"),
            Self::Ge88 => write!(f, "Ge88"),
            Self::Ge89 => write!(f, "Ge89"),
            Self::Ge90 => write!(f, "Ge90"),
        }
    }
}
