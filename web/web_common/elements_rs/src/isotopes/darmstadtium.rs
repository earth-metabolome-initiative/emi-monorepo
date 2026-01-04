//! Isotopes of the element Darmstadtium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Darmstadtium
pub enum DarmstadtiumIsotope {
    /// Isotope Ds267 of Darmstadtium
    Ds267,
    /// Isotope Ds268 of Darmstadtium
    Ds268,
    /// Isotope Ds269 of Darmstadtium
    Ds269,
    /// Isotope Ds270 of Darmstadtium
    Ds270,
    /// Isotope Ds271 of Darmstadtium
    Ds271,
    /// Isotope Ds272 of Darmstadtium
    Ds272,
    /// Isotope Ds273 of Darmstadtium
    Ds273,
    /// Isotope Ds274 of Darmstadtium
    Ds274,
    /// Isotope Ds275 of Darmstadtium
    Ds275,
    /// Isotope Ds276 of Darmstadtium
    Ds276,
    /// Isotope Ds277 of Darmstadtium
    Ds277,
    /// Isotope Ds278 of Darmstadtium
    Ds278,
    /// Isotope Ds279 of Darmstadtium
    Ds279,
    /// Isotope Ds280 of Darmstadtium
    Ds280,
    /// Isotope Ds281 of Darmstadtium
    Ds281,
}
impl super::RelativeAtomicMass for DarmstadtiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ds267 => 267.14377f64,
            Self::Ds268 => 268.14348f64,
            Self::Ds269 => 269.144752f64,
            Self::Ds270 => 270.144584f64,
            Self::Ds271 => 271.14595f64,
            Self::Ds272 => 272.14602f64,
            Self::Ds273 => 273.14856f64,
            Self::Ds274 => 274.14941f64,
            Self::Ds275 => 275.15203f64,
            Self::Ds276 => 276.15303f64,
            Self::Ds277 => 277.15591f64,
            Self::Ds278 => 278.15704f64,
            Self::Ds279 => 279.1601f64,
            Self::Ds280 => 280.16131f64,
            Self::Ds281 => 281.16451f64,
        }
    }
}
impl super::ElementVariant for DarmstadtiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ds
    }
}
impl super::MassNumber for DarmstadtiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ds267 => 267u16,
            Self::Ds268 => 268u16,
            Self::Ds269 => 269u16,
            Self::Ds270 => 270u16,
            Self::Ds271 => 271u16,
            Self::Ds272 => 272u16,
            Self::Ds273 => 273u16,
            Self::Ds274 => 274u16,
            Self::Ds275 => 275u16,
            Self::Ds276 => 276u16,
            Self::Ds277 => 277u16,
            Self::Ds278 => 278u16,
            Self::Ds279 => 279u16,
            Self::Ds280 => 280u16,
            Self::Ds281 => 281u16,
        }
    }
}
impl super::IsotopicComposition for DarmstadtiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for DarmstadtiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ds281
    }
}
impl From<DarmstadtiumIsotope> for crate::Isotope {
    fn from(isotope: DarmstadtiumIsotope) -> Self {
        crate::Isotope::Ds(isotope)
    }
}
impl From<DarmstadtiumIsotope> for crate::Element {
    fn from(_isotope: DarmstadtiumIsotope) -> Self {
        crate::Element::Ds
    }
}
impl TryFrom<u16> for DarmstadtiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            267u16 => Ok(Self::Ds267),
            268u16 => Ok(Self::Ds268),
            269u16 => Ok(Self::Ds269),
            270u16 => Ok(Self::Ds270),
            271u16 => Ok(Self::Ds271),
            272u16 => Ok(Self::Ds272),
            273u16 => Ok(Self::Ds273),
            274u16 => Ok(Self::Ds274),
            275u16 => Ok(Self::Ds275),
            276u16 => Ok(Self::Ds276),
            277u16 => Ok(Self::Ds277),
            278u16 => Ok(Self::Ds278),
            279u16 => Ok(Self::Ds279),
            280u16 => Ok(Self::Ds280),
            281u16 => Ok(Self::Ds281),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ds, value)),
        }
    }
}
impl std::fmt::Display for DarmstadtiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ds267 => write!(f, "Ds267"),
            Self::Ds268 => write!(f, "Ds268"),
            Self::Ds269 => write!(f, "Ds269"),
            Self::Ds270 => write!(f, "Ds270"),
            Self::Ds271 => write!(f, "Ds271"),
            Self::Ds272 => write!(f, "Ds272"),
            Self::Ds273 => write!(f, "Ds273"),
            Self::Ds274 => write!(f, "Ds274"),
            Self::Ds275 => write!(f, "Ds275"),
            Self::Ds276 => write!(f, "Ds276"),
            Self::Ds277 => write!(f, "Ds277"),
            Self::Ds278 => write!(f, "Ds278"),
            Self::Ds279 => write!(f, "Ds279"),
            Self::Ds280 => write!(f, "Ds280"),
            Self::Ds281 => write!(f, "Ds281"),
        }
    }
}
