//! Isotopes of the element Hydrogen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Hydrogen
pub enum HydrogenIsotope {
    /// Isotope H1 of Hydrogen
    H1,
    /// Isotope D2 of Hydrogen
    D2,
    /// Isotope T3 of Hydrogen
    T3,
    /// Isotope H4 of Hydrogen
    H4,
    /// Isotope H5 of Hydrogen
    H5,
    /// Isotope H6 of Hydrogen
    H6,
    /// Isotope H7 of Hydrogen
    H7,
}
impl super::RelativeAtomicMass for HydrogenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::H1 => 1.00782503223f64,
            Self::D2 => 2.01410177812f64,
            Self::T3 => 3.0160492779f64,
            Self::H4 => 4.02643f64,
            Self::H5 => 5.035311f64,
            Self::H6 => 6.04496f64,
            Self::H7 => 7.0527f64,
        }
    }
}
impl super::ElementVariant for HydrogenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::H
    }
}
impl super::MassNumber for HydrogenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::H1 => 1u16,
            Self::D2 => 2u16,
            Self::T3 => 3u16,
            Self::H4 => 4u16,
            Self::H5 => 5u16,
            Self::H6 => 6u16,
            Self::H7 => 7u16,
        }
    }
}
impl super::IsotopicComposition for HydrogenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::H1 => Some(0.999885f64),
            Self::D2 => Some(0.000115f64),
            Self::T3 | Self::H4 | Self::H5 | Self::H6 | Self::H7 => None,
        }
    }
}
impl super::MostAbundantIsotope for HydrogenIsotope {
    fn most_abundant_isotope() -> Self {
        Self::H1
    }
}
impl From<HydrogenIsotope> for crate::Isotope {
    fn from(isotope: HydrogenIsotope) -> Self {
        crate::Isotope::H(isotope)
    }
}
impl From<HydrogenIsotope> for crate::Element {
    fn from(_isotope: HydrogenIsotope) -> Self {
        crate::Element::H
    }
}
impl TryFrom<u16> for HydrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1u16 => Ok(Self::H1),
            2u16 => Ok(Self::D2),
            3u16 => Ok(Self::T3),
            4u16 => Ok(Self::H4),
            5u16 => Ok(Self::H5),
            6u16 => Ok(Self::H6),
            7u16 => Ok(Self::H7),
            _ => Err(crate::errors::Error::Isotope(crate::Element::H, value)),
        }
    }
}
impl std::fmt::Display for HydrogenIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::H1 => write!(f, "H1"),
            Self::D2 => write!(f, "D2"),
            Self::T3 => write!(f, "T3"),
            Self::H4 => write!(f, "H4"),
            Self::H5 => write!(f, "H5"),
            Self::H6 => write!(f, "H6"),
            Self::H7 => write!(f, "H7"),
        }
    }
}
