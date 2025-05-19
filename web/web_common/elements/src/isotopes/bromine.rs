//! Isotopes of the element Bromine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Bromine
pub enum BromineIsotope {
    /// Isotope Br67 of Bromine
    Br67,
    /// Isotope Br68 of Bromine
    Br68,
    /// Isotope Br69 of Bromine
    Br69,
    /// Isotope Br70 of Bromine
    Br70,
    /// Isotope Br71 of Bromine
    Br71,
    /// Isotope Br72 of Bromine
    Br72,
    /// Isotope Br73 of Bromine
    Br73,
    /// Isotope Br74 of Bromine
    Br74,
    /// Isotope Br75 of Bromine
    Br75,
    /// Isotope Br76 of Bromine
    Br76,
    /// Isotope Br77 of Bromine
    Br77,
    /// Isotope Br78 of Bromine
    Br78,
    /// Isotope Br79 of Bromine
    Br79,
    /// Isotope Br80 of Bromine
    Br80,
    /// Isotope Br81 of Bromine
    Br81,
    /// Isotope Br82 of Bromine
    Br82,
    /// Isotope Br83 of Bromine
    Br83,
    /// Isotope Br84 of Bromine
    Br84,
    /// Isotope Br85 of Bromine
    Br85,
    /// Isotope Br86 of Bromine
    Br86,
    /// Isotope Br87 of Bromine
    Br87,
    /// Isotope Br88 of Bromine
    Br88,
    /// Isotope Br89 of Bromine
    Br89,
    /// Isotope Br90 of Bromine
    Br90,
    /// Isotope Br91 of Bromine
    Br91,
    /// Isotope Br92 of Bromine
    Br92,
    /// Isotope Br93 of Bromine
    Br93,
    /// Isotope Br94 of Bromine
    Br94,
    /// Isotope Br95 of Bromine
    Br95,
    /// Isotope Br96 of Bromine
    Br96,
    /// Isotope Br97 of Bromine
    Br97,
    /// Isotope Br98 of Bromine
    Br98,
}
impl super::RelativeAtomicMass for BromineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Br67 => 66.96465f64,
            Self::Br68 => 67.95873f64,
            Self::Br69 => 68.950497f64,
            Self::Br70 => 69.944792f64,
            Self::Br71 => 70.9393422f64,
            Self::Br72 => 71.9365886f64,
            Self::Br73 => 72.9316715f64,
            Self::Br74 => 73.9299102f64,
            Self::Br75 => 74.9258105f64,
            Self::Br76 => 75.924542f64,
            Self::Br77 => 76.9213792f64,
            Self::Br78 => 77.9211459f64,
            Self::Br79 => 78.9183376f64,
            Self::Br80 => 79.9185298f64,
            Self::Br81 => 80.9162897f64,
            Self::Br82 => 81.9168032f64,
            Self::Br83 => 82.9151756f64,
            Self::Br84 => 83.916496f64,
            Self::Br85 => 84.9156458f64,
            Self::Br86 => 85.9188054f64,
            Self::Br87 => 86.920674f64,
            Self::Br88 => 87.9240833f64,
            Self::Br89 => 88.9267046f64,
            Self::Br90 => 89.9312928f64,
            Self::Br91 => 90.9343986f64,
            Self::Br92 => 91.9396316f64,
            Self::Br93 => 92.94313f64,
            Self::Br94 => 93.9489f64,
            Self::Br95 => 94.95301f64,
            Self::Br96 => 95.95903f64,
            Self::Br97 => 96.96344f64,
            Self::Br98 => 97.96946f64,
        }
    }
}
impl super::ElementVariant for BromineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Br
    }
}
impl super::MassNumber for BromineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Br67 => 67u16,
            Self::Br68 => 68u16,
            Self::Br69 => 69u16,
            Self::Br70 => 70u16,
            Self::Br71 => 71u16,
            Self::Br72 => 72u16,
            Self::Br73 => 73u16,
            Self::Br74 => 74u16,
            Self::Br75 => 75u16,
            Self::Br76 => 76u16,
            Self::Br77 => 77u16,
            Self::Br78 => 78u16,
            Self::Br79 => 79u16,
            Self::Br80 => 80u16,
            Self::Br81 => 81u16,
            Self::Br82 => 82u16,
            Self::Br83 => 83u16,
            Self::Br84 => 84u16,
            Self::Br85 => 85u16,
            Self::Br86 => 86u16,
            Self::Br87 => 87u16,
            Self::Br88 => 88u16,
            Self::Br89 => 89u16,
            Self::Br90 => 90u16,
            Self::Br91 => 91u16,
            Self::Br92 => 92u16,
            Self::Br93 => 93u16,
            Self::Br94 => 94u16,
            Self::Br95 => 95u16,
            Self::Br96 => 96u16,
            Self::Br97 => 97u16,
            Self::Br98 => 98u16,
        }
    }
}
impl super::IsotopicComposition for BromineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Br79 => Some(0.5069f64),
            Self::Br81 => Some(0.4931f64),
            Self::Br67
            | Self::Br68
            | Self::Br69
            | Self::Br70
            | Self::Br71
            | Self::Br72
            | Self::Br73
            | Self::Br74
            | Self::Br75
            | Self::Br76
            | Self::Br77
            | Self::Br78
            | Self::Br80
            | Self::Br82
            | Self::Br83
            | Self::Br84
            | Self::Br85
            | Self::Br86
            | Self::Br87
            | Self::Br88
            | Self::Br89
            | Self::Br90
            | Self::Br91
            | Self::Br92
            | Self::Br93
            | Self::Br94
            | Self::Br95
            | Self::Br96
            | Self::Br97
            | Self::Br98 => None,
        }
    }
}
impl super::MostAbundantIsotope for BromineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Br79
    }
}
impl TryFrom<u16> for BromineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            67u16 => Ok(Self::Br67),
            68u16 => Ok(Self::Br68),
            69u16 => Ok(Self::Br69),
            70u16 => Ok(Self::Br70),
            71u16 => Ok(Self::Br71),
            72u16 => Ok(Self::Br72),
            73u16 => Ok(Self::Br73),
            74u16 => Ok(Self::Br74),
            75u16 => Ok(Self::Br75),
            76u16 => Ok(Self::Br76),
            77u16 => Ok(Self::Br77),
            78u16 => Ok(Self::Br78),
            79u16 => Ok(Self::Br79),
            80u16 => Ok(Self::Br80),
            81u16 => Ok(Self::Br81),
            82u16 => Ok(Self::Br82),
            83u16 => Ok(Self::Br83),
            84u16 => Ok(Self::Br84),
            85u16 => Ok(Self::Br85),
            86u16 => Ok(Self::Br86),
            87u16 => Ok(Self::Br87),
            88u16 => Ok(Self::Br88),
            89u16 => Ok(Self::Br89),
            90u16 => Ok(Self::Br90),
            91u16 => Ok(Self::Br91),
            92u16 => Ok(Self::Br92),
            93u16 => Ok(Self::Br93),
            94u16 => Ok(Self::Br94),
            95u16 => Ok(Self::Br95),
            96u16 => Ok(Self::Br96),
            97u16 => Ok(Self::Br97),
            98u16 => Ok(Self::Br98),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Br, value)),
        }
    }
}
impl std::fmt::Display for BromineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Br67 => write!(f, "Br67"),
            Self::Br68 => write!(f, "Br68"),
            Self::Br69 => write!(f, "Br69"),
            Self::Br70 => write!(f, "Br70"),
            Self::Br71 => write!(f, "Br71"),
            Self::Br72 => write!(f, "Br72"),
            Self::Br73 => write!(f, "Br73"),
            Self::Br74 => write!(f, "Br74"),
            Self::Br75 => write!(f, "Br75"),
            Self::Br76 => write!(f, "Br76"),
            Self::Br77 => write!(f, "Br77"),
            Self::Br78 => write!(f, "Br78"),
            Self::Br79 => write!(f, "Br79"),
            Self::Br80 => write!(f, "Br80"),
            Self::Br81 => write!(f, "Br81"),
            Self::Br82 => write!(f, "Br82"),
            Self::Br83 => write!(f, "Br83"),
            Self::Br84 => write!(f, "Br84"),
            Self::Br85 => write!(f, "Br85"),
            Self::Br86 => write!(f, "Br86"),
            Self::Br87 => write!(f, "Br87"),
            Self::Br88 => write!(f, "Br88"),
            Self::Br89 => write!(f, "Br89"),
            Self::Br90 => write!(f, "Br90"),
            Self::Br91 => write!(f, "Br91"),
            Self::Br92 => write!(f, "Br92"),
            Self::Br93 => write!(f, "Br93"),
            Self::Br94 => write!(f, "Br94"),
            Self::Br95 => write!(f, "Br95"),
            Self::Br96 => write!(f, "Br96"),
            Self::Br97 => write!(f, "Br97"),
            Self::Br98 => write!(f, "Br98"),
        }
    }
}
