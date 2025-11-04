//! Isotopes of the element Rutherfordium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Rutherfordium
pub enum RutherfordiumIsotope {
    /// Isotope Rf253 of Rutherfordium
    Rf253,
    /// Isotope Rf254 of Rutherfordium
    Rf254,
    /// Isotope Rf255 of Rutherfordium
    Rf255,
    /// Isotope Rf256 of Rutherfordium
    Rf256,
    /// Isotope Rf257 of Rutherfordium
    Rf257,
    /// Isotope Rf258 of Rutherfordium
    Rf258,
    /// Isotope Rf259 of Rutherfordium
    Rf259,
    /// Isotope Rf260 of Rutherfordium
    Rf260,
    /// Isotope Rf261 of Rutherfordium
    Rf261,
    /// Isotope Rf262 of Rutherfordium
    Rf262,
    /// Isotope Rf263 of Rutherfordium
    Rf263,
    /// Isotope Rf264 of Rutherfordium
    Rf264,
    /// Isotope Rf265 of Rutherfordium
    Rf265,
    /// Isotope Rf266 of Rutherfordium
    Rf266,
    /// Isotope Rf267 of Rutherfordium
    Rf267,
    /// Isotope Rf268 of Rutherfordium
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
        None
    }
}
impl super::MostAbundantIsotope for RutherfordiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Rf268
    }
}
impl From<RutherfordiumIsotope> for crate::Isotope {
    fn from(isotope: RutherfordiumIsotope) -> Self {
        crate::Isotope::Rf(isotope)
    }
}
impl From<RutherfordiumIsotope> for crate::Element {
    fn from(_isotope: RutherfordiumIsotope) -> Self {
        crate::Element::Rf
    }
}
impl TryFrom<u16> for RutherfordiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            253u16 => Ok(Self::Rf253),
            254u16 => Ok(Self::Rf254),
            255u16 => Ok(Self::Rf255),
            256u16 => Ok(Self::Rf256),
            257u16 => Ok(Self::Rf257),
            258u16 => Ok(Self::Rf258),
            259u16 => Ok(Self::Rf259),
            260u16 => Ok(Self::Rf260),
            261u16 => Ok(Self::Rf261),
            262u16 => Ok(Self::Rf262),
            263u16 => Ok(Self::Rf263),
            264u16 => Ok(Self::Rf264),
            265u16 => Ok(Self::Rf265),
            266u16 => Ok(Self::Rf266),
            267u16 => Ok(Self::Rf267),
            268u16 => Ok(Self::Rf268),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Rf, value)),
        }
    }
}
impl std::fmt::Display for RutherfordiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rf253 => write!(f, "Rf253"),
            Self::Rf254 => write!(f, "Rf254"),
            Self::Rf255 => write!(f, "Rf255"),
            Self::Rf256 => write!(f, "Rf256"),
            Self::Rf257 => write!(f, "Rf257"),
            Self::Rf258 => write!(f, "Rf258"),
            Self::Rf259 => write!(f, "Rf259"),
            Self::Rf260 => write!(f, "Rf260"),
            Self::Rf261 => write!(f, "Rf261"),
            Self::Rf262 => write!(f, "Rf262"),
            Self::Rf263 => write!(f, "Rf263"),
            Self::Rf264 => write!(f, "Rf264"),
            Self::Rf265 => write!(f, "Rf265"),
            Self::Rf266 => write!(f, "Rf266"),
            Self::Rf267 => write!(f, "Rf267"),
            Self::Rf268 => write!(f, "Rf268"),
        }
    }
}
