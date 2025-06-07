//! Isotopes of the element Carbon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Carbon
pub enum CarbonIsotope {
    /// Isotope C8 of Carbon
    C8,
    /// Isotope C9 of Carbon
    C9,
    /// Isotope C10 of Carbon
    C10,
    /// Isotope C11 of Carbon
    C11,
    /// Isotope C12 of Carbon
    C12,
    /// Isotope C13 of Carbon
    C13,
    /// Isotope C14 of Carbon
    C14,
    /// Isotope C15 of Carbon
    C15,
    /// Isotope C16 of Carbon
    C16,
    /// Isotope C17 of Carbon
    C17,
    /// Isotope C18 of Carbon
    C18,
    /// Isotope C19 of Carbon
    C19,
    /// Isotope C20 of Carbon
    C20,
    /// Isotope C21 of Carbon
    C21,
    /// Isotope C22 of Carbon
    C22,
    /// Isotope C23 of Carbon
    C23,
}
impl super::RelativeAtomicMass for CarbonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::C8 => 8.037643f64,
            Self::C9 => 9.0310372f64,
            Self::C10 => 10.01685331f64,
            Self::C11 => 11.0114336f64,
            Self::C12 => 12f64,
            Self::C13 => 13.00335483507f64,
            Self::C14 => 14.0032419884f64,
            Self::C15 => 15.01059926f64,
            Self::C16 => 16.0147013f64,
            Self::C17 => 17.022577f64,
            Self::C18 => 18.026751f64,
            Self::C19 => 19.0348f64,
            Self::C20 => 20.04032f64,
            Self::C21 => 21.049f64,
            Self::C22 => 22.05753f64,
            Self::C23 => 23.0689f64,
        }
    }
}
impl super::ElementVariant for CarbonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::C
    }
}
impl super::MassNumber for CarbonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::C8 => 8u16,
            Self::C9 => 9u16,
            Self::C10 => 10u16,
            Self::C11 => 11u16,
            Self::C12 => 12u16,
            Self::C13 => 13u16,
            Self::C14 => 14u16,
            Self::C15 => 15u16,
            Self::C16 => 16u16,
            Self::C17 => 17u16,
            Self::C18 => 18u16,
            Self::C19 => 19u16,
            Self::C20 => 20u16,
            Self::C21 => 21u16,
            Self::C22 => 22u16,
            Self::C23 => 23u16,
        }
    }
}
impl super::IsotopicComposition for CarbonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::C12 => Some(0.9893f64),
            Self::C13 => Some(0.0107f64),
            Self::C8
            | Self::C9
            | Self::C10
            | Self::C11
            | Self::C14
            | Self::C15
            | Self::C16
            | Self::C17
            | Self::C18
            | Self::C19
            | Self::C20
            | Self::C21
            | Self::C22
            | Self::C23 => None,
        }
    }
}
impl super::MostAbundantIsotope for CarbonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::C12
    }
}
impl From<CarbonIsotope> for crate::Isotope {
    fn from(isotope: CarbonIsotope) -> Self {
        crate::Isotope::C(isotope)
    }
}
impl From<CarbonIsotope> for crate::Element {
    fn from(_isotope: CarbonIsotope) -> Self {
        crate::Element::C
    }
}
impl TryFrom<u16> for CarbonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            8u16 => Ok(Self::C8),
            9u16 => Ok(Self::C9),
            10u16 => Ok(Self::C10),
            11u16 => Ok(Self::C11),
            12u16 => Ok(Self::C12),
            13u16 => Ok(Self::C13),
            14u16 => Ok(Self::C14),
            15u16 => Ok(Self::C15),
            16u16 => Ok(Self::C16),
            17u16 => Ok(Self::C17),
            18u16 => Ok(Self::C18),
            19u16 => Ok(Self::C19),
            20u16 => Ok(Self::C20),
            21u16 => Ok(Self::C21),
            22u16 => Ok(Self::C22),
            23u16 => Ok(Self::C23),
            _ => Err(crate::errors::Error::Isotope(crate::Element::C, value)),
        }
    }
}
impl std::fmt::Display for CarbonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C8 => write!(f, "C8"),
            Self::C9 => write!(f, "C9"),
            Self::C10 => write!(f, "C10"),
            Self::C11 => write!(f, "C11"),
            Self::C12 => write!(f, "C12"),
            Self::C13 => write!(f, "C13"),
            Self::C14 => write!(f, "C14"),
            Self::C15 => write!(f, "C15"),
            Self::C16 => write!(f, "C16"),
            Self::C17 => write!(f, "C17"),
            Self::C18 => write!(f, "C18"),
            Self::C19 => write!(f, "C19"),
            Self::C20 => write!(f, "C20"),
            Self::C21 => write!(f, "C21"),
            Self::C22 => write!(f, "C22"),
            Self::C23 => write!(f, "C23"),
        }
    }
}
