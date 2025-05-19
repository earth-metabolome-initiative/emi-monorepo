//! Isotopes of the element Meitnerium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Meitnerium
pub enum MeitneriumIsotope {
    /// Isotope Mt265 of Meitnerium
    Mt265,
    /// Isotope Mt266 of Meitnerium
    Mt266,
    /// Isotope Mt267 of Meitnerium
    Mt267,
    /// Isotope Mt268 of Meitnerium
    Mt268,
    /// Isotope Mt269 of Meitnerium
    Mt269,
    /// Isotope Mt270 of Meitnerium
    Mt270,
    /// Isotope Mt271 of Meitnerium
    Mt271,
    /// Isotope Mt272 of Meitnerium
    Mt272,
    /// Isotope Mt273 of Meitnerium
    Mt273,
    /// Isotope Mt274 of Meitnerium
    Mt274,
    /// Isotope Mt275 of Meitnerium
    Mt275,
    /// Isotope Mt276 of Meitnerium
    Mt276,
    /// Isotope Mt277 of Meitnerium
    Mt277,
    /// Isotope Mt278 of Meitnerium
    Mt278,
    /// Isotope Mt279 of Meitnerium
    Mt279,
}
impl super::RelativeAtomicMass for MeitneriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mt265 => 265.136f64,
            Self::Mt266 => 266.13737f64,
            Self::Mt267 => 267.13719f64,
            Self::Mt268 => 268.13865f64,
            Self::Mt269 => 269.13882f64,
            Self::Mt270 => 270.14033f64,
            Self::Mt271 => 271.14074f64,
            Self::Mt272 => 272.14341f64,
            Self::Mt273 => 273.1444f64,
            Self::Mt274 => 274.14724f64,
            Self::Mt275 => 275.14882f64,
            Self::Mt276 => 276.15159f64,
            Self::Mt277 => 277.15327f64,
            Self::Mt278 => 278.15631f64,
            Self::Mt279 => 279.15808f64,
        }
    }
}
impl super::ElementVariant for MeitneriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mt
    }
}
impl super::MassNumber for MeitneriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mt265 => 265u16,
            Self::Mt266 => 266u16,
            Self::Mt267 => 267u16,
            Self::Mt268 => 268u16,
            Self::Mt269 => 269u16,
            Self::Mt270 => 270u16,
            Self::Mt271 => 271u16,
            Self::Mt272 => 272u16,
            Self::Mt273 => 273u16,
            Self::Mt274 => 274u16,
            Self::Mt275 => 275u16,
            Self::Mt276 => 276u16,
            Self::Mt277 => 277u16,
            Self::Mt278 => 278u16,
            Self::Mt279 => 279u16,
        }
    }
}
impl super::IsotopicComposition for MeitneriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for MeitneriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mt279
    }
}
impl TryFrom<u16> for MeitneriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            265u16 => Ok(Self::Mt265),
            266u16 => Ok(Self::Mt266),
            267u16 => Ok(Self::Mt267),
            268u16 => Ok(Self::Mt268),
            269u16 => Ok(Self::Mt269),
            270u16 => Ok(Self::Mt270),
            271u16 => Ok(Self::Mt271),
            272u16 => Ok(Self::Mt272),
            273u16 => Ok(Self::Mt273),
            274u16 => Ok(Self::Mt274),
            275u16 => Ok(Self::Mt275),
            276u16 => Ok(Self::Mt276),
            277u16 => Ok(Self::Mt277),
            278u16 => Ok(Self::Mt278),
            279u16 => Ok(Self::Mt279),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Mt, value)),
        }
    }
}
impl std::fmt::Display for MeitneriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mt265 => write!(f, "Mt265"),
            Self::Mt266 => write!(f, "Mt266"),
            Self::Mt267 => write!(f, "Mt267"),
            Self::Mt268 => write!(f, "Mt268"),
            Self::Mt269 => write!(f, "Mt269"),
            Self::Mt270 => write!(f, "Mt270"),
            Self::Mt271 => write!(f, "Mt271"),
            Self::Mt272 => write!(f, "Mt272"),
            Self::Mt273 => write!(f, "Mt273"),
            Self::Mt274 => write!(f, "Mt274"),
            Self::Mt275 => write!(f, "Mt275"),
            Self::Mt276 => write!(f, "Mt276"),
            Self::Mt277 => write!(f, "Mt277"),
            Self::Mt278 => write!(f, "Mt278"),
            Self::Mt279 => write!(f, "Mt279"),
        }
    }
}
