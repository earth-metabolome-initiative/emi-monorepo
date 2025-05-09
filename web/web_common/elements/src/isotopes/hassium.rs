#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum HassiumIsotope {
    Hs263,
    Hs264,
    Hs265,
    Hs266,
    Hs267,
    Hs268,
    Hs269,
    Hs270,
    Hs271,
    Hs272,
    Hs273,
    Hs274,
    Hs275,
    Hs276,
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
        match self {
            Self::Hs263 => None,
            Self::Hs264 => None,
            Self::Hs265 => None,
            Self::Hs266 => None,
            Self::Hs267 => None,
            Self::Hs268 => None,
            Self::Hs269 => None,
            Self::Hs270 => None,
            Self::Hs271 => None,
            Self::Hs272 => None,
            Self::Hs273 => None,
            Self::Hs274 => None,
            Self::Hs275 => None,
            Self::Hs276 => None,
            Self::Hs277 => None,
        }
    }
}
impl super::MostAbundantIsotope for HassiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Hs277
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
