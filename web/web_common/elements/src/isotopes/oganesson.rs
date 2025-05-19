//! Isotopes of the element Oganesson
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Oganesson
pub enum OganessonIsotope {
    /// Isotope Og293 of Oganesson
    Og293,
    /// Isotope Og294 of Oganesson
    Og294,
    /// Isotope Og295 of Oganesson
    Og295,
}
impl super::RelativeAtomicMass for OganessonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Og293 => 293.21356f64,
            Self::Og294 => 294.21392f64,
            Self::Og295 => 295.21624f64,
        }
    }
}
impl super::ElementVariant for OganessonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Og
    }
}
impl super::MassNumber for OganessonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Og293 => 293u16,
            Self::Og294 => 294u16,
            Self::Og295 => 295u16,
        }
    }
}
impl super::IsotopicComposition for OganessonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for OganessonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Og295
    }
}
impl TryFrom<u16> for OganessonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            293u16 => Ok(Self::Og293),
            294u16 => Ok(Self::Og294),
            295u16 => Ok(Self::Og295),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Og, value)),
        }
    }
}
impl std::fmt::Display for OganessonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Og293 => write!(f, "Og293"),
            Self::Og294 => write!(f, "Og294"),
            Self::Og295 => write!(f, "Og295"),
        }
    }
}
