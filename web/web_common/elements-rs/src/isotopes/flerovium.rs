//! Isotopes of the element Flerovium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Flerovium
pub enum FleroviumIsotope {
    /// Isotope Fl285 of Flerovium
    Fl285,
    /// Isotope Fl286 of Flerovium
    Fl286,
    /// Isotope Fl287 of Flerovium
    Fl287,
    /// Isotope Fl288 of Flerovium
    Fl288,
    /// Isotope Fl289 of Flerovium
    Fl289,
}
impl super::RelativeAtomicMass for FleroviumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fl285 => 285.18364f64,
            Self::Fl286 => 286.18423f64,
            Self::Fl287 => 287.18678f64,
            Self::Fl288 => 288.18757f64,
            Self::Fl289 => 289.19042f64,
        }
    }
}
impl super::ElementVariant for FleroviumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fl
    }
}
impl super::MassNumber for FleroviumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fl285 => 285u16,
            Self::Fl286 => 286u16,
            Self::Fl287 => 287u16,
            Self::Fl288 => 288u16,
            Self::Fl289 => 289u16,
        }
    }
}
impl super::IsotopicComposition for FleroviumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for FleroviumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Fl289
    }
}
impl From<FleroviumIsotope> for crate::Isotope {
    fn from(isotope: FleroviumIsotope) -> Self {
        crate::Isotope::Fl(isotope)
    }
}
impl From<FleroviumIsotope> for crate::Element {
    fn from(_isotope: FleroviumIsotope) -> Self {
        crate::Element::Fl
    }
}
impl TryFrom<u16> for FleroviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            285u16 => Ok(Self::Fl285),
            286u16 => Ok(Self::Fl286),
            287u16 => Ok(Self::Fl287),
            288u16 => Ok(Self::Fl288),
            289u16 => Ok(Self::Fl289),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Fl, value)),
        }
    }
}
impl std::fmt::Display for FleroviumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fl285 => write!(f, "Fl285"),
            Self::Fl286 => write!(f, "Fl286"),
            Self::Fl287 => write!(f, "Fl287"),
            Self::Fl288 => write!(f, "Fl288"),
            Self::Fl289 => write!(f, "Fl289"),
        }
    }
}
