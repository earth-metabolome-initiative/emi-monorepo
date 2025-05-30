//! Isotopes of the element Scandium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Scandium
pub enum ScandiumIsotope {
    /// Isotope Sc36 of Scandium
    Sc36,
    /// Isotope Sc37 of Scandium
    Sc37,
    /// Isotope Sc38 of Scandium
    Sc38,
    /// Isotope Sc39 of Scandium
    Sc39,
    /// Isotope Sc40 of Scandium
    Sc40,
    /// Isotope Sc41 of Scandium
    Sc41,
    /// Isotope Sc42 of Scandium
    Sc42,
    /// Isotope Sc43 of Scandium
    Sc43,
    /// Isotope Sc44 of Scandium
    Sc44,
    /// Isotope Sc45 of Scandium
    Sc45,
    /// Isotope Sc46 of Scandium
    Sc46,
    /// Isotope Sc47 of Scandium
    Sc47,
    /// Isotope Sc48 of Scandium
    Sc48,
    /// Isotope Sc49 of Scandium
    Sc49,
    /// Isotope Sc50 of Scandium
    Sc50,
    /// Isotope Sc51 of Scandium
    Sc51,
    /// Isotope Sc52 of Scandium
    Sc52,
    /// Isotope Sc53 of Scandium
    Sc53,
    /// Isotope Sc54 of Scandium
    Sc54,
    /// Isotope Sc55 of Scandium
    Sc55,
    /// Isotope Sc56 of Scandium
    Sc56,
    /// Isotope Sc57 of Scandium
    Sc57,
    /// Isotope Sc58 of Scandium
    Sc58,
    /// Isotope Sc59 of Scandium
    Sc59,
    /// Isotope Sc60 of Scandium
    Sc60,
    /// Isotope Sc61 of Scandium
    Sc61,
}
impl super::RelativeAtomicMass for ScandiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sc36 => 36.01648f64,
            Self::Sc37 => 37.00374f64,
            Self::Sc38 => 37.99512f64,
            Self::Sc39 => 38.984785f64,
            Self::Sc40 => 39.9779673f64,
            Self::Sc41 => 40.969251105f64,
            Self::Sc42 => 41.96551653f64,
            Self::Sc43 => 42.9611505f64,
            Self::Sc44 => 43.9594029f64,
            Self::Sc45 => 44.95590828f64,
            Self::Sc46 => 45.95516826f64,
            Self::Sc47 => 46.9524037f64,
            Self::Sc48 => 47.9522236f64,
            Self::Sc49 => 48.9500146f64,
            Self::Sc50 => 49.952176f64,
            Self::Sc51 => 50.953592f64,
            Self::Sc52 => 51.95688f64,
            Self::Sc53 => 52.95909f64,
            Self::Sc54 => 53.96393f64,
            Self::Sc55 => 54.96782f64,
            Self::Sc56 => 55.97345f64,
            Self::Sc57 => 56.97777f64,
            Self::Sc58 => 57.98403f64,
            Self::Sc59 => 58.98894f64,
            Self::Sc60 => 59.99565f64,
            Self::Sc61 => 61.001f64,
        }
    }
}
impl super::ElementVariant for ScandiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sc
    }
}
impl super::MassNumber for ScandiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sc36 => 36u16,
            Self::Sc37 => 37u16,
            Self::Sc38 => 38u16,
            Self::Sc39 => 39u16,
            Self::Sc40 => 40u16,
            Self::Sc41 => 41u16,
            Self::Sc42 => 42u16,
            Self::Sc43 => 43u16,
            Self::Sc44 => 44u16,
            Self::Sc45 => 45u16,
            Self::Sc46 => 46u16,
            Self::Sc47 => 47u16,
            Self::Sc48 => 48u16,
            Self::Sc49 => 49u16,
            Self::Sc50 => 50u16,
            Self::Sc51 => 51u16,
            Self::Sc52 => 52u16,
            Self::Sc53 => 53u16,
            Self::Sc54 => 54u16,
            Self::Sc55 => 55u16,
            Self::Sc56 => 56u16,
            Self::Sc57 => 57u16,
            Self::Sc58 => 58u16,
            Self::Sc59 => 59u16,
            Self::Sc60 => 60u16,
            Self::Sc61 => 61u16,
        }
    }
}
impl super::IsotopicComposition for ScandiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sc45 => Some(1f64),
            Self::Sc36
            | Self::Sc37
            | Self::Sc38
            | Self::Sc39
            | Self::Sc40
            | Self::Sc41
            | Self::Sc42
            | Self::Sc43
            | Self::Sc44
            | Self::Sc46
            | Self::Sc47
            | Self::Sc48
            | Self::Sc49
            | Self::Sc50
            | Self::Sc51
            | Self::Sc52
            | Self::Sc53
            | Self::Sc54
            | Self::Sc55
            | Self::Sc56
            | Self::Sc57
            | Self::Sc58
            | Self::Sc59
            | Self::Sc60
            | Self::Sc61 => None,
        }
    }
}
impl super::MostAbundantIsotope for ScandiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sc45
    }
}
impl From<ScandiumIsotope> for crate::Isotope {
    fn from(isotope: ScandiumIsotope) -> Self {
        crate::Isotope::Sc(isotope)
    }
}
impl From<ScandiumIsotope> for crate::Element {
    fn from(_isotope: ScandiumIsotope) -> Self {
        crate::Element::Sc
    }
}
impl TryFrom<u16> for ScandiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            36u16 => Ok(Self::Sc36),
            37u16 => Ok(Self::Sc37),
            38u16 => Ok(Self::Sc38),
            39u16 => Ok(Self::Sc39),
            40u16 => Ok(Self::Sc40),
            41u16 => Ok(Self::Sc41),
            42u16 => Ok(Self::Sc42),
            43u16 => Ok(Self::Sc43),
            44u16 => Ok(Self::Sc44),
            45u16 => Ok(Self::Sc45),
            46u16 => Ok(Self::Sc46),
            47u16 => Ok(Self::Sc47),
            48u16 => Ok(Self::Sc48),
            49u16 => Ok(Self::Sc49),
            50u16 => Ok(Self::Sc50),
            51u16 => Ok(Self::Sc51),
            52u16 => Ok(Self::Sc52),
            53u16 => Ok(Self::Sc53),
            54u16 => Ok(Self::Sc54),
            55u16 => Ok(Self::Sc55),
            56u16 => Ok(Self::Sc56),
            57u16 => Ok(Self::Sc57),
            58u16 => Ok(Self::Sc58),
            59u16 => Ok(Self::Sc59),
            60u16 => Ok(Self::Sc60),
            61u16 => Ok(Self::Sc61),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sc, value)),
        }
    }
}
impl std::fmt::Display for ScandiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sc36 => write!(f, "Sc36"),
            Self::Sc37 => write!(f, "Sc37"),
            Self::Sc38 => write!(f, "Sc38"),
            Self::Sc39 => write!(f, "Sc39"),
            Self::Sc40 => write!(f, "Sc40"),
            Self::Sc41 => write!(f, "Sc41"),
            Self::Sc42 => write!(f, "Sc42"),
            Self::Sc43 => write!(f, "Sc43"),
            Self::Sc44 => write!(f, "Sc44"),
            Self::Sc45 => write!(f, "Sc45"),
            Self::Sc46 => write!(f, "Sc46"),
            Self::Sc47 => write!(f, "Sc47"),
            Self::Sc48 => write!(f, "Sc48"),
            Self::Sc49 => write!(f, "Sc49"),
            Self::Sc50 => write!(f, "Sc50"),
            Self::Sc51 => write!(f, "Sc51"),
            Self::Sc52 => write!(f, "Sc52"),
            Self::Sc53 => write!(f, "Sc53"),
            Self::Sc54 => write!(f, "Sc54"),
            Self::Sc55 => write!(f, "Sc55"),
            Self::Sc56 => write!(f, "Sc56"),
            Self::Sc57 => write!(f, "Sc57"),
            Self::Sc58 => write!(f, "Sc58"),
            Self::Sc59 => write!(f, "Sc59"),
            Self::Sc60 => write!(f, "Sc60"),
            Self::Sc61 => write!(f, "Sc61"),
        }
    }
}
