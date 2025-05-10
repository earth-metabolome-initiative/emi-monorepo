//! Isotopes of the element Strontium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Strontium
pub enum StrontiumIsotope {
    /// Isotope Sr73 of Strontium
    Sr73,
    /// Isotope Sr74 of Strontium
    Sr74,
    /// Isotope Sr75 of Strontium
    Sr75,
    /// Isotope Sr76 of Strontium
    Sr76,
    /// Isotope Sr77 of Strontium
    Sr77,
    /// Isotope Sr78 of Strontium
    Sr78,
    /// Isotope Sr79 of Strontium
    Sr79,
    /// Isotope Sr80 of Strontium
    Sr80,
    /// Isotope Sr81 of Strontium
    Sr81,
    /// Isotope Sr82 of Strontium
    Sr82,
    /// Isotope Sr83 of Strontium
    Sr83,
    /// Isotope Sr84 of Strontium
    Sr84,
    /// Isotope Sr85 of Strontium
    Sr85,
    /// Isotope Sr86 of Strontium
    Sr86,
    /// Isotope Sr87 of Strontium
    Sr87,
    /// Isotope Sr88 of Strontium
    Sr88,
    /// Isotope Sr89 of Strontium
    Sr89,
    /// Isotope Sr90 of Strontium
    Sr90,
    /// Isotope Sr91 of Strontium
    Sr91,
    /// Isotope Sr92 of Strontium
    Sr92,
    /// Isotope Sr93 of Strontium
    Sr93,
    /// Isotope Sr94 of Strontium
    Sr94,
    /// Isotope Sr95 of Strontium
    Sr95,
    /// Isotope Sr96 of Strontium
    Sr96,
    /// Isotope Sr97 of Strontium
    Sr97,
    /// Isotope Sr98 of Strontium
    Sr98,
    /// Isotope Sr99 of Strontium
    Sr99,
    /// Isotope Sr100 of Strontium
    Sr100,
    /// Isotope Sr101 of Strontium
    Sr101,
    /// Isotope Sr102 of Strontium
    Sr102,
    /// Isotope Sr103 of Strontium
    Sr103,
    /// Isotope Sr104 of Strontium
    Sr104,
    /// Isotope Sr105 of Strontium
    Sr105,
    /// Isotope Sr106 of Strontium
    Sr106,
    /// Isotope Sr107 of Strontium
    Sr107,
}
impl super::RelativeAtomicMass for StrontiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sr73 => 72.9657f64,
            Self::Sr74 => 73.95617f64,
            Self::Sr75 => 74.94995f64,
            Self::Sr76 => 75.941763f64,
            Self::Sr77 => 76.9379455f64,
            Self::Sr78 => 77.93218f64,
            Self::Sr79 => 78.9297077f64,
            Self::Sr80 => 79.9245175f64,
            Self::Sr81 => 80.9232114f64,
            Self::Sr82 => 81.9183999f64,
            Self::Sr83 => 82.9175544f64,
            Self::Sr84 => 83.9134191f64,
            Self::Sr85 => 84.912932f64,
            Self::Sr86 => 85.9092606f64,
            Self::Sr87 => 86.9088775f64,
            Self::Sr88 => 87.9056125f64,
            Self::Sr89 => 88.9074511f64,
            Self::Sr90 => 89.90773f64,
            Self::Sr91 => 90.9101954f64,
            Self::Sr92 => 91.9110382f64,
            Self::Sr93 => 92.9140242f64,
            Self::Sr94 => 93.9153556f64,
            Self::Sr95 => 94.9193529f64,
            Self::Sr96 => 95.9217066f64,
            Self::Sr97 => 96.926374f64,
            Self::Sr98 => 97.9286888f64,
            Self::Sr99 => 98.9328907f64,
            Self::Sr100 => 99.93577f64,
            Self::Sr101 => 100.940352f64,
            Self::Sr102 => 101.943791f64,
            Self::Sr103 => 102.94909f64,
            Self::Sr104 => 103.95265f64,
            Self::Sr105 => 104.95855f64,
            Self::Sr106 => 105.96265f64,
            Self::Sr107 => 106.96897f64,
        }
    }
}
impl super::ElementVariant for StrontiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sr
    }
}
impl super::MassNumber for StrontiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sr73 => 73u16,
            Self::Sr74 => 74u16,
            Self::Sr75 => 75u16,
            Self::Sr76 => 76u16,
            Self::Sr77 => 77u16,
            Self::Sr78 => 78u16,
            Self::Sr79 => 79u16,
            Self::Sr80 => 80u16,
            Self::Sr81 => 81u16,
            Self::Sr82 => 82u16,
            Self::Sr83 => 83u16,
            Self::Sr84 => 84u16,
            Self::Sr85 => 85u16,
            Self::Sr86 => 86u16,
            Self::Sr87 => 87u16,
            Self::Sr88 => 88u16,
            Self::Sr89 => 89u16,
            Self::Sr90 => 90u16,
            Self::Sr91 => 91u16,
            Self::Sr92 => 92u16,
            Self::Sr93 => 93u16,
            Self::Sr94 => 94u16,
            Self::Sr95 => 95u16,
            Self::Sr96 => 96u16,
            Self::Sr97 => 97u16,
            Self::Sr98 => 98u16,
            Self::Sr99 => 99u16,
            Self::Sr100 => 100u16,
            Self::Sr101 => 101u16,
            Self::Sr102 => 102u16,
            Self::Sr103 => 103u16,
            Self::Sr104 => 104u16,
            Self::Sr105 => 105u16,
            Self::Sr106 => 106u16,
            Self::Sr107 => 107u16,
        }
    }
}
impl super::IsotopicComposition for StrontiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sr84 => Some(0.0056f64),
            Self::Sr86 => Some(0.0986f64),
            Self::Sr87 => Some(0.07f64),
            Self::Sr88 => Some(0.8258f64),
            Self::Sr73
            | Self::Sr74
            | Self::Sr75
            | Self::Sr76
            | Self::Sr77
            | Self::Sr78
            | Self::Sr79
            | Self::Sr80
            | Self::Sr81
            | Self::Sr82
            | Self::Sr83
            | Self::Sr85
            | Self::Sr89
            | Self::Sr90
            | Self::Sr91
            | Self::Sr92
            | Self::Sr93
            | Self::Sr94
            | Self::Sr95
            | Self::Sr96
            | Self::Sr97
            | Self::Sr98
            | Self::Sr99
            | Self::Sr100
            | Self::Sr101
            | Self::Sr102
            | Self::Sr103
            | Self::Sr104
            | Self::Sr105
            | Self::Sr106
            | Self::Sr107 => None,
        }
    }
}
impl super::MostAbundantIsotope for StrontiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sr88
    }
}
impl TryFrom<u16> for StrontiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            73u16 => Ok(Self::Sr73),
            74u16 => Ok(Self::Sr74),
            75u16 => Ok(Self::Sr75),
            76u16 => Ok(Self::Sr76),
            77u16 => Ok(Self::Sr77),
            78u16 => Ok(Self::Sr78),
            79u16 => Ok(Self::Sr79),
            80u16 => Ok(Self::Sr80),
            81u16 => Ok(Self::Sr81),
            82u16 => Ok(Self::Sr82),
            83u16 => Ok(Self::Sr83),
            84u16 => Ok(Self::Sr84),
            85u16 => Ok(Self::Sr85),
            86u16 => Ok(Self::Sr86),
            87u16 => Ok(Self::Sr87),
            88u16 => Ok(Self::Sr88),
            89u16 => Ok(Self::Sr89),
            90u16 => Ok(Self::Sr90),
            91u16 => Ok(Self::Sr91),
            92u16 => Ok(Self::Sr92),
            93u16 => Ok(Self::Sr93),
            94u16 => Ok(Self::Sr94),
            95u16 => Ok(Self::Sr95),
            96u16 => Ok(Self::Sr96),
            97u16 => Ok(Self::Sr97),
            98u16 => Ok(Self::Sr98),
            99u16 => Ok(Self::Sr99),
            100u16 => Ok(Self::Sr100),
            101u16 => Ok(Self::Sr101),
            102u16 => Ok(Self::Sr102),
            103u16 => Ok(Self::Sr103),
            104u16 => Ok(Self::Sr104),
            105u16 => Ok(Self::Sr105),
            106u16 => Ok(Self::Sr106),
            107u16 => Ok(Self::Sr107),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sr, value)),
        }
    }
}
impl std::fmt::Display for StrontiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sr73 => write!(f, "Sr73"),
            Self::Sr74 => write!(f, "Sr74"),
            Self::Sr75 => write!(f, "Sr75"),
            Self::Sr76 => write!(f, "Sr76"),
            Self::Sr77 => write!(f, "Sr77"),
            Self::Sr78 => write!(f, "Sr78"),
            Self::Sr79 => write!(f, "Sr79"),
            Self::Sr80 => write!(f, "Sr80"),
            Self::Sr81 => write!(f, "Sr81"),
            Self::Sr82 => write!(f, "Sr82"),
            Self::Sr83 => write!(f, "Sr83"),
            Self::Sr84 => write!(f, "Sr84"),
            Self::Sr85 => write!(f, "Sr85"),
            Self::Sr86 => write!(f, "Sr86"),
            Self::Sr87 => write!(f, "Sr87"),
            Self::Sr88 => write!(f, "Sr88"),
            Self::Sr89 => write!(f, "Sr89"),
            Self::Sr90 => write!(f, "Sr90"),
            Self::Sr91 => write!(f, "Sr91"),
            Self::Sr92 => write!(f, "Sr92"),
            Self::Sr93 => write!(f, "Sr93"),
            Self::Sr94 => write!(f, "Sr94"),
            Self::Sr95 => write!(f, "Sr95"),
            Self::Sr96 => write!(f, "Sr96"),
            Self::Sr97 => write!(f, "Sr97"),
            Self::Sr98 => write!(f, "Sr98"),
            Self::Sr99 => write!(f, "Sr99"),
            Self::Sr100 => write!(f, "Sr100"),
            Self::Sr101 => write!(f, "Sr101"),
            Self::Sr102 => write!(f, "Sr102"),
            Self::Sr103 => write!(f, "Sr103"),
            Self::Sr104 => write!(f, "Sr104"),
            Self::Sr105 => write!(f, "Sr105"),
            Self::Sr106 => write!(f, "Sr106"),
            Self::Sr107 => write!(f, "Sr107"),
        }
    }
}
