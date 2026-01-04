//! Isotopes of the element Copernicium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Copernicium
pub enum CoperniciumIsotope {
    /// Isotope Cn276 of Copernicium
    Cn276,
    /// Isotope Cn277 of Copernicium
    Cn277,
    /// Isotope Cn278 of Copernicium
    Cn278,
    /// Isotope Cn279 of Copernicium
    Cn279,
    /// Isotope Cn280 of Copernicium
    Cn280,
    /// Isotope Cn281 of Copernicium
    Cn281,
    /// Isotope Cn282 of Copernicium
    Cn282,
    /// Isotope Cn283 of Copernicium
    Cn283,
    /// Isotope Cn284 of Copernicium
    Cn284,
    /// Isotope Cn285 of Copernicium
    Cn285,
}
impl super::RelativeAtomicMass for CoperniciumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cn276 => 276.16141f64,
            Self::Cn277 => 277.16364f64,
            Self::Cn278 => 278.16416f64,
            Self::Cn279 => 279.16654f64,
            Self::Cn280 => 280.16715f64,
            Self::Cn281 => 281.16975f64,
            Self::Cn282 => 282.1705f64,
            Self::Cn283 => 283.17327f64,
            Self::Cn284 => 284.17416f64,
            Self::Cn285 => 285.17712f64,
        }
    }
}
impl super::ElementVariant for CoperniciumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cn
    }
}
impl super::MassNumber for CoperniciumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cn276 => 276u16,
            Self::Cn277 => 277u16,
            Self::Cn278 => 278u16,
            Self::Cn279 => 279u16,
            Self::Cn280 => 280u16,
            Self::Cn281 => 281u16,
            Self::Cn282 => 282u16,
            Self::Cn283 => 283u16,
            Self::Cn284 => 284u16,
            Self::Cn285 => 285u16,
        }
    }
}
impl super::IsotopicComposition for CoperniciumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for CoperniciumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cn285
    }
}
impl From<CoperniciumIsotope> for crate::Isotope {
    fn from(isotope: CoperniciumIsotope) -> Self {
        crate::Isotope::Cn(isotope)
    }
}
impl From<CoperniciumIsotope> for crate::Element {
    fn from(_isotope: CoperniciumIsotope) -> Self {
        crate::Element::Cn
    }
}
impl TryFrom<u16> for CoperniciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            276u16 => Ok(Self::Cn276),
            277u16 => Ok(Self::Cn277),
            278u16 => Ok(Self::Cn278),
            279u16 => Ok(Self::Cn279),
            280u16 => Ok(Self::Cn280),
            281u16 => Ok(Self::Cn281),
            282u16 => Ok(Self::Cn282),
            283u16 => Ok(Self::Cn283),
            284u16 => Ok(Self::Cn284),
            285u16 => Ok(Self::Cn285),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cn, value)),
        }
    }
}
impl std::fmt::Display for CoperniciumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cn276 => write!(f, "Cn276"),
            Self::Cn277 => write!(f, "Cn277"),
            Self::Cn278 => write!(f, "Cn278"),
            Self::Cn279 => write!(f, "Cn279"),
            Self::Cn280 => write!(f, "Cn280"),
            Self::Cn281 => write!(f, "Cn281"),
            Self::Cn282 => write!(f, "Cn282"),
            Self::Cn283 => write!(f, "Cn283"),
            Self::Cn284 => write!(f, "Cn284"),
            Self::Cn285 => write!(f, "Cn285"),
        }
    }
}
