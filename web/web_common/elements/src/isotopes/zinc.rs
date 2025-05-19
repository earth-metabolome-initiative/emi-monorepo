//! Isotopes of the element Zinc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Zinc
pub enum ZincIsotope {
    /// Isotope Zn54 of Zinc
    Zn54,
    /// Isotope Zn55 of Zinc
    Zn55,
    /// Isotope Zn56 of Zinc
    Zn56,
    /// Isotope Zn57 of Zinc
    Zn57,
    /// Isotope Zn58 of Zinc
    Zn58,
    /// Isotope Zn59 of Zinc
    Zn59,
    /// Isotope Zn60 of Zinc
    Zn60,
    /// Isotope Zn61 of Zinc
    Zn61,
    /// Isotope Zn62 of Zinc
    Zn62,
    /// Isotope Zn63 of Zinc
    Zn63,
    /// Isotope Zn64 of Zinc
    Zn64,
    /// Isotope Zn65 of Zinc
    Zn65,
    /// Isotope Zn66 of Zinc
    Zn66,
    /// Isotope Zn67 of Zinc
    Zn67,
    /// Isotope Zn68 of Zinc
    Zn68,
    /// Isotope Zn69 of Zinc
    Zn69,
    /// Isotope Zn70 of Zinc
    Zn70,
    /// Isotope Zn71 of Zinc
    Zn71,
    /// Isotope Zn72 of Zinc
    Zn72,
    /// Isotope Zn73 of Zinc
    Zn73,
    /// Isotope Zn74 of Zinc
    Zn74,
    /// Isotope Zn75 of Zinc
    Zn75,
    /// Isotope Zn76 of Zinc
    Zn76,
    /// Isotope Zn77 of Zinc
    Zn77,
    /// Isotope Zn78 of Zinc
    Zn78,
    /// Isotope Zn79 of Zinc
    Zn79,
    /// Isotope Zn80 of Zinc
    Zn80,
    /// Isotope Zn81 of Zinc
    Zn81,
    /// Isotope Zn82 of Zinc
    Zn82,
    /// Isotope Zn83 of Zinc
    Zn83,
    /// Isotope Zn84 of Zinc
    Zn84,
    /// Isotope Zn85 of Zinc
    Zn85,
}
impl super::RelativeAtomicMass for ZincIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Zn54 => 53.99204f64,
            Self::Zn55 => 54.98398f64,
            Self::Zn56 => 55.97254f64,
            Self::Zn57 => 56.96506f64,
            Self::Zn58 => 57.954591f64,
            Self::Zn59 => 58.94931266f64,
            Self::Zn60 => 59.9418421f64,
            Self::Zn61 => 60.939507f64,
            Self::Zn62 => 61.93433397f64,
            Self::Zn63 => 62.9332115f64,
            Self::Zn64 => 63.92914201f64,
            Self::Zn65 => 64.92924077f64,
            Self::Zn66 => 65.92603381f64,
            Self::Zn67 => 66.92712775f64,
            Self::Zn68 => 67.92484455f64,
            Self::Zn69 => 68.9265507f64,
            Self::Zn70 => 69.9253192f64,
            Self::Zn71 => 70.9277196f64,
            Self::Zn72 => 71.9268428f64,
            Self::Zn73 => 72.9295826f64,
            Self::Zn74 => 73.9294073f64,
            Self::Zn75 => 74.9328402f64,
            Self::Zn76 => 75.933115f64,
            Self::Zn77 => 76.9368872f64,
            Self::Zn78 => 77.9382892f64,
            Self::Zn79 => 78.9426381f64,
            Self::Zn80 => 79.9445529f64,
            Self::Zn81 => 80.9504026f64,
            Self::Zn82 => 81.95426f64,
            Self::Zn83 => 82.96056f64,
            Self::Zn84 => 83.96521f64,
            Self::Zn85 => 84.97226f64,
        }
    }
}
impl super::ElementVariant for ZincIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Zn
    }
}
impl super::MassNumber for ZincIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Zn54 => 54u16,
            Self::Zn55 => 55u16,
            Self::Zn56 => 56u16,
            Self::Zn57 => 57u16,
            Self::Zn58 => 58u16,
            Self::Zn59 => 59u16,
            Self::Zn60 => 60u16,
            Self::Zn61 => 61u16,
            Self::Zn62 => 62u16,
            Self::Zn63 => 63u16,
            Self::Zn64 => 64u16,
            Self::Zn65 => 65u16,
            Self::Zn66 => 66u16,
            Self::Zn67 => 67u16,
            Self::Zn68 => 68u16,
            Self::Zn69 => 69u16,
            Self::Zn70 => 70u16,
            Self::Zn71 => 71u16,
            Self::Zn72 => 72u16,
            Self::Zn73 => 73u16,
            Self::Zn74 => 74u16,
            Self::Zn75 => 75u16,
            Self::Zn76 => 76u16,
            Self::Zn77 => 77u16,
            Self::Zn78 => 78u16,
            Self::Zn79 => 79u16,
            Self::Zn80 => 80u16,
            Self::Zn81 => 81u16,
            Self::Zn82 => 82u16,
            Self::Zn83 => 83u16,
            Self::Zn84 => 84u16,
            Self::Zn85 => 85u16,
        }
    }
}
impl super::IsotopicComposition for ZincIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Zn64 => Some(0.4917f64),
            Self::Zn66 => Some(0.2773f64),
            Self::Zn67 => Some(0.0404f64),
            Self::Zn68 => Some(0.1845f64),
            Self::Zn70 => Some(0.0061f64),
            Self::Zn54
            | Self::Zn55
            | Self::Zn56
            | Self::Zn57
            | Self::Zn58
            | Self::Zn59
            | Self::Zn60
            | Self::Zn61
            | Self::Zn62
            | Self::Zn63
            | Self::Zn65
            | Self::Zn69
            | Self::Zn71
            | Self::Zn72
            | Self::Zn73
            | Self::Zn74
            | Self::Zn75
            | Self::Zn76
            | Self::Zn77
            | Self::Zn78
            | Self::Zn79
            | Self::Zn80
            | Self::Zn81
            | Self::Zn82
            | Self::Zn83
            | Self::Zn84
            | Self::Zn85 => None,
        }
    }
}
impl super::MostAbundantIsotope for ZincIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Zn64
    }
}
impl TryFrom<u16> for ZincIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            54u16 => Ok(Self::Zn54),
            55u16 => Ok(Self::Zn55),
            56u16 => Ok(Self::Zn56),
            57u16 => Ok(Self::Zn57),
            58u16 => Ok(Self::Zn58),
            59u16 => Ok(Self::Zn59),
            60u16 => Ok(Self::Zn60),
            61u16 => Ok(Self::Zn61),
            62u16 => Ok(Self::Zn62),
            63u16 => Ok(Self::Zn63),
            64u16 => Ok(Self::Zn64),
            65u16 => Ok(Self::Zn65),
            66u16 => Ok(Self::Zn66),
            67u16 => Ok(Self::Zn67),
            68u16 => Ok(Self::Zn68),
            69u16 => Ok(Self::Zn69),
            70u16 => Ok(Self::Zn70),
            71u16 => Ok(Self::Zn71),
            72u16 => Ok(Self::Zn72),
            73u16 => Ok(Self::Zn73),
            74u16 => Ok(Self::Zn74),
            75u16 => Ok(Self::Zn75),
            76u16 => Ok(Self::Zn76),
            77u16 => Ok(Self::Zn77),
            78u16 => Ok(Self::Zn78),
            79u16 => Ok(Self::Zn79),
            80u16 => Ok(Self::Zn80),
            81u16 => Ok(Self::Zn81),
            82u16 => Ok(Self::Zn82),
            83u16 => Ok(Self::Zn83),
            84u16 => Ok(Self::Zn84),
            85u16 => Ok(Self::Zn85),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Zn, value)),
        }
    }
}
impl std::fmt::Display for ZincIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zn54 => write!(f, "Zn54"),
            Self::Zn55 => write!(f, "Zn55"),
            Self::Zn56 => write!(f, "Zn56"),
            Self::Zn57 => write!(f, "Zn57"),
            Self::Zn58 => write!(f, "Zn58"),
            Self::Zn59 => write!(f, "Zn59"),
            Self::Zn60 => write!(f, "Zn60"),
            Self::Zn61 => write!(f, "Zn61"),
            Self::Zn62 => write!(f, "Zn62"),
            Self::Zn63 => write!(f, "Zn63"),
            Self::Zn64 => write!(f, "Zn64"),
            Self::Zn65 => write!(f, "Zn65"),
            Self::Zn66 => write!(f, "Zn66"),
            Self::Zn67 => write!(f, "Zn67"),
            Self::Zn68 => write!(f, "Zn68"),
            Self::Zn69 => write!(f, "Zn69"),
            Self::Zn70 => write!(f, "Zn70"),
            Self::Zn71 => write!(f, "Zn71"),
            Self::Zn72 => write!(f, "Zn72"),
            Self::Zn73 => write!(f, "Zn73"),
            Self::Zn74 => write!(f, "Zn74"),
            Self::Zn75 => write!(f, "Zn75"),
            Self::Zn76 => write!(f, "Zn76"),
            Self::Zn77 => write!(f, "Zn77"),
            Self::Zn78 => write!(f, "Zn78"),
            Self::Zn79 => write!(f, "Zn79"),
            Self::Zn80 => write!(f, "Zn80"),
            Self::Zn81 => write!(f, "Zn81"),
            Self::Zn82 => write!(f, "Zn82"),
            Self::Zn83 => write!(f, "Zn83"),
            Self::Zn84 => write!(f, "Zn84"),
            Self::Zn85 => write!(f, "Zn85"),
        }
    }
}
