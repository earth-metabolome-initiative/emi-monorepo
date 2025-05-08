#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum EinsteiniumIsotope {
    Es239,
    Es240,
    Es241,
    Es242,
    Es243,
    Es244,
    Es245,
    Es246,
    Es247,
    Es248,
    Es249,
    Es250,
    Es251,
    Es252,
    Es253,
    Es254,
    Es255,
    Es256,
    Es257,
    Es258,
}
impl super::RelativeAtomicMass for EinsteiniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Es239 => 239.06823f64,
            Self::Es240 => 240.06892f64,
            Self::Es241 => 241.06856f64,
            Self::Es242 => 242.06957f64,
            Self::Es243 => 243.06951f64,
            Self::Es244 => 244.07088f64,
            Self::Es245 => 245.07125f64,
            Self::Es246 => 246.0729f64,
            Self::Es247 => 247.073622f64,
            Self::Es248 => 248.075471f64,
            Self::Es249 => 249.076411f64,
            Self::Es250 => 250.07861f64,
            Self::Es251 => 251.0799936f64,
            Self::Es252 => 252.08298f64,
            Self::Es253 => 253.0848257f64,
            Self::Es254 => 254.0880222f64,
            Self::Es255 => 255.090275f64,
            Self::Es256 => 256.0936f64,
            Self::Es257 => 257.09598f64,
            Self::Es258 => 258.09952f64,
        }
    }
}
impl super::ElementVariant for EinsteiniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Es
    }
}
impl super::MassNumber for EinsteiniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Es239 => 239u16,
            Self::Es240 => 240u16,
            Self::Es241 => 241u16,
            Self::Es242 => 242u16,
            Self::Es243 => 243u16,
            Self::Es244 => 244u16,
            Self::Es245 => 245u16,
            Self::Es246 => 246u16,
            Self::Es247 => 247u16,
            Self::Es248 => 248u16,
            Self::Es249 => 249u16,
            Self::Es250 => 250u16,
            Self::Es251 => 251u16,
            Self::Es252 => 252u16,
            Self::Es253 => 253u16,
            Self::Es254 => 254u16,
            Self::Es255 => 255u16,
            Self::Es256 => 256u16,
            Self::Es257 => 257u16,
            Self::Es258 => 258u16,
        }
    }
}
impl super::IsotopicComposition for EinsteiniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Es239 => None,
            Self::Es240 => None,
            Self::Es241 => None,
            Self::Es242 => None,
            Self::Es243 => None,
            Self::Es244 => None,
            Self::Es245 => None,
            Self::Es246 => None,
            Self::Es247 => None,
            Self::Es248 => None,
            Self::Es249 => None,
            Self::Es250 => None,
            Self::Es251 => None,
            Self::Es252 => None,
            Self::Es253 => None,
            Self::Es254 => None,
            Self::Es255 => None,
            Self::Es256 => None,
            Self::Es257 => None,
            Self::Es258 => None,
        }
    }
}
impl super::MostCommonIsotope for EinsteiniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Es258
    }
}
