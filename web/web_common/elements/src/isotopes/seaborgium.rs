//! Isotopes of the element Seaborgium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Seaborgium
pub enum SeaborgiumIsotope {
    /// Isotope Sg258 of Seaborgium
    Sg258,
    /// Isotope Sg259 of Seaborgium
    Sg259,
    /// Isotope Sg260 of Seaborgium
    Sg260,
    /// Isotope Sg261 of Seaborgium
    Sg261,
    /// Isotope Sg262 of Seaborgium
    Sg262,
    /// Isotope Sg263 of Seaborgium
    Sg263,
    /// Isotope Sg264 of Seaborgium
    Sg264,
    /// Isotope Sg265 of Seaborgium
    Sg265,
    /// Isotope Sg266 of Seaborgium
    Sg266,
    /// Isotope Sg267 of Seaborgium
    Sg267,
    /// Isotope Sg268 of Seaborgium
    Sg268,
    /// Isotope Sg269 of Seaborgium
    Sg269,
    /// Isotope Sg270 of Seaborgium
    Sg270,
    /// Isotope Sg271 of Seaborgium
    Sg271,
    /// Isotope Sg272 of Seaborgium
    Sg272,
    /// Isotope Sg273 of Seaborgium
    Sg273,
}
impl super::RelativeAtomicMass for SeaborgiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sg258 => 258.11298f64,
            Self::Sg259 => 259.1144f64,
            Self::Sg260 => 260.114384f64,
            Self::Sg261 => 261.115949f64,
            Self::Sg262 => 262.116337f64,
            Self::Sg263 => 263.11829f64,
            Self::Sg264 => 264.11893f64,
            Self::Sg265 => 265.12109f64,
            Self::Sg266 => 266.12198f64,
            Self::Sg267 => 267.12436f64,
            Self::Sg268 => 268.12539f64,
            Self::Sg269 => 269.12863f64,
            Self::Sg270 => 270.13043f64,
            Self::Sg271 => 271.13393f64,
            Self::Sg272 => 272.13589f64,
            Self::Sg273 => 273.13958f64,
        }
    }
}
impl super::ElementVariant for SeaborgiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sg
    }
}
impl super::MassNumber for SeaborgiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sg258 => 258u16,
            Self::Sg259 => 259u16,
            Self::Sg260 => 260u16,
            Self::Sg261 => 261u16,
            Self::Sg262 => 262u16,
            Self::Sg263 => 263u16,
            Self::Sg264 => 264u16,
            Self::Sg265 => 265u16,
            Self::Sg266 => 266u16,
            Self::Sg267 => 267u16,
            Self::Sg268 => 268u16,
            Self::Sg269 => 269u16,
            Self::Sg270 => 270u16,
            Self::Sg271 => 271u16,
            Self::Sg272 => 272u16,
            Self::Sg273 => 273u16,
        }
    }
}
impl super::IsotopicComposition for SeaborgiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for SeaborgiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sg273
    }
}
impl From<SeaborgiumIsotope> for crate::Isotope {
    fn from(isotope: SeaborgiumIsotope) -> Self {
        crate::Isotope::Sg(isotope)
    }
}
impl From<SeaborgiumIsotope> for crate::Element {
    fn from(_isotope: SeaborgiumIsotope) -> Self {
        crate::Element::Sg
    }
}
impl TryFrom<u16> for SeaborgiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            258u16 => Ok(Self::Sg258),
            259u16 => Ok(Self::Sg259),
            260u16 => Ok(Self::Sg260),
            261u16 => Ok(Self::Sg261),
            262u16 => Ok(Self::Sg262),
            263u16 => Ok(Self::Sg263),
            264u16 => Ok(Self::Sg264),
            265u16 => Ok(Self::Sg265),
            266u16 => Ok(Self::Sg266),
            267u16 => Ok(Self::Sg267),
            268u16 => Ok(Self::Sg268),
            269u16 => Ok(Self::Sg269),
            270u16 => Ok(Self::Sg270),
            271u16 => Ok(Self::Sg271),
            272u16 => Ok(Self::Sg272),
            273u16 => Ok(Self::Sg273),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sg, value)),
        }
    }
}
impl std::fmt::Display for SeaborgiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sg258 => write!(f, "Sg258"),
            Self::Sg259 => write!(f, "Sg259"),
            Self::Sg260 => write!(f, "Sg260"),
            Self::Sg261 => write!(f, "Sg261"),
            Self::Sg262 => write!(f, "Sg262"),
            Self::Sg263 => write!(f, "Sg263"),
            Self::Sg264 => write!(f, "Sg264"),
            Self::Sg265 => write!(f, "Sg265"),
            Self::Sg266 => write!(f, "Sg266"),
            Self::Sg267 => write!(f, "Sg267"),
            Self::Sg268 => write!(f, "Sg268"),
            Self::Sg269 => write!(f, "Sg269"),
            Self::Sg270 => write!(f, "Sg270"),
            Self::Sg271 => write!(f, "Sg271"),
            Self::Sg272 => write!(f, "Sg272"),
            Self::Sg273 => write!(f, "Sg273"),
        }
    }
}
