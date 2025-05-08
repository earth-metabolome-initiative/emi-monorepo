#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum RutherfordiumIsotope {
    Rf253,
    Rf254,
    Rf255,
    Rf256,
    Rf257,
    Rf258,
    Rf259,
    Rf260,
    Rf261,
    Rf262,
    Rf263,
    Rf264,
    Rf265,
    Rf266,
    Rf267,
    Rf268,
}
impl super::RelativeAtomicMass for RutherfordiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rf253 => 253.10044f64,
            Self::Rf254 => 254.10005f64,
            Self::Rf255 => 255.10127f64,
            Self::Rf256 => 256.101152f64,
            Self::Rf257 => 257.102918f64,
            Self::Rf258 => 258.103428f64,
            Self::Rf259 => 259.105596f64,
            Self::Rf260 => 260.10644f64,
            Self::Rf261 => 261.108773f64,
            Self::Rf262 => 262.10992f64,
            Self::Rf263 => 263.11249f64,
            Self::Rf264 => 264.11388f64,
            Self::Rf265 => 265.11668f64,
            Self::Rf266 => 266.11817f64,
            Self::Rf267 => 267.12179f64,
            Self::Rf268 => 268.12397f64,
        }
    }
}
impl super::ElementVariant for RutherfordiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Rf
    }
}
impl super::MassNumber for RutherfordiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rf253 => 253u16,
            Self::Rf254 => 254u16,
            Self::Rf255 => 255u16,
            Self::Rf256 => 256u16,
            Self::Rf257 => 257u16,
            Self::Rf258 => 258u16,
            Self::Rf259 => 259u16,
            Self::Rf260 => 260u16,
            Self::Rf261 => 261u16,
            Self::Rf262 => 262u16,
            Self::Rf263 => 263u16,
            Self::Rf264 => 264u16,
            Self::Rf265 => 265u16,
            Self::Rf266 => 266u16,
            Self::Rf267 => 267u16,
            Self::Rf268 => 268u16,
        }
    }
}
impl super::IsotopicComposition for RutherfordiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Rf253 => None,
            Self::Rf254 => None,
            Self::Rf255 => None,
            Self::Rf256 => None,
            Self::Rf257 => None,
            Self::Rf258 => None,
            Self::Rf259 => None,
            Self::Rf260 => None,
            Self::Rf261 => None,
            Self::Rf262 => None,
            Self::Rf263 => None,
            Self::Rf264 => None,
            Self::Rf265 => None,
            Self::Rf266 => None,
            Self::Rf267 => None,
            Self::Rf268 => None,
        }
    }
}
impl super::MostCommonIsotope for RutherfordiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Rf268
    }
}
