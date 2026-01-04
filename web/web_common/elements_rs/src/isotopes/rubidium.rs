//! Isotopes of the element Rubidium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Rubidium
pub enum RubidiumIsotope {
    /// Isotope Rb71 of Rubidium
    Rb71,
    /// Isotope Rb72 of Rubidium
    Rb72,
    /// Isotope Rb73 of Rubidium
    Rb73,
    /// Isotope Rb74 of Rubidium
    Rb74,
    /// Isotope Rb75 of Rubidium
    Rb75,
    /// Isotope Rb76 of Rubidium
    Rb76,
    /// Isotope Rb77 of Rubidium
    Rb77,
    /// Isotope Rb78 of Rubidium
    Rb78,
    /// Isotope Rb79 of Rubidium
    Rb79,
    /// Isotope Rb80 of Rubidium
    Rb80,
    /// Isotope Rb81 of Rubidium
    Rb81,
    /// Isotope Rb82 of Rubidium
    Rb82,
    /// Isotope Rb83 of Rubidium
    Rb83,
    /// Isotope Rb84 of Rubidium
    Rb84,
    /// Isotope Rb85 of Rubidium
    Rb85,
    /// Isotope Rb86 of Rubidium
    Rb86,
    /// Isotope Rb87 of Rubidium
    Rb87,
    /// Isotope Rb88 of Rubidium
    Rb88,
    /// Isotope Rb89 of Rubidium
    Rb89,
    /// Isotope Rb90 of Rubidium
    Rb90,
    /// Isotope Rb91 of Rubidium
    Rb91,
    /// Isotope Rb92 of Rubidium
    Rb92,
    /// Isotope Rb93 of Rubidium
    Rb93,
    /// Isotope Rb94 of Rubidium
    Rb94,
    /// Isotope Rb95 of Rubidium
    Rb95,
    /// Isotope Rb96 of Rubidium
    Rb96,
    /// Isotope Rb97 of Rubidium
    Rb97,
    /// Isotope Rb98 of Rubidium
    Rb98,
    /// Isotope Rb99 of Rubidium
    Rb99,
    /// Isotope Rb100 of Rubidium
    Rb100,
    /// Isotope Rb101 of Rubidium
    Rb101,
    /// Isotope Rb102 of Rubidium
    Rb102,
    /// Isotope Rb103 of Rubidium
    Rb103,
}
impl super::RelativeAtomicMass for RubidiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rb71 => 70.96532f64,
            Self::Rb72 => 71.95908f64,
            Self::Rb73 => 72.95053f64,
            Self::Rb74 => 73.9442659f64,
            Self::Rb75 => 74.9385732f64,
            Self::Rb76 => 75.935073f64,
            Self::Rb77 => 76.9304016f64,
            Self::Rb78 => 77.9281419f64,
            Self::Rb79 => 78.9239899f64,
            Self::Rb80 => 79.9225164f64,
            Self::Rb81 => 80.9189939f64,
            Self::Rb82 => 81.918209f64,
            Self::Rb83 => 82.9151142f64,
            Self::Rb84 => 83.9143752f64,
            Self::Rb85 => 84.9117897379f64,
            Self::Rb86 => 85.91116743f64,
            Self::Rb87 => 86.909180531f64,
            Self::Rb88 => 87.91131559f64,
            Self::Rb89 => 88.9122783f64,
            Self::Rb90 => 89.9147985f64,
            Self::Rb91 => 90.9165372f64,
            Self::Rb92 => 91.9197284f64,
            Self::Rb93 => 92.9220393f64,
            Self::Rb94 => 93.9263948f64,
            Self::Rb95 => 94.92926f64,
            Self::Rb96 => 95.9341334f64,
            Self::Rb97 => 96.9371771f64,
            Self::Rb98 => 97.9416869f64,
            Self::Rb99 => 98.94503f64,
            Self::Rb100 => 99.95003f64,
            Self::Rb101 => 100.95404f64,
            Self::Rb102 => 101.95952f64,
            Self::Rb103 => 102.96392f64,
        }
    }
}
impl super::ElementVariant for RubidiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Rb
    }
}
impl super::MassNumber for RubidiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rb71 => 71u16,
            Self::Rb72 => 72u16,
            Self::Rb73 => 73u16,
            Self::Rb74 => 74u16,
            Self::Rb75 => 75u16,
            Self::Rb76 => 76u16,
            Self::Rb77 => 77u16,
            Self::Rb78 => 78u16,
            Self::Rb79 => 79u16,
            Self::Rb80 => 80u16,
            Self::Rb81 => 81u16,
            Self::Rb82 => 82u16,
            Self::Rb83 => 83u16,
            Self::Rb84 => 84u16,
            Self::Rb85 => 85u16,
            Self::Rb86 => 86u16,
            Self::Rb87 => 87u16,
            Self::Rb88 => 88u16,
            Self::Rb89 => 89u16,
            Self::Rb90 => 90u16,
            Self::Rb91 => 91u16,
            Self::Rb92 => 92u16,
            Self::Rb93 => 93u16,
            Self::Rb94 => 94u16,
            Self::Rb95 => 95u16,
            Self::Rb96 => 96u16,
            Self::Rb97 => 97u16,
            Self::Rb98 => 98u16,
            Self::Rb99 => 99u16,
            Self::Rb100 => 100u16,
            Self::Rb101 => 101u16,
            Self::Rb102 => 102u16,
            Self::Rb103 => 103u16,
        }
    }
}
impl super::IsotopicComposition for RubidiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Rb85 => Some(0.7217f64),
            Self::Rb87 => Some(0.2783f64),
            Self::Rb71
            | Self::Rb72
            | Self::Rb73
            | Self::Rb74
            | Self::Rb75
            | Self::Rb76
            | Self::Rb77
            | Self::Rb78
            | Self::Rb79
            | Self::Rb80
            | Self::Rb81
            | Self::Rb82
            | Self::Rb83
            | Self::Rb84
            | Self::Rb86
            | Self::Rb88
            | Self::Rb89
            | Self::Rb90
            | Self::Rb91
            | Self::Rb92
            | Self::Rb93
            | Self::Rb94
            | Self::Rb95
            | Self::Rb96
            | Self::Rb97
            | Self::Rb98
            | Self::Rb99
            | Self::Rb100
            | Self::Rb101
            | Self::Rb102
            | Self::Rb103 => None,
        }
    }
}
impl super::MostAbundantIsotope for RubidiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rb85
    }
}
impl From<RubidiumIsotope> for crate::Isotope {
    fn from(isotope: RubidiumIsotope) -> Self {
        crate::Isotope::Rb(isotope)
    }
}
impl From<RubidiumIsotope> for crate::Element {
    fn from(_isotope: RubidiumIsotope) -> Self {
        crate::Element::Rb
    }
}
impl TryFrom<u16> for RubidiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            71u16 => Ok(Self::Rb71),
            72u16 => Ok(Self::Rb72),
            73u16 => Ok(Self::Rb73),
            74u16 => Ok(Self::Rb74),
            75u16 => Ok(Self::Rb75),
            76u16 => Ok(Self::Rb76),
            77u16 => Ok(Self::Rb77),
            78u16 => Ok(Self::Rb78),
            79u16 => Ok(Self::Rb79),
            80u16 => Ok(Self::Rb80),
            81u16 => Ok(Self::Rb81),
            82u16 => Ok(Self::Rb82),
            83u16 => Ok(Self::Rb83),
            84u16 => Ok(Self::Rb84),
            85u16 => Ok(Self::Rb85),
            86u16 => Ok(Self::Rb86),
            87u16 => Ok(Self::Rb87),
            88u16 => Ok(Self::Rb88),
            89u16 => Ok(Self::Rb89),
            90u16 => Ok(Self::Rb90),
            91u16 => Ok(Self::Rb91),
            92u16 => Ok(Self::Rb92),
            93u16 => Ok(Self::Rb93),
            94u16 => Ok(Self::Rb94),
            95u16 => Ok(Self::Rb95),
            96u16 => Ok(Self::Rb96),
            97u16 => Ok(Self::Rb97),
            98u16 => Ok(Self::Rb98),
            99u16 => Ok(Self::Rb99),
            100u16 => Ok(Self::Rb100),
            101u16 => Ok(Self::Rb101),
            102u16 => Ok(Self::Rb102),
            103u16 => Ok(Self::Rb103),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rb, value)),
        }
    }
}
impl std::fmt::Display for RubidiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rb71 => write!(f, "Rb71"),
            Self::Rb72 => write!(f, "Rb72"),
            Self::Rb73 => write!(f, "Rb73"),
            Self::Rb74 => write!(f, "Rb74"),
            Self::Rb75 => write!(f, "Rb75"),
            Self::Rb76 => write!(f, "Rb76"),
            Self::Rb77 => write!(f, "Rb77"),
            Self::Rb78 => write!(f, "Rb78"),
            Self::Rb79 => write!(f, "Rb79"),
            Self::Rb80 => write!(f, "Rb80"),
            Self::Rb81 => write!(f, "Rb81"),
            Self::Rb82 => write!(f, "Rb82"),
            Self::Rb83 => write!(f, "Rb83"),
            Self::Rb84 => write!(f, "Rb84"),
            Self::Rb85 => write!(f, "Rb85"),
            Self::Rb86 => write!(f, "Rb86"),
            Self::Rb87 => write!(f, "Rb87"),
            Self::Rb88 => write!(f, "Rb88"),
            Self::Rb89 => write!(f, "Rb89"),
            Self::Rb90 => write!(f, "Rb90"),
            Self::Rb91 => write!(f, "Rb91"),
            Self::Rb92 => write!(f, "Rb92"),
            Self::Rb93 => write!(f, "Rb93"),
            Self::Rb94 => write!(f, "Rb94"),
            Self::Rb95 => write!(f, "Rb95"),
            Self::Rb96 => write!(f, "Rb96"),
            Self::Rb97 => write!(f, "Rb97"),
            Self::Rb98 => write!(f, "Rb98"),
            Self::Rb99 => write!(f, "Rb99"),
            Self::Rb100 => write!(f, "Rb100"),
            Self::Rb101 => write!(f, "Rb101"),
            Self::Rb102 => write!(f, "Rb102"),
            Self::Rb103 => write!(f, "Rb103"),
        }
    }
}
