#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum LawrenciumIsotope {
    Lr251,
    Lr252,
    Lr253,
    Lr254,
    Lr255,
    Lr256,
    Lr257,
    Lr258,
    Lr259,
    Lr260,
    Lr261,
    Lr262,
    Lr263,
    Lr264,
    Lr265,
    Lr266,
}
impl super::RelativeAtomicMass for LawrenciumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Lr251 => 251.09418f64,
            Self::Lr252 => 252.09526f64,
            Self::Lr253 => 253.09509f64,
            Self::Lr254 => 254.09648f64,
            Self::Lr255 => 255.096562f64,
            Self::Lr256 => 256.098494f64,
            Self::Lr257 => 257.099418f64,
            Self::Lr258 => 258.10176f64,
            Self::Lr259 => 259.102902f64,
            Self::Lr260 => 260.1055f64,
            Self::Lr261 => 261.10688f64,
            Self::Lr262 => 262.10961f64,
            Self::Lr263 => 263.11136f64,
            Self::Lr264 => 264.1142f64,
            Self::Lr265 => 265.11619f64,
            Self::Lr266 => 266.11983f64,
        }
    }
}
impl super::ElementVariant for LawrenciumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Lr
    }
}
impl super::MassNumber for LawrenciumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Lr251 => 251u16,
            Self::Lr252 => 252u16,
            Self::Lr253 => 253u16,
            Self::Lr254 => 254u16,
            Self::Lr255 => 255u16,
            Self::Lr256 => 256u16,
            Self::Lr257 => 257u16,
            Self::Lr258 => 258u16,
            Self::Lr259 => 259u16,
            Self::Lr260 => 260u16,
            Self::Lr261 => 261u16,
            Self::Lr262 => 262u16,
            Self::Lr263 => 263u16,
            Self::Lr264 => 264u16,
            Self::Lr265 => 265u16,
            Self::Lr266 => 266u16,
        }
    }
}
impl super::IsotopicComposition for LawrenciumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Lr251 => None,
            Self::Lr252 => None,
            Self::Lr253 => None,
            Self::Lr254 => None,
            Self::Lr255 => None,
            Self::Lr256 => None,
            Self::Lr257 => None,
            Self::Lr258 => None,
            Self::Lr259 => None,
            Self::Lr260 => None,
            Self::Lr261 => None,
            Self::Lr262 => None,
            Self::Lr263 => None,
            Self::Lr264 => None,
            Self::Lr265 => None,
            Self::Lr266 => None,
        }
    }
}
impl super::MostCommonIsotope for LawrenciumIsotope {
    fn most_common_isotope() -> Self {
        Self::Lr266
    }
}
