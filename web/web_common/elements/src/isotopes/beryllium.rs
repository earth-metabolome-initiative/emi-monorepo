//! Isotopes of the element Beryllium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Beryllium
pub enum BerylliumIsotope {
    /// Isotope Be5 of Beryllium
    Be5,
    /// Isotope Be6 of Beryllium
    Be6,
    /// Isotope Be7 of Beryllium
    Be7,
    /// Isotope Be8 of Beryllium
    Be8,
    /// Isotope Be9 of Beryllium
    Be9,
    /// Isotope Be10 of Beryllium
    Be10,
    /// Isotope Be11 of Beryllium
    Be11,
    /// Isotope Be12 of Beryllium
    Be12,
    /// Isotope Be13 of Beryllium
    Be13,
    /// Isotope Be14 of Beryllium
    Be14,
    /// Isotope Be15 of Beryllium
    Be15,
    /// Isotope Be16 of Beryllium
    Be16,
}
impl super::RelativeAtomicMass for BerylliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Be5 => 5.0399f64,
            Self::Be6 => 6.0197264f64,
            Self::Be7 => 7.016928717f64,
            Self::Be8 => 8.005305102f64,
            Self::Be9 => 9.012183065f64,
            Self::Be10 => 10.013534695f64,
            Self::Be11 => 11.02166108f64,
            Self::Be12 => 12.0269221f64,
            Self::Be13 => 13.036135f64,
            Self::Be14 => 14.04289f64,
            Self::Be15 => 15.05342f64,
            Self::Be16 => 16.06167f64,
        }
    }
}
impl super::ElementVariant for BerylliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Be
    }
}
impl super::MassNumber for BerylliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Be5 => 5u16,
            Self::Be6 => 6u16,
            Self::Be7 => 7u16,
            Self::Be8 => 8u16,
            Self::Be9 => 9u16,
            Self::Be10 => 10u16,
            Self::Be11 => 11u16,
            Self::Be12 => 12u16,
            Self::Be13 => 13u16,
            Self::Be14 => 14u16,
            Self::Be15 => 15u16,
            Self::Be16 => 16u16,
        }
    }
}
impl super::IsotopicComposition for BerylliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Be9 => Some(1f64),
            Self::Be5
            | Self::Be6
            | Self::Be7
            | Self::Be8
            | Self::Be10
            | Self::Be11
            | Self::Be12
            | Self::Be13
            | Self::Be14
            | Self::Be15
            | Self::Be16 => None,
        }
    }
}
impl super::MostAbundantIsotope for BerylliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Be9
    }
}
impl From<BerylliumIsotope> for crate::Isotope {
    fn from(isotope: BerylliumIsotope) -> Self {
        crate::Isotope::Be(isotope)
    }
}
impl From<BerylliumIsotope> for crate::Element {
    fn from(_isotope: BerylliumIsotope) -> Self {
        crate::Element::Be
    }
}
impl TryFrom<u16> for BerylliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            5u16 => Ok(Self::Be5),
            6u16 => Ok(Self::Be6),
            7u16 => Ok(Self::Be7),
            8u16 => Ok(Self::Be8),
            9u16 => Ok(Self::Be9),
            10u16 => Ok(Self::Be10),
            11u16 => Ok(Self::Be11),
            12u16 => Ok(Self::Be12),
            13u16 => Ok(Self::Be13),
            14u16 => Ok(Self::Be14),
            15u16 => Ok(Self::Be15),
            16u16 => Ok(Self::Be16),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Be, value)),
        }
    }
}
impl std::fmt::Display for BerylliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Be5 => write!(f, "Be5"),
            Self::Be6 => write!(f, "Be6"),
            Self::Be7 => write!(f, "Be7"),
            Self::Be8 => write!(f, "Be8"),
            Self::Be9 => write!(f, "Be9"),
            Self::Be10 => write!(f, "Be10"),
            Self::Be11 => write!(f, "Be11"),
            Self::Be12 => write!(f, "Be12"),
            Self::Be13 => write!(f, "Be13"),
            Self::Be14 => write!(f, "Be14"),
            Self::Be15 => write!(f, "Be15"),
            Self::Be16 => write!(f, "Be16"),
        }
    }
}
