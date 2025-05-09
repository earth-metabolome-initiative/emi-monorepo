#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum CuriumIsotope {
    Cm232,
    Cm233,
    Cm234,
    Cm235,
    Cm236,
    Cm237,
    Cm238,
    Cm239,
    Cm240,
    Cm241,
    Cm242,
    Cm243,
    Cm244,
    Cm245,
    Cm246,
    Cm247,
    Cm248,
    Cm249,
    Cm250,
    Cm251,
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
        match self {
            Self::Cm232 => None,
            Self::Cm233 => None,
            Self::Cm234 => None,
            Self::Cm235 => None,
            Self::Cm236 => None,
            Self::Cm237 => None,
            Self::Cm238 => None,
            Self::Cm239 => None,
            Self::Cm240 => None,
            Self::Cm241 => None,
            Self::Cm242 => None,
            Self::Cm243 => None,
            Self::Cm244 => None,
            Self::Cm245 => None,
            Self::Cm246 => None,
            Self::Cm247 => None,
            Self::Cm248 => None,
            Self::Cm249 => None,
            Self::Cm250 => None,
            Self::Cm251 => None,
            Self::Cm252 => None,
        }
    }
}
impl super::MostAbundantIsotope for CuriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cm252
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
