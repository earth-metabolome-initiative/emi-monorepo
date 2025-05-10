//! Isotopes of the element Roentgenium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Roentgenium
pub enum RoentgeniumIsotope {
    /// Isotope Rg272 of Roentgenium
    Rg272,
    /// Isotope Rg273 of Roentgenium
    Rg273,
    /// Isotope Rg274 of Roentgenium
    Rg274,
    /// Isotope Rg275 of Roentgenium
    Rg275,
    /// Isotope Rg276 of Roentgenium
    Rg276,
    /// Isotope Rg277 of Roentgenium
    Rg277,
    /// Isotope Rg278 of Roentgenium
    Rg278,
    /// Isotope Rg279 of Roentgenium
    Rg279,
    /// Isotope Rg280 of Roentgenium
    Rg280,
    /// Isotope Rg281 of Roentgenium
    Rg281,
    /// Isotope Rg282 of Roentgenium
    Rg282,
    /// Isotope Rg283 of Roentgenium
    Rg283,
}
impl super::RelativeAtomicMass for RoentgeniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rg272 => 272.15327f64,
            Self::Rg273 => 273.15313f64,
            Self::Rg274 => 274.15525f64,
            Self::Rg275 => 275.15594f64,
            Self::Rg276 => 276.15833f64,
            Self::Rg277 => 277.15907f64,
            Self::Rg278 => 278.16149f64,
            Self::Rg279 => 279.16272f64,
            Self::Rg280 => 280.16514f64,
            Self::Rg281 => 281.16636f64,
            Self::Rg282 => 282.16912f64,
            Self::Rg283 => 283.17054f64,
        }
    }
}
impl super::ElementVariant for RoentgeniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Rg
    }
}
impl super::MassNumber for RoentgeniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rg272 => 272u16,
            Self::Rg273 => 273u16,
            Self::Rg274 => 274u16,
            Self::Rg275 => 275u16,
            Self::Rg276 => 276u16,
            Self::Rg277 => 277u16,
            Self::Rg278 => 278u16,
            Self::Rg279 => 279u16,
            Self::Rg280 => 280u16,
            Self::Rg281 => 281u16,
            Self::Rg282 => 282u16,
            Self::Rg283 => 283u16,
        }
    }
}
impl super::IsotopicComposition for RoentgeniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for RoentgeniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rg283
    }
}
impl TryFrom<u16> for RoentgeniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            272u16 => Ok(Self::Rg272),
            273u16 => Ok(Self::Rg273),
            274u16 => Ok(Self::Rg274),
            275u16 => Ok(Self::Rg275),
            276u16 => Ok(Self::Rg276),
            277u16 => Ok(Self::Rg277),
            278u16 => Ok(Self::Rg278),
            279u16 => Ok(Self::Rg279),
            280u16 => Ok(Self::Rg280),
            281u16 => Ok(Self::Rg281),
            282u16 => Ok(Self::Rg282),
            283u16 => Ok(Self::Rg283),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rg, value)),
        }
    }
}
impl std::fmt::Display for RoentgeniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rg272 => write!(f, "Rg272"),
            Self::Rg273 => write!(f, "Rg273"),
            Self::Rg274 => write!(f, "Rg274"),
            Self::Rg275 => write!(f, "Rg275"),
            Self::Rg276 => write!(f, "Rg276"),
            Self::Rg277 => write!(f, "Rg277"),
            Self::Rg278 => write!(f, "Rg278"),
            Self::Rg279 => write!(f, "Rg279"),
            Self::Rg280 => write!(f, "Rg280"),
            Self::Rg281 => write!(f, "Rg281"),
            Self::Rg282 => write!(f, "Rg282"),
            Self::Rg283 => write!(f, "Rg283"),
        }
    }
}
