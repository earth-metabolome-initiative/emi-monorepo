//! Isotopes of the element Americium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Americium
pub enum AmericiumIsotope {
    /// Isotope Am230 of Americium
    Am230,
    /// Isotope Am231 of Americium
    Am231,
    /// Isotope Am232 of Americium
    Am232,
    /// Isotope Am233 of Americium
    Am233,
    /// Isotope Am234 of Americium
    Am234,
    /// Isotope Am235 of Americium
    Am235,
    /// Isotope Am236 of Americium
    Am236,
    /// Isotope Am237 of Americium
    Am237,
    /// Isotope Am238 of Americium
    Am238,
    /// Isotope Am239 of Americium
    Am239,
    /// Isotope Am240 of Americium
    Am240,
    /// Isotope Am241 of Americium
    Am241,
    /// Isotope Am242 of Americium
    Am242,
    /// Isotope Am243 of Americium
    Am243,
    /// Isotope Am244 of Americium
    Am244,
    /// Isotope Am245 of Americium
    Am245,
    /// Isotope Am246 of Americium
    Am246,
    /// Isotope Am247 of Americium
    Am247,
    /// Isotope Am248 of Americium
    Am248,
    /// Isotope Am249 of Americium
    Am249,
}
impl super::RelativeAtomicMass for AmericiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Am230 => 230.04609f64,
            Self::Am231 => 231.04556f64,
            Self::Am232 => 232.04645f64,
            Self::Am233 => 233.04644f64,
            Self::Am234 => 234.04773f64,
            Self::Am235 => 235.047908f64,
            Self::Am236 => 236.04943f64,
            Self::Am237 => 237.049996f64,
            Self::Am238 => 238.051985f64,
            Self::Am239 => 239.0530247f64,
            Self::Am240 => 240.0553f64,
            Self::Am241 => 241.0568293f64,
            Self::Am242 => 242.0595494f64,
            Self::Am243 => 243.0613813f64,
            Self::Am244 => 244.0642851f64,
            Self::Am245 => 245.0664548f64,
            Self::Am246 => 246.069775f64,
            Self::Am247 => 247.07209f64,
            Self::Am248 => 248.07575f64,
            Self::Am249 => 249.07848f64,
        }
    }
}
impl super::ElementVariant for AmericiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Am
    }
}
impl super::MassNumber for AmericiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Am230 => 230u16,
            Self::Am231 => 231u16,
            Self::Am232 => 232u16,
            Self::Am233 => 233u16,
            Self::Am234 => 234u16,
            Self::Am235 => 235u16,
            Self::Am236 => 236u16,
            Self::Am237 => 237u16,
            Self::Am238 => 238u16,
            Self::Am239 => 239u16,
            Self::Am240 => 240u16,
            Self::Am241 => 241u16,
            Self::Am242 => 242u16,
            Self::Am243 => 243u16,
            Self::Am244 => 244u16,
            Self::Am245 => 245u16,
            Self::Am246 => 246u16,
            Self::Am247 => 247u16,
            Self::Am248 => 248u16,
            Self::Am249 => 249u16,
        }
    }
}
impl super::IsotopicComposition for AmericiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for AmericiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Am249
    }
}
impl TryFrom<u16> for AmericiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            230u16 => Ok(Self::Am230),
            231u16 => Ok(Self::Am231),
            232u16 => Ok(Self::Am232),
            233u16 => Ok(Self::Am233),
            234u16 => Ok(Self::Am234),
            235u16 => Ok(Self::Am235),
            236u16 => Ok(Self::Am236),
            237u16 => Ok(Self::Am237),
            238u16 => Ok(Self::Am238),
            239u16 => Ok(Self::Am239),
            240u16 => Ok(Self::Am240),
            241u16 => Ok(Self::Am241),
            242u16 => Ok(Self::Am242),
            243u16 => Ok(Self::Am243),
            244u16 => Ok(Self::Am244),
            245u16 => Ok(Self::Am245),
            246u16 => Ok(Self::Am246),
            247u16 => Ok(Self::Am247),
            248u16 => Ok(Self::Am248),
            249u16 => Ok(Self::Am249),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Am, value)),
        }
    }
}
impl std::fmt::Display for AmericiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Am230 => write!(f, "Am230"),
            Self::Am231 => write!(f, "Am231"),
            Self::Am232 => write!(f, "Am232"),
            Self::Am233 => write!(f, "Am233"),
            Self::Am234 => write!(f, "Am234"),
            Self::Am235 => write!(f, "Am235"),
            Self::Am236 => write!(f, "Am236"),
            Self::Am237 => write!(f, "Am237"),
            Self::Am238 => write!(f, "Am238"),
            Self::Am239 => write!(f, "Am239"),
            Self::Am240 => write!(f, "Am240"),
            Self::Am241 => write!(f, "Am241"),
            Self::Am242 => write!(f, "Am242"),
            Self::Am243 => write!(f, "Am243"),
            Self::Am244 => write!(f, "Am244"),
            Self::Am245 => write!(f, "Am245"),
            Self::Am246 => write!(f, "Am246"),
            Self::Am247 => write!(f, "Am247"),
            Self::Am248 => write!(f, "Am248"),
            Self::Am249 => write!(f, "Am249"),
        }
    }
}
