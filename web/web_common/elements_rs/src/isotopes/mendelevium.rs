//! Isotopes of the element Mendelevium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Mendelevium
pub enum MendeleviumIsotope {
    /// Isotope Md245 of Mendelevium
    Md245,
    /// Isotope Md246 of Mendelevium
    Md246,
    /// Isotope Md247 of Mendelevium
    Md247,
    /// Isotope Md248 of Mendelevium
    Md248,
    /// Isotope Md249 of Mendelevium
    Md249,
    /// Isotope Md250 of Mendelevium
    Md250,
    /// Isotope Md251 of Mendelevium
    Md251,
    /// Isotope Md252 of Mendelevium
    Md252,
    /// Isotope Md253 of Mendelevium
    Md253,
    /// Isotope Md254 of Mendelevium
    Md254,
    /// Isotope Md255 of Mendelevium
    Md255,
    /// Isotope Md256 of Mendelevium
    Md256,
    /// Isotope Md257 of Mendelevium
    Md257,
    /// Isotope Md258 of Mendelevium
    Md258,
    /// Isotope Md259 of Mendelevium
    Md259,
    /// Isotope Md260 of Mendelevium
    Md260,
    /// Isotope Md261 of Mendelevium
    Md261,
    /// Isotope Md262 of Mendelevium
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
        None
    }
}
impl super::MostAbundantIsotope for MendeleviumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Md262
    }
}
impl From<MendeleviumIsotope> for crate::Isotope {
    fn from(isotope: MendeleviumIsotope) -> Self {
        crate::Isotope::Md(isotope)
    }
}
impl From<MendeleviumIsotope> for crate::Element {
    fn from(_isotope: MendeleviumIsotope) -> Self {
        crate::Element::Md
    }
}
impl TryFrom<u16> for MendeleviumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            245u16 => Ok(Self::Md245),
            246u16 => Ok(Self::Md246),
            247u16 => Ok(Self::Md247),
            248u16 => Ok(Self::Md248),
            249u16 => Ok(Self::Md249),
            250u16 => Ok(Self::Md250),
            251u16 => Ok(Self::Md251),
            252u16 => Ok(Self::Md252),
            253u16 => Ok(Self::Md253),
            254u16 => Ok(Self::Md254),
            255u16 => Ok(Self::Md255),
            256u16 => Ok(Self::Md256),
            257u16 => Ok(Self::Md257),
            258u16 => Ok(Self::Md258),
            259u16 => Ok(Self::Md259),
            260u16 => Ok(Self::Md260),
            261u16 => Ok(Self::Md261),
            262u16 => Ok(Self::Md262),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Md, value)),
        }
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
