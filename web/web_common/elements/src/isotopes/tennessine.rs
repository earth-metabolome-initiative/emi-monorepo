//! Isotopes of the element Tennessine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Tennessine
pub enum TennessineIsotope {
    /// Isotope Ts291 of Tennessine
    Ts291,
    /// Isotope Ts292 of Tennessine
    Ts292,
    /// Isotope Ts293 of Tennessine
    Ts293,
    /// Isotope Ts294 of Tennessine
    Ts294,
}
impl super::RelativeAtomicMass for TennessineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ts291 => 291.20553f64,
            Self::Ts292 => 292.20746f64,
            Self::Ts293 => 293.20824f64,
            Self::Ts294 => 294.21046f64,
        }
    }
}
impl super::ElementVariant for TennessineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ts
    }
}
impl super::MassNumber for TennessineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ts291 => 291u16,
            Self::Ts292 => 292u16,
            Self::Ts293 => 293u16,
            Self::Ts294 => 294u16,
        }
    }
}
impl super::IsotopicComposition for TennessineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for TennessineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ts294
    }
}
impl TryFrom<u16> for TennessineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            291u16 => Ok(Self::Ts291),
            292u16 => Ok(Self::Ts292),
            293u16 => Ok(Self::Ts293),
            294u16 => Ok(Self::Ts294),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ts, value)),
        }
    }
}
impl std::fmt::Display for TennessineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ts291 => write!(f, "Ts291"),
            Self::Ts292 => write!(f, "Ts292"),
            Self::Ts293 => write!(f, "Ts293"),
            Self::Ts294 => write!(f, "Ts294"),
        }
    }
}
