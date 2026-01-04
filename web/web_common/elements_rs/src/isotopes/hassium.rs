//! Isotopes of the element Hassium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Hassium
pub enum HassiumIsotope {
    /// Isotope Hs263 of Hassium
    Hs263,
    /// Isotope Hs264 of Hassium
    Hs264,
    /// Isotope Hs265 of Hassium
    Hs265,
    /// Isotope Hs266 of Hassium
    Hs266,
    /// Isotope Hs267 of Hassium
    Hs267,
    /// Isotope Hs268 of Hassium
    Hs268,
    /// Isotope Hs269 of Hassium
    Hs269,
    /// Isotope Hs270 of Hassium
    Hs270,
    /// Isotope Hs271 of Hassium
    Hs271,
    /// Isotope Hs272 of Hassium
    Hs272,
    /// Isotope Hs273 of Hassium
    Hs273,
    /// Isotope Hs274 of Hassium
    Hs274,
    /// Isotope Hs275 of Hassium
    Hs275,
    /// Isotope Hs276 of Hassium
    Hs276,
    /// Isotope Hs277 of Hassium
    Hs277,
}
impl super::RelativeAtomicMass for HassiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Hs263 => 263.12852f64,
            Self::Hs264 => 264.128357f64,
            Self::Hs265 => 265.129793f64,
            Self::Hs266 => 266.130046f64,
            Self::Hs267 => 267.13167f64,
            Self::Hs268 => 268.13186f64,
            Self::Hs269 => 269.13375f64,
            Self::Hs270 => 270.13429f64,
            Self::Hs271 => 271.13717f64,
            Self::Hs272 => 272.1385f64,
            Self::Hs273 => 273.14168f64,
            Self::Hs274 => 274.1433f64,
            Self::Hs275 => 275.14667f64,
            Self::Hs276 => 276.14846f64,
            Self::Hs277 => 277.1519f64,
        }
    }
}
impl super::ElementVariant for HassiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Hs
    }
}
impl super::MassNumber for HassiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Hs263 => 263u16,
            Self::Hs264 => 264u16,
            Self::Hs265 => 265u16,
            Self::Hs266 => 266u16,
            Self::Hs267 => 267u16,
            Self::Hs268 => 268u16,
            Self::Hs269 => 269u16,
            Self::Hs270 => 270u16,
            Self::Hs271 => 271u16,
            Self::Hs272 => 272u16,
            Self::Hs273 => 273u16,
            Self::Hs274 => 274u16,
            Self::Hs275 => 275u16,
            Self::Hs276 => 276u16,
            Self::Hs277 => 277u16,
        }
    }
}
impl super::IsotopicComposition for HassiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for HassiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Hs277
    }
}
impl From<HassiumIsotope> for crate::Isotope {
    fn from(isotope: HassiumIsotope) -> Self {
        crate::Isotope::Hs(isotope)
    }
}
impl From<HassiumIsotope> for crate::Element {
    fn from(_isotope: HassiumIsotope) -> Self {
        crate::Element::Hs
    }
}
impl TryFrom<u16> for HassiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            263u16 => Ok(Self::Hs263),
            264u16 => Ok(Self::Hs264),
            265u16 => Ok(Self::Hs265),
            266u16 => Ok(Self::Hs266),
            267u16 => Ok(Self::Hs267),
            268u16 => Ok(Self::Hs268),
            269u16 => Ok(Self::Hs269),
            270u16 => Ok(Self::Hs270),
            271u16 => Ok(Self::Hs271),
            272u16 => Ok(Self::Hs272),
            273u16 => Ok(Self::Hs273),
            274u16 => Ok(Self::Hs274),
            275u16 => Ok(Self::Hs275),
            276u16 => Ok(Self::Hs276),
            277u16 => Ok(Self::Hs277),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Hs, value)),
        }
    }
}
impl std::fmt::Display for HassiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hs263 => write!(f, "Hs263"),
            Self::Hs264 => write!(f, "Hs264"),
            Self::Hs265 => write!(f, "Hs265"),
            Self::Hs266 => write!(f, "Hs266"),
            Self::Hs267 => write!(f, "Hs267"),
            Self::Hs268 => write!(f, "Hs268"),
            Self::Hs269 => write!(f, "Hs269"),
            Self::Hs270 => write!(f, "Hs270"),
            Self::Hs271 => write!(f, "Hs271"),
            Self::Hs272 => write!(f, "Hs272"),
            Self::Hs273 => write!(f, "Hs273"),
            Self::Hs274 => write!(f, "Hs274"),
            Self::Hs275 => write!(f, "Hs275"),
            Self::Hs276 => write!(f, "Hs276"),
            Self::Hs277 => write!(f, "Hs277"),
        }
    }
}
