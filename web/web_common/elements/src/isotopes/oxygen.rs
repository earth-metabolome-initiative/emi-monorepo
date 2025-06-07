//! Isotopes of the element Oxygen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Oxygen
pub enum OxygenIsotope {
    /// Isotope O12 of Oxygen
    O12,
    /// Isotope O13 of Oxygen
    O13,
    /// Isotope O14 of Oxygen
    O14,
    /// Isotope O15 of Oxygen
    O15,
    /// Isotope O16 of Oxygen
    O16,
    /// Isotope O17 of Oxygen
    O17,
    /// Isotope O18 of Oxygen
    O18,
    /// Isotope O19 of Oxygen
    O19,
    /// Isotope O20 of Oxygen
    O20,
    /// Isotope O21 of Oxygen
    O21,
    /// Isotope O22 of Oxygen
    O22,
    /// Isotope O23 of Oxygen
    O23,
    /// Isotope O24 of Oxygen
    O24,
    /// Isotope O25 of Oxygen
    O25,
    /// Isotope O26 of Oxygen
    O26,
    /// Isotope O27 of Oxygen
    O27,
    /// Isotope O28 of Oxygen
    O28,
}
impl super::RelativeAtomicMass for OxygenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::O12 => 12.034262f64,
            Self::O13 => 13.024815f64,
            Self::O14 => 14.00859636f64,
            Self::O15 => 15.00306562f64,
            Self::O16 => 15.99491461957f64,
            Self::O17 => 16.9991317565f64,
            Self::O18 => 17.99915961286f64,
            Self::O19 => 19.003578f64,
            Self::O20 => 20.00407535f64,
            Self::O21 => 21.008655f64,
            Self::O22 => 22.009966f64,
            Self::O23 => 23.015696f64,
            Self::O24 => 24.01986f64,
            Self::O25 => 25.02936f64,
            Self::O26 => 26.03729f64,
            Self::O27 => 27.04772f64,
            Self::O28 => 28.05591f64,
        }
    }
}
impl super::ElementVariant for OxygenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::O
    }
}
impl super::MassNumber for OxygenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::O12 => 12u16,
            Self::O13 => 13u16,
            Self::O14 => 14u16,
            Self::O15 => 15u16,
            Self::O16 => 16u16,
            Self::O17 => 17u16,
            Self::O18 => 18u16,
            Self::O19 => 19u16,
            Self::O20 => 20u16,
            Self::O21 => 21u16,
            Self::O22 => 22u16,
            Self::O23 => 23u16,
            Self::O24 => 24u16,
            Self::O25 => 25u16,
            Self::O26 => 26u16,
            Self::O27 => 27u16,
            Self::O28 => 28u16,
        }
    }
}
impl super::IsotopicComposition for OxygenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::O16 => Some(0.99757f64),
            Self::O17 => Some(0.00038f64),
            Self::O18 => Some(0.00205f64),
            Self::O12
            | Self::O13
            | Self::O14
            | Self::O15
            | Self::O19
            | Self::O20
            | Self::O21
            | Self::O22
            | Self::O23
            | Self::O24
            | Self::O25
            | Self::O26
            | Self::O27
            | Self::O28 => None,
        }
    }
}
impl super::MostAbundantIsotope for OxygenIsotope {
    fn most_abundant_isotope() -> Self {
        Self::O16
    }
}
impl From<OxygenIsotope> for crate::Isotope {
    fn from(isotope: OxygenIsotope) -> Self {
        crate::Isotope::O(isotope)
    }
}
impl From<OxygenIsotope> for crate::Element {
    fn from(_isotope: OxygenIsotope) -> Self {
        crate::Element::O
    }
}
impl TryFrom<u16> for OxygenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            12u16 => Ok(Self::O12),
            13u16 => Ok(Self::O13),
            14u16 => Ok(Self::O14),
            15u16 => Ok(Self::O15),
            16u16 => Ok(Self::O16),
            17u16 => Ok(Self::O17),
            18u16 => Ok(Self::O18),
            19u16 => Ok(Self::O19),
            20u16 => Ok(Self::O20),
            21u16 => Ok(Self::O21),
            22u16 => Ok(Self::O22),
            23u16 => Ok(Self::O23),
            24u16 => Ok(Self::O24),
            25u16 => Ok(Self::O25),
            26u16 => Ok(Self::O26),
            27u16 => Ok(Self::O27),
            28u16 => Ok(Self::O28),
            _ => Err(crate::errors::Error::Isotope(crate::Element::O, value)),
        }
    }
}
impl std::fmt::Display for OxygenIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::O12 => write!(f, "O12"),
            Self::O13 => write!(f, "O13"),
            Self::O14 => write!(f, "O14"),
            Self::O15 => write!(f, "O15"),
            Self::O16 => write!(f, "O16"),
            Self::O17 => write!(f, "O17"),
            Self::O18 => write!(f, "O18"),
            Self::O19 => write!(f, "O19"),
            Self::O20 => write!(f, "O20"),
            Self::O21 => write!(f, "O21"),
            Self::O22 => write!(f, "O22"),
            Self::O23 => write!(f, "O23"),
            Self::O24 => write!(f, "O24"),
            Self::O25 => write!(f, "O25"),
            Self::O26 => write!(f, "O26"),
            Self::O27 => write!(f, "O27"),
            Self::O28 => write!(f, "O28"),
        }
    }
}
