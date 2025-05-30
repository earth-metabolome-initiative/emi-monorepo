//! Isotopes of the element Nihonium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Nihonium
pub enum NihoniumIsotope {
    /// Isotope Nh278 of Nihonium
    Nh278,
    /// Isotope Nh279 of Nihonium
    Nh279,
    /// Isotope Nh280 of Nihonium
    Nh280,
    /// Isotope Nh281 of Nihonium
    Nh281,
    /// Isotope Nh282 of Nihonium
    Nh282,
    /// Isotope Nh283 of Nihonium
    Nh283,
    /// Isotope Nh284 of Nihonium
    Nh284,
    /// Isotope Nh285 of Nihonium
    Nh285,
    /// Isotope Nh286 of Nihonium
    Nh286,
    /// Isotope Nh287 of Nihonium
    Nh287,
}
impl super::RelativeAtomicMass for NihoniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Nh278 => 278.17058f64,
            Self::Nh279 => 279.17095f64,
            Self::Nh280 => 280.17293f64,
            Self::Nh281 => 281.17348f64,
            Self::Nh282 => 282.17567f64,
            Self::Nh283 => 283.17657f64,
            Self::Nh284 => 284.17873f64,
            Self::Nh285 => 285.17973f64,
            Self::Nh286 => 286.18221f64,
            Self::Nh287 => 287.18339f64,
        }
    }
}
impl super::ElementVariant for NihoniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Nh
    }
}
impl super::MassNumber for NihoniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Nh278 => 278u16,
            Self::Nh279 => 279u16,
            Self::Nh280 => 280u16,
            Self::Nh281 => 281u16,
            Self::Nh282 => 282u16,
            Self::Nh283 => 283u16,
            Self::Nh284 => 284u16,
            Self::Nh285 => 285u16,
            Self::Nh286 => 286u16,
            Self::Nh287 => 287u16,
        }
    }
}
impl super::IsotopicComposition for NihoniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for NihoniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Nh287
    }
}
impl From<NihoniumIsotope> for crate::Isotope {
    fn from(isotope: NihoniumIsotope) -> Self {
        crate::Isotope::Nh(isotope)
    }
}
impl From<NihoniumIsotope> for crate::Element {
    fn from(_isotope: NihoniumIsotope) -> Self {
        crate::Element::Nh
    }
}
impl TryFrom<u16> for NihoniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            278u16 => Ok(Self::Nh278),
            279u16 => Ok(Self::Nh279),
            280u16 => Ok(Self::Nh280),
            281u16 => Ok(Self::Nh281),
            282u16 => Ok(Self::Nh282),
            283u16 => Ok(Self::Nh283),
            284u16 => Ok(Self::Nh284),
            285u16 => Ok(Self::Nh285),
            286u16 => Ok(Self::Nh286),
            287u16 => Ok(Self::Nh287),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Nh, value)),
        }
    }
}
impl std::fmt::Display for NihoniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nh278 => write!(f, "Nh278"),
            Self::Nh279 => write!(f, "Nh279"),
            Self::Nh280 => write!(f, "Nh280"),
            Self::Nh281 => write!(f, "Nh281"),
            Self::Nh282 => write!(f, "Nh282"),
            Self::Nh283 => write!(f, "Nh283"),
            Self::Nh284 => write!(f, "Nh284"),
            Self::Nh285 => write!(f, "Nh285"),
            Self::Nh286 => write!(f, "Nh286"),
            Self::Nh287 => write!(f, "Nh287"),
        }
    }
}
