//! Isotopes of the element Berkelium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Berkelium
pub enum BerkeliumIsotope {
    /// Isotope Bk234 of Berkelium
    Bk234,
    /// Isotope Bk235 of Berkelium
    Bk235,
    /// Isotope Bk236 of Berkelium
    Bk236,
    /// Isotope Bk237 of Berkelium
    Bk237,
    /// Isotope Bk238 of Berkelium
    Bk238,
    /// Isotope Bk239 of Berkelium
    Bk239,
    /// Isotope Bk240 of Berkelium
    Bk240,
    /// Isotope Bk241 of Berkelium
    Bk241,
    /// Isotope Bk242 of Berkelium
    Bk242,
    /// Isotope Bk243 of Berkelium
    Bk243,
    /// Isotope Bk244 of Berkelium
    Bk244,
    /// Isotope Bk245 of Berkelium
    Bk245,
    /// Isotope Bk246 of Berkelium
    Bk246,
    /// Isotope Bk247 of Berkelium
    Bk247,
    /// Isotope Bk248 of Berkelium
    Bk248,
    /// Isotope Bk249 of Berkelium
    Bk249,
    /// Isotope Bk250 of Berkelium
    Bk250,
    /// Isotope Bk251 of Berkelium
    Bk251,
    /// Isotope Bk252 of Berkelium
    Bk252,
    /// Isotope Bk253 of Berkelium
    Bk253,
    /// Isotope Bk254 of Berkelium
    Bk254,
}
impl super::RelativeAtomicMass for BerkeliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Bk234 => 234.05727f64,
            Self::Bk235 => 235.05658f64,
            Self::Bk236 => 236.05748f64,
            Self::Bk237 => 237.0571f64,
            Self::Bk238 => 238.0582f64,
            Self::Bk239 => 239.05824f64,
            Self::Bk240 => 240.05976f64,
            Self::Bk241 => 241.06016f64,
            Self::Bk242 => 242.06198f64,
            Self::Bk243 => 243.0630078f64,
            Self::Bk244 => 244.065181f64,
            Self::Bk245 => 245.0663618f64,
            Self::Bk246 => 246.068673f64,
            Self::Bk247 => 247.0703073f64,
            Self::Bk248 => 248.073088f64,
            Self::Bk249 => 249.0749877f64,
            Self::Bk250 => 250.0783167f64,
            Self::Bk251 => 251.080762f64,
            Self::Bk252 => 252.08431f64,
            Self::Bk253 => 253.08688f64,
            Self::Bk254 => 254.0906f64,
        }
    }
}
impl super::ElementVariant for BerkeliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Bk
    }
}
impl super::MassNumber for BerkeliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Bk234 => 234u16,
            Self::Bk235 => 235u16,
            Self::Bk236 => 236u16,
            Self::Bk237 => 237u16,
            Self::Bk238 => 238u16,
            Self::Bk239 => 239u16,
            Self::Bk240 => 240u16,
            Self::Bk241 => 241u16,
            Self::Bk242 => 242u16,
            Self::Bk243 => 243u16,
            Self::Bk244 => 244u16,
            Self::Bk245 => 245u16,
            Self::Bk246 => 246u16,
            Self::Bk247 => 247u16,
            Self::Bk248 => 248u16,
            Self::Bk249 => 249u16,
            Self::Bk250 => 250u16,
            Self::Bk251 => 251u16,
            Self::Bk252 => 252u16,
            Self::Bk253 => 253u16,
            Self::Bk254 => 254u16,
        }
    }
}
impl super::IsotopicComposition for BerkeliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for BerkeliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Bk254
    }
}
impl TryFrom<u16> for BerkeliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            234u16 => Ok(Self::Bk234),
            235u16 => Ok(Self::Bk235),
            236u16 => Ok(Self::Bk236),
            237u16 => Ok(Self::Bk237),
            238u16 => Ok(Self::Bk238),
            239u16 => Ok(Self::Bk239),
            240u16 => Ok(Self::Bk240),
            241u16 => Ok(Self::Bk241),
            242u16 => Ok(Self::Bk242),
            243u16 => Ok(Self::Bk243),
            244u16 => Ok(Self::Bk244),
            245u16 => Ok(Self::Bk245),
            246u16 => Ok(Self::Bk246),
            247u16 => Ok(Self::Bk247),
            248u16 => Ok(Self::Bk248),
            249u16 => Ok(Self::Bk249),
            250u16 => Ok(Self::Bk250),
            251u16 => Ok(Self::Bk251),
            252u16 => Ok(Self::Bk252),
            253u16 => Ok(Self::Bk253),
            254u16 => Ok(Self::Bk254),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Bk, value)),
        }
    }
}
impl std::fmt::Display for BerkeliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bk234 => write!(f, "Bk234"),
            Self::Bk235 => write!(f, "Bk235"),
            Self::Bk236 => write!(f, "Bk236"),
            Self::Bk237 => write!(f, "Bk237"),
            Self::Bk238 => write!(f, "Bk238"),
            Self::Bk239 => write!(f, "Bk239"),
            Self::Bk240 => write!(f, "Bk240"),
            Self::Bk241 => write!(f, "Bk241"),
            Self::Bk242 => write!(f, "Bk242"),
            Self::Bk243 => write!(f, "Bk243"),
            Self::Bk244 => write!(f, "Bk244"),
            Self::Bk245 => write!(f, "Bk245"),
            Self::Bk246 => write!(f, "Bk246"),
            Self::Bk247 => write!(f, "Bk247"),
            Self::Bk248 => write!(f, "Bk248"),
            Self::Bk249 => write!(f, "Bk249"),
            Self::Bk250 => write!(f, "Bk250"),
            Self::Bk251 => write!(f, "Bk251"),
            Self::Bk252 => write!(f, "Bk252"),
            Self::Bk253 => write!(f, "Bk253"),
            Self::Bk254 => write!(f, "Bk254"),
        }
    }
}
