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
impl super::MostAbundantIsotope for LawrenciumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Lr266
    }
}
impl TryFrom<u16> for LawrenciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            251u16 => Ok(Self::Lr251),
            252u16 => Ok(Self::Lr252),
            253u16 => Ok(Self::Lr253),
            254u16 => Ok(Self::Lr254),
            255u16 => Ok(Self::Lr255),
            256u16 => Ok(Self::Lr256),
            257u16 => Ok(Self::Lr257),
            258u16 => Ok(Self::Lr258),
            259u16 => Ok(Self::Lr259),
            260u16 => Ok(Self::Lr260),
            261u16 => Ok(Self::Lr261),
            262u16 => Ok(Self::Lr262),
            263u16 => Ok(Self::Lr263),
            264u16 => Ok(Self::Lr264),
            265u16 => Ok(Self::Lr265),
            266u16 => Ok(Self::Lr266),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Lr, value)),
        }
    }
}
impl std::fmt::Display for LawrenciumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lr251 => write!(f, "Lr251"),
            Self::Lr252 => write!(f, "Lr252"),
            Self::Lr253 => write!(f, "Lr253"),
            Self::Lr254 => write!(f, "Lr254"),
            Self::Lr255 => write!(f, "Lr255"),
            Self::Lr256 => write!(f, "Lr256"),
            Self::Lr257 => write!(f, "Lr257"),
            Self::Lr258 => write!(f, "Lr258"),
            Self::Lr259 => write!(f, "Lr259"),
            Self::Lr260 => write!(f, "Lr260"),
            Self::Lr261 => write!(f, "Lr261"),
            Self::Lr262 => write!(f, "Lr262"),
            Self::Lr263 => write!(f, "Lr263"),
            Self::Lr264 => write!(f, "Lr264"),
            Self::Lr265 => write!(f, "Lr265"),
            Self::Lr266 => write!(f, "Lr266"),
        }
    }
}
