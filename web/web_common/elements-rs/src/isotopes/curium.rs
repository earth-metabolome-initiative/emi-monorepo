//! Isotopes of the element Curium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Curium
pub enum CuriumIsotope {
    /// Isotope Cm232 of Curium
    Cm232,
    /// Isotope Cm233 of Curium
    Cm233,
    /// Isotope Cm234 of Curium
    Cm234,
    /// Isotope Cm235 of Curium
    Cm235,
    /// Isotope Cm236 of Curium
    Cm236,
    /// Isotope Cm237 of Curium
    Cm237,
    /// Isotope Cm238 of Curium
    Cm238,
    /// Isotope Cm239 of Curium
    Cm239,
    /// Isotope Cm240 of Curium
    Cm240,
    /// Isotope Cm241 of Curium
    Cm241,
    /// Isotope Cm242 of Curium
    Cm242,
    /// Isotope Cm243 of Curium
    Cm243,
    /// Isotope Cm244 of Curium
    Cm244,
    /// Isotope Cm245 of Curium
    Cm245,
    /// Isotope Cm246 of Curium
    Cm246,
    /// Isotope Cm247 of Curium
    Cm247,
    /// Isotope Cm248 of Curium
    Cm248,
    /// Isotope Cm249 of Curium
    Cm249,
    /// Isotope Cm250 of Curium
    Cm250,
    /// Isotope Cm251 of Curium
    Cm251,
    /// Isotope Cm252 of Curium
    Cm252,
}
impl super::RelativeAtomicMass for CuriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cm232 => 232.04982f64,
            Self::Cm233 => 233.05077f64,
            Self::Cm234 => 234.05016f64,
            Self::Cm235 => 235.05154f64,
            Self::Cm236 => 236.051374f64,
            Self::Cm237 => 237.052869f64,
            Self::Cm238 => 238.053081f64,
            Self::Cm239 => 239.05491f64,
            Self::Cm240 => 240.0555297f64,
            Self::Cm241 => 241.0576532f64,
            Self::Cm242 => 242.058836f64,
            Self::Cm243 => 243.0613893f64,
            Self::Cm244 => 244.0627528f64,
            Self::Cm245 => 245.0654915f64,
            Self::Cm246 => 246.0672238f64,
            Self::Cm247 => 247.0703541f64,
            Self::Cm248 => 248.0723499f64,
            Self::Cm249 => 249.0759548f64,
            Self::Cm250 => 250.078358f64,
            Self::Cm251 => 251.082286f64,
            Self::Cm252 => 252.08487f64,
        }
    }
}
impl super::ElementVariant for CuriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cm
    }
}
impl super::MassNumber for CuriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cm232 => 232u16,
            Self::Cm233 => 233u16,
            Self::Cm234 => 234u16,
            Self::Cm235 => 235u16,
            Self::Cm236 => 236u16,
            Self::Cm237 => 237u16,
            Self::Cm238 => 238u16,
            Self::Cm239 => 239u16,
            Self::Cm240 => 240u16,
            Self::Cm241 => 241u16,
            Self::Cm242 => 242u16,
            Self::Cm243 => 243u16,
            Self::Cm244 => 244u16,
            Self::Cm245 => 245u16,
            Self::Cm246 => 246u16,
            Self::Cm247 => 247u16,
            Self::Cm248 => 248u16,
            Self::Cm249 => 249u16,
            Self::Cm250 => 250u16,
            Self::Cm251 => 251u16,
            Self::Cm252 => 252u16,
        }
    }
}
impl super::IsotopicComposition for CuriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for CuriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cm252
    }
}
impl From<CuriumIsotope> for crate::Isotope {
    fn from(isotope: CuriumIsotope) -> Self {
        crate::Isotope::Cm(isotope)
    }
}
impl From<CuriumIsotope> for crate::Element {
    fn from(_isotope: CuriumIsotope) -> Self {
        crate::Element::Cm
    }
}
impl TryFrom<u16> for CuriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            232u16 => Ok(Self::Cm232),
            233u16 => Ok(Self::Cm233),
            234u16 => Ok(Self::Cm234),
            235u16 => Ok(Self::Cm235),
            236u16 => Ok(Self::Cm236),
            237u16 => Ok(Self::Cm237),
            238u16 => Ok(Self::Cm238),
            239u16 => Ok(Self::Cm239),
            240u16 => Ok(Self::Cm240),
            241u16 => Ok(Self::Cm241),
            242u16 => Ok(Self::Cm242),
            243u16 => Ok(Self::Cm243),
            244u16 => Ok(Self::Cm244),
            245u16 => Ok(Self::Cm245),
            246u16 => Ok(Self::Cm246),
            247u16 => Ok(Self::Cm247),
            248u16 => Ok(Self::Cm248),
            249u16 => Ok(Self::Cm249),
            250u16 => Ok(Self::Cm250),
            251u16 => Ok(Self::Cm251),
            252u16 => Ok(Self::Cm252),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cm, value)),
        }
    }
}
impl std::fmt::Display for CuriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cm232 => write!(f, "Cm232"),
            Self::Cm233 => write!(f, "Cm233"),
            Self::Cm234 => write!(f, "Cm234"),
            Self::Cm235 => write!(f, "Cm235"),
            Self::Cm236 => write!(f, "Cm236"),
            Self::Cm237 => write!(f, "Cm237"),
            Self::Cm238 => write!(f, "Cm238"),
            Self::Cm239 => write!(f, "Cm239"),
            Self::Cm240 => write!(f, "Cm240"),
            Self::Cm241 => write!(f, "Cm241"),
            Self::Cm242 => write!(f, "Cm242"),
            Self::Cm243 => write!(f, "Cm243"),
            Self::Cm244 => write!(f, "Cm244"),
            Self::Cm245 => write!(f, "Cm245"),
            Self::Cm246 => write!(f, "Cm246"),
            Self::Cm247 => write!(f, "Cm247"),
            Self::Cm248 => write!(f, "Cm248"),
            Self::Cm249 => write!(f, "Cm249"),
            Self::Cm250 => write!(f, "Cm250"),
            Self::Cm251 => write!(f, "Cm251"),
            Self::Cm252 => write!(f, "Cm252"),
        }
    }
}
