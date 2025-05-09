#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum MendeleviumIsotope {
    Md245,
    Md246,
    Md247,
    Md248,
    Md249,
    Md250,
    Md251,
    Md252,
    Md253,
    Md254,
    Md255,
    Md256,
    Md257,
    Md258,
    Md259,
    Md260,
    Md261,
    Md262,
}
impl super::RelativeAtomicMass for MendeleviumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Md245 => 245.08081f64,
            Self::Md246 => 246.08171f64,
            Self::Md247 => 247.08152f64,
            Self::Md248 => 248.08282f64,
            Self::Md249 => 249.08291f64,
            Self::Md250 => 250.08441f64,
            Self::Md251 => 251.084774f64,
            Self::Md252 => 252.08643f64,
            Self::Md253 => 253.087144f64,
            Self::Md254 => 254.08959f64,
            Self::Md255 => 255.0910841f64,
            Self::Md256 => 256.09389f64,
            Self::Md257 => 257.0955424f64,
            Self::Md258 => 258.0984315f64,
            Self::Md259 => 259.10051f64,
            Self::Md260 => 260.10365f64,
            Self::Md261 => 261.10583f64,
            Self::Md262 => 262.1091f64,
        }
    }
}
impl super::ElementVariant for MendeleviumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Md
    }
}
impl super::MassNumber for MendeleviumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Md245 => 245u16,
            Self::Md246 => 246u16,
            Self::Md247 => 247u16,
            Self::Md248 => 248u16,
            Self::Md249 => 249u16,
            Self::Md250 => 250u16,
            Self::Md251 => 251u16,
            Self::Md252 => 252u16,
            Self::Md253 => 253u16,
            Self::Md254 => 254u16,
            Self::Md255 => 255u16,
            Self::Md256 => 256u16,
            Self::Md257 => 257u16,
            Self::Md258 => 258u16,
            Self::Md259 => 259u16,
            Self::Md260 => 260u16,
            Self::Md261 => 261u16,
            Self::Md262 => 262u16,
        }
    }
}
impl super::IsotopicComposition for MendeleviumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Md245 => None,
            Self::Md246 => None,
            Self::Md247 => None,
            Self::Md248 => None,
            Self::Md249 => None,
            Self::Md250 => None,
            Self::Md251 => None,
            Self::Md252 => None,
            Self::Md253 => None,
            Self::Md254 => None,
            Self::Md255 => None,
            Self::Md256 => None,
            Self::Md257 => None,
            Self::Md258 => None,
            Self::Md259 => None,
            Self::Md260 => None,
            Self::Md261 => None,
            Self::Md262 => None,
        }
    }
}
impl super::MostAbundantIsotope for MendeleviumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Md262
    }
}
impl std::fmt::Display for MendeleviumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Md245 => write!(f, "Md245"),
            Self::Md246 => write!(f, "Md246"),
            Self::Md247 => write!(f, "Md247"),
            Self::Md248 => write!(f, "Md248"),
            Self::Md249 => write!(f, "Md249"),
            Self::Md250 => write!(f, "Md250"),
            Self::Md251 => write!(f, "Md251"),
            Self::Md252 => write!(f, "Md252"),
            Self::Md253 => write!(f, "Md253"),
            Self::Md254 => write!(f, "Md254"),
            Self::Md255 => write!(f, "Md255"),
            Self::Md256 => write!(f, "Md256"),
            Self::Md257 => write!(f, "Md257"),
            Self::Md258 => write!(f, "Md258"),
            Self::Md259 => write!(f, "Md259"),
            Self::Md260 => write!(f, "Md260"),
            Self::Md261 => write!(f, "Md261"),
            Self::Md262 => write!(f, "Md262"),
        }
    }
}
