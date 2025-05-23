//! Isotopes of the element Gallium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Gallium
pub enum GalliumIsotope {
    /// Isotope Ga56 of Gallium
    Ga56,
    /// Isotope Ga57 of Gallium
    Ga57,
    /// Isotope Ga58 of Gallium
    Ga58,
    /// Isotope Ga59 of Gallium
    Ga59,
    /// Isotope Ga60 of Gallium
    Ga60,
    /// Isotope Ga61 of Gallium
    Ga61,
    /// Isotope Ga62 of Gallium
    Ga62,
    /// Isotope Ga63 of Gallium
    Ga63,
    /// Isotope Ga64 of Gallium
    Ga64,
    /// Isotope Ga65 of Gallium
    Ga65,
    /// Isotope Ga66 of Gallium
    Ga66,
    /// Isotope Ga67 of Gallium
    Ga67,
    /// Isotope Ga68 of Gallium
    Ga68,
    /// Isotope Ga69 of Gallium
    Ga69,
    /// Isotope Ga70 of Gallium
    Ga70,
    /// Isotope Ga71 of Gallium
    Ga71,
    /// Isotope Ga72 of Gallium
    Ga72,
    /// Isotope Ga73 of Gallium
    Ga73,
    /// Isotope Ga74 of Gallium
    Ga74,
    /// Isotope Ga75 of Gallium
    Ga75,
    /// Isotope Ga76 of Gallium
    Ga76,
    /// Isotope Ga77 of Gallium
    Ga77,
    /// Isotope Ga78 of Gallium
    Ga78,
    /// Isotope Ga79 of Gallium
    Ga79,
    /// Isotope Ga80 of Gallium
    Ga80,
    /// Isotope Ga81 of Gallium
    Ga81,
    /// Isotope Ga82 of Gallium
    Ga82,
    /// Isotope Ga83 of Gallium
    Ga83,
    /// Isotope Ga84 of Gallium
    Ga84,
    /// Isotope Ga85 of Gallium
    Ga85,
    /// Isotope Ga86 of Gallium
    Ga86,
    /// Isotope Ga87 of Gallium
    Ga87,
}
impl super::RelativeAtomicMass for GalliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ga56 => 55.99536f64,
            Self::Ga57 => 56.9832f64,
            Self::Ga58 => 57.97478f64,
            Self::Ga59 => 58.96353f64,
            Self::Ga60 => 59.95729f64,
            Self::Ga61 => 60.949399f64,
            Self::Ga62 => 61.94419025f64,
            Self::Ga63 => 62.9392942f64,
            Self::Ga64 => 63.9368404f64,
            Self::Ga65 => 64.93273459f64,
            Self::Ga66 => 65.9315894f64,
            Self::Ga67 => 66.9282025f64,
            Self::Ga68 => 67.9279805f64,
            Self::Ga69 => 68.9255735f64,
            Self::Ga70 => 69.9260219f64,
            Self::Ga71 => 70.92470258f64,
            Self::Ga72 => 71.92636747f64,
            Self::Ga73 => 72.9251747f64,
            Self::Ga74 => 73.9269457f64,
            Self::Ga75 => 74.9265002f64,
            Self::Ga76 => 75.9288276f64,
            Self::Ga77 => 76.9291543f64,
            Self::Ga78 => 77.9316088f64,
            Self::Ga79 => 78.9328523f64,
            Self::Ga80 => 79.9364208f64,
            Self::Ga81 => 80.9381338f64,
            Self::Ga82 => 81.9431765f64,
            Self::Ga83 => 82.9471203f64,
            Self::Ga84 => 83.95246f64,
            Self::Ga85 => 84.95699f64,
            Self::Ga86 => 85.96301f64,
            Self::Ga87 => 86.96824f64,
        }
    }
}
impl super::ElementVariant for GalliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ga
    }
}
impl super::MassNumber for GalliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ga56 => 56u16,
            Self::Ga57 => 57u16,
            Self::Ga58 => 58u16,
            Self::Ga59 => 59u16,
            Self::Ga60 => 60u16,
            Self::Ga61 => 61u16,
            Self::Ga62 => 62u16,
            Self::Ga63 => 63u16,
            Self::Ga64 => 64u16,
            Self::Ga65 => 65u16,
            Self::Ga66 => 66u16,
            Self::Ga67 => 67u16,
            Self::Ga68 => 68u16,
            Self::Ga69 => 69u16,
            Self::Ga70 => 70u16,
            Self::Ga71 => 71u16,
            Self::Ga72 => 72u16,
            Self::Ga73 => 73u16,
            Self::Ga74 => 74u16,
            Self::Ga75 => 75u16,
            Self::Ga76 => 76u16,
            Self::Ga77 => 77u16,
            Self::Ga78 => 78u16,
            Self::Ga79 => 79u16,
            Self::Ga80 => 80u16,
            Self::Ga81 => 81u16,
            Self::Ga82 => 82u16,
            Self::Ga83 => 83u16,
            Self::Ga84 => 84u16,
            Self::Ga85 => 85u16,
            Self::Ga86 => 86u16,
            Self::Ga87 => 87u16,
        }
    }
}
impl super::IsotopicComposition for GalliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ga69 => Some(0.60108f64),
            Self::Ga71 => Some(0.39892f64),
            Self::Ga56
            | Self::Ga57
            | Self::Ga58
            | Self::Ga59
            | Self::Ga60
            | Self::Ga61
            | Self::Ga62
            | Self::Ga63
            | Self::Ga64
            | Self::Ga65
            | Self::Ga66
            | Self::Ga67
            | Self::Ga68
            | Self::Ga70
            | Self::Ga72
            | Self::Ga73
            | Self::Ga74
            | Self::Ga75
            | Self::Ga76
            | Self::Ga77
            | Self::Ga78
            | Self::Ga79
            | Self::Ga80
            | Self::Ga81
            | Self::Ga82
            | Self::Ga83
            | Self::Ga84
            | Self::Ga85
            | Self::Ga86
            | Self::Ga87 => None,
        }
    }
}
impl super::MostAbundantIsotope for GalliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ga69
    }
}
impl From<GalliumIsotope> for crate::Isotope {
    fn from(isotope: GalliumIsotope) -> Self {
        crate::Isotope::Ga(isotope)
    }
}
impl From<GalliumIsotope> for crate::Element {
    fn from(_isotope: GalliumIsotope) -> Self {
        crate::Element::Ga
    }
}
impl TryFrom<u16> for GalliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            56u16 => Ok(Self::Ga56),
            57u16 => Ok(Self::Ga57),
            58u16 => Ok(Self::Ga58),
            59u16 => Ok(Self::Ga59),
            60u16 => Ok(Self::Ga60),
            61u16 => Ok(Self::Ga61),
            62u16 => Ok(Self::Ga62),
            63u16 => Ok(Self::Ga63),
            64u16 => Ok(Self::Ga64),
            65u16 => Ok(Self::Ga65),
            66u16 => Ok(Self::Ga66),
            67u16 => Ok(Self::Ga67),
            68u16 => Ok(Self::Ga68),
            69u16 => Ok(Self::Ga69),
            70u16 => Ok(Self::Ga70),
            71u16 => Ok(Self::Ga71),
            72u16 => Ok(Self::Ga72),
            73u16 => Ok(Self::Ga73),
            74u16 => Ok(Self::Ga74),
            75u16 => Ok(Self::Ga75),
            76u16 => Ok(Self::Ga76),
            77u16 => Ok(Self::Ga77),
            78u16 => Ok(Self::Ga78),
            79u16 => Ok(Self::Ga79),
            80u16 => Ok(Self::Ga80),
            81u16 => Ok(Self::Ga81),
            82u16 => Ok(Self::Ga82),
            83u16 => Ok(Self::Ga83),
            84u16 => Ok(Self::Ga84),
            85u16 => Ok(Self::Ga85),
            86u16 => Ok(Self::Ga86),
            87u16 => Ok(Self::Ga87),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ga, value)),
        }
    }
}
impl std::fmt::Display for GalliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ga56 => write!(f, "Ga56"),
            Self::Ga57 => write!(f, "Ga57"),
            Self::Ga58 => write!(f, "Ga58"),
            Self::Ga59 => write!(f, "Ga59"),
            Self::Ga60 => write!(f, "Ga60"),
            Self::Ga61 => write!(f, "Ga61"),
            Self::Ga62 => write!(f, "Ga62"),
            Self::Ga63 => write!(f, "Ga63"),
            Self::Ga64 => write!(f, "Ga64"),
            Self::Ga65 => write!(f, "Ga65"),
            Self::Ga66 => write!(f, "Ga66"),
            Self::Ga67 => write!(f, "Ga67"),
            Self::Ga68 => write!(f, "Ga68"),
            Self::Ga69 => write!(f, "Ga69"),
            Self::Ga70 => write!(f, "Ga70"),
            Self::Ga71 => write!(f, "Ga71"),
            Self::Ga72 => write!(f, "Ga72"),
            Self::Ga73 => write!(f, "Ga73"),
            Self::Ga74 => write!(f, "Ga74"),
            Self::Ga75 => write!(f, "Ga75"),
            Self::Ga76 => write!(f, "Ga76"),
            Self::Ga77 => write!(f, "Ga77"),
            Self::Ga78 => write!(f, "Ga78"),
            Self::Ga79 => write!(f, "Ga79"),
            Self::Ga80 => write!(f, "Ga80"),
            Self::Ga81 => write!(f, "Ga81"),
            Self::Ga82 => write!(f, "Ga82"),
            Self::Ga83 => write!(f, "Ga83"),
            Self::Ga84 => write!(f, "Ga84"),
            Self::Ga85 => write!(f, "Ga85"),
            Self::Ga86 => write!(f, "Ga86"),
            Self::Ga87 => write!(f, "Ga87"),
        }
    }
}
