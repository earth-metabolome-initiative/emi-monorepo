//! Isotopes of the element Krypton
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Krypton
pub enum KryptonIsotope {
    /// Isotope Kr69 of Krypton
    Kr69,
    /// Isotope Kr70 of Krypton
    Kr70,
    /// Isotope Kr71 of Krypton
    Kr71,
    /// Isotope Kr72 of Krypton
    Kr72,
    /// Isotope Kr73 of Krypton
    Kr73,
    /// Isotope Kr74 of Krypton
    Kr74,
    /// Isotope Kr75 of Krypton
    Kr75,
    /// Isotope Kr76 of Krypton
    Kr76,
    /// Isotope Kr77 of Krypton
    Kr77,
    /// Isotope Kr78 of Krypton
    Kr78,
    /// Isotope Kr79 of Krypton
    Kr79,
    /// Isotope Kr80 of Krypton
    Kr80,
    /// Isotope Kr81 of Krypton
    Kr81,
    /// Isotope Kr82 of Krypton
    Kr82,
    /// Isotope Kr83 of Krypton
    Kr83,
    /// Isotope Kr84 of Krypton
    Kr84,
    /// Isotope Kr85 of Krypton
    Kr85,
    /// Isotope Kr86 of Krypton
    Kr86,
    /// Isotope Kr87 of Krypton
    Kr87,
    /// Isotope Kr88 of Krypton
    Kr88,
    /// Isotope Kr89 of Krypton
    Kr89,
    /// Isotope Kr90 of Krypton
    Kr90,
    /// Isotope Kr91 of Krypton
    Kr91,
    /// Isotope Kr92 of Krypton
    Kr92,
    /// Isotope Kr93 of Krypton
    Kr93,
    /// Isotope Kr94 of Krypton
    Kr94,
    /// Isotope Kr95 of Krypton
    Kr95,
    /// Isotope Kr96 of Krypton
    Kr96,
    /// Isotope Kr97 of Krypton
    Kr97,
    /// Isotope Kr98 of Krypton
    Kr98,
    /// Isotope Kr99 of Krypton
    Kr99,
    /// Isotope Kr100 of Krypton
    Kr100,
    /// Isotope Kr101 of Krypton
    Kr101,
}
impl super::RelativeAtomicMass for KryptonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Kr69 => 68.96518f64,
            Self::Kr70 => 69.95604f64,
            Self::Kr71 => 70.95027f64,
            Self::Kr72 => 71.9420924f64,
            Self::Kr73 => 72.9392892f64,
            Self::Kr74 => 73.933084f64,
            Self::Kr75 => 74.9309457f64,
            Self::Kr76 => 75.9259103f64,
            Self::Kr77 => 76.92467f64,
            Self::Kr78 => 77.92036494f64,
            Self::Kr79 => 78.9200829f64,
            Self::Kr80 => 79.91637808f64,
            Self::Kr81 => 80.9165912f64,
            Self::Kr82 => 81.91348273f64,
            Self::Kr83 => 82.91412716f64,
            Self::Kr84 => 83.9114977282f64,
            Self::Kr85 => 84.9125273f64,
            Self::Kr86 => 85.9106106269f64,
            Self::Kr87 => 86.91335476f64,
            Self::Kr88 => 87.9144479f64,
            Self::Kr89 => 88.9178355f64,
            Self::Kr90 => 89.9195279f64,
            Self::Kr91 => 90.9238063f64,
            Self::Kr92 => 91.9261731f64,
            Self::Kr93 => 92.9311472f64,
            Self::Kr94 => 93.93414f64,
            Self::Kr95 => 94.939711f64,
            Self::Kr96 => 95.943017f64,
            Self::Kr97 => 96.94909f64,
            Self::Kr98 => 97.95243f64,
            Self::Kr99 => 98.95839f64,
            Self::Kr100 => 99.96237f64,
            Self::Kr101 => 100.96873f64,
        }
    }
}
impl super::ElementVariant for KryptonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Kr
    }
}
impl super::MassNumber for KryptonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Kr69 => 69u16,
            Self::Kr70 => 70u16,
            Self::Kr71 => 71u16,
            Self::Kr72 => 72u16,
            Self::Kr73 => 73u16,
            Self::Kr74 => 74u16,
            Self::Kr75 => 75u16,
            Self::Kr76 => 76u16,
            Self::Kr77 => 77u16,
            Self::Kr78 => 78u16,
            Self::Kr79 => 79u16,
            Self::Kr80 => 80u16,
            Self::Kr81 => 81u16,
            Self::Kr82 => 82u16,
            Self::Kr83 => 83u16,
            Self::Kr84 => 84u16,
            Self::Kr85 => 85u16,
            Self::Kr86 => 86u16,
            Self::Kr87 => 87u16,
            Self::Kr88 => 88u16,
            Self::Kr89 => 89u16,
            Self::Kr90 => 90u16,
            Self::Kr91 => 91u16,
            Self::Kr92 => 92u16,
            Self::Kr93 => 93u16,
            Self::Kr94 => 94u16,
            Self::Kr95 => 95u16,
            Self::Kr96 => 96u16,
            Self::Kr97 => 97u16,
            Self::Kr98 => 98u16,
            Self::Kr99 => 99u16,
            Self::Kr100 => 100u16,
            Self::Kr101 => 101u16,
        }
    }
}
impl super::IsotopicComposition for KryptonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Kr78 => Some(0.00355f64),
            Self::Kr80 => Some(0.02286f64),
            Self::Kr82 => Some(0.11593f64),
            Self::Kr83 => Some(0.115f64),
            Self::Kr84 => Some(0.56987f64),
            Self::Kr86 => Some(0.17279f64),
            Self::Kr69
            | Self::Kr70
            | Self::Kr71
            | Self::Kr72
            | Self::Kr73
            | Self::Kr74
            | Self::Kr75
            | Self::Kr76
            | Self::Kr77
            | Self::Kr79
            | Self::Kr81
            | Self::Kr85
            | Self::Kr87
            | Self::Kr88
            | Self::Kr89
            | Self::Kr90
            | Self::Kr91
            | Self::Kr92
            | Self::Kr93
            | Self::Kr94
            | Self::Kr95
            | Self::Kr96
            | Self::Kr97
            | Self::Kr98
            | Self::Kr99
            | Self::Kr100
            | Self::Kr101 => None,
        }
    }
}
impl super::MostAbundantIsotope for KryptonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Kr84
    }
}
impl TryFrom<u16> for KryptonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            69u16 => Ok(Self::Kr69),
            70u16 => Ok(Self::Kr70),
            71u16 => Ok(Self::Kr71),
            72u16 => Ok(Self::Kr72),
            73u16 => Ok(Self::Kr73),
            74u16 => Ok(Self::Kr74),
            75u16 => Ok(Self::Kr75),
            76u16 => Ok(Self::Kr76),
            77u16 => Ok(Self::Kr77),
            78u16 => Ok(Self::Kr78),
            79u16 => Ok(Self::Kr79),
            80u16 => Ok(Self::Kr80),
            81u16 => Ok(Self::Kr81),
            82u16 => Ok(Self::Kr82),
            83u16 => Ok(Self::Kr83),
            84u16 => Ok(Self::Kr84),
            85u16 => Ok(Self::Kr85),
            86u16 => Ok(Self::Kr86),
            87u16 => Ok(Self::Kr87),
            88u16 => Ok(Self::Kr88),
            89u16 => Ok(Self::Kr89),
            90u16 => Ok(Self::Kr90),
            91u16 => Ok(Self::Kr91),
            92u16 => Ok(Self::Kr92),
            93u16 => Ok(Self::Kr93),
            94u16 => Ok(Self::Kr94),
            95u16 => Ok(Self::Kr95),
            96u16 => Ok(Self::Kr96),
            97u16 => Ok(Self::Kr97),
            98u16 => Ok(Self::Kr98),
            99u16 => Ok(Self::Kr99),
            100u16 => Ok(Self::Kr100),
            101u16 => Ok(Self::Kr101),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Kr, value)),
        }
    }
}
impl std::fmt::Display for KryptonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kr69 => write!(f, "Kr69"),
            Self::Kr70 => write!(f, "Kr70"),
            Self::Kr71 => write!(f, "Kr71"),
            Self::Kr72 => write!(f, "Kr72"),
            Self::Kr73 => write!(f, "Kr73"),
            Self::Kr74 => write!(f, "Kr74"),
            Self::Kr75 => write!(f, "Kr75"),
            Self::Kr76 => write!(f, "Kr76"),
            Self::Kr77 => write!(f, "Kr77"),
            Self::Kr78 => write!(f, "Kr78"),
            Self::Kr79 => write!(f, "Kr79"),
            Self::Kr80 => write!(f, "Kr80"),
            Self::Kr81 => write!(f, "Kr81"),
            Self::Kr82 => write!(f, "Kr82"),
            Self::Kr83 => write!(f, "Kr83"),
            Self::Kr84 => write!(f, "Kr84"),
            Self::Kr85 => write!(f, "Kr85"),
            Self::Kr86 => write!(f, "Kr86"),
            Self::Kr87 => write!(f, "Kr87"),
            Self::Kr88 => write!(f, "Kr88"),
            Self::Kr89 => write!(f, "Kr89"),
            Self::Kr90 => write!(f, "Kr90"),
            Self::Kr91 => write!(f, "Kr91"),
            Self::Kr92 => write!(f, "Kr92"),
            Self::Kr93 => write!(f, "Kr93"),
            Self::Kr94 => write!(f, "Kr94"),
            Self::Kr95 => write!(f, "Kr95"),
            Self::Kr96 => write!(f, "Kr96"),
            Self::Kr97 => write!(f, "Kr97"),
            Self::Kr98 => write!(f, "Kr98"),
            Self::Kr99 => write!(f, "Kr99"),
            Self::Kr100 => write!(f, "Kr100"),
            Self::Kr101 => write!(f, "Kr101"),
        }
    }
}
