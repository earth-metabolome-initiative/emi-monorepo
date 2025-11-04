//! Isotopes of the element Fermium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Fermium
pub enum FermiumIsotope {
    /// Isotope Fm241 of Fermium
    Fm241,
    /// Isotope Fm242 of Fermium
    Fm242,
    /// Isotope Fm243 of Fermium
    Fm243,
    /// Isotope Fm244 of Fermium
    Fm244,
    /// Isotope Fm245 of Fermium
    Fm245,
    /// Isotope Fm246 of Fermium
    Fm246,
    /// Isotope Fm247 of Fermium
    Fm247,
    /// Isotope Fm248 of Fermium
    Fm248,
    /// Isotope Fm249 of Fermium
    Fm249,
    /// Isotope Fm250 of Fermium
    Fm250,
    /// Isotope Fm251 of Fermium
    Fm251,
    /// Isotope Fm252 of Fermium
    Fm252,
    /// Isotope Fm253 of Fermium
    Fm253,
    /// Isotope Fm254 of Fermium
    Fm254,
    /// Isotope Fm255 of Fermium
    Fm255,
    /// Isotope Fm256 of Fermium
    Fm256,
    /// Isotope Fm257 of Fermium
    Fm257,
    /// Isotope Fm258 of Fermium
    Fm258,
    /// Isotope Fm259 of Fermium
    Fm259,
    /// Isotope Fm260 of Fermium
    Fm260,
}
impl super::RelativeAtomicMass for FermiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fm241 => 241.07421f64,
            Self::Fm242 => 242.07343f64,
            Self::Fm243 => 243.07446f64,
            Self::Fm244 => 244.07404f64,
            Self::Fm245 => 245.07535f64,
            Self::Fm246 => 246.07535f64,
            Self::Fm247 => 247.07694f64,
            Self::Fm248 => 248.0771865f64,
            Self::Fm249 => 249.0789275f64,
            Self::Fm250 => 250.079521f64,
            Self::Fm251 => 251.08154f64,
            Self::Fm252 => 252.0824671f64,
            Self::Fm253 => 253.0851846f64,
            Self::Fm254 => 254.0868544f64,
            Self::Fm255 => 255.089964f64,
            Self::Fm256 => 256.0917745f64,
            Self::Fm257 => 257.0951061f64,
            Self::Fm258 => 258.09708f64,
            Self::Fm259 => 259.1006f64,
            Self::Fm260 => 260.10281f64,
        }
    }
}
impl super::ElementVariant for FermiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fm
    }
}
impl super::MassNumber for FermiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fm241 => 241u16,
            Self::Fm242 => 242u16,
            Self::Fm243 => 243u16,
            Self::Fm244 => 244u16,
            Self::Fm245 => 245u16,
            Self::Fm246 => 246u16,
            Self::Fm247 => 247u16,
            Self::Fm248 => 248u16,
            Self::Fm249 => 249u16,
            Self::Fm250 => 250u16,
            Self::Fm251 => 251u16,
            Self::Fm252 => 252u16,
            Self::Fm253 => 253u16,
            Self::Fm254 => 254u16,
            Self::Fm255 => 255u16,
            Self::Fm256 => 256u16,
            Self::Fm257 => 257u16,
            Self::Fm258 => 258u16,
            Self::Fm259 => 259u16,
            Self::Fm260 => 260u16,
        }
    }
}
impl super::IsotopicComposition for FermiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for FermiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Fm260
    }
}
impl From<FermiumIsotope> for crate::Isotope {
    fn from(isotope: FermiumIsotope) -> Self {
        crate::Isotope::Fm(isotope)
    }
}
impl From<FermiumIsotope> for crate::Element {
    fn from(_isotope: FermiumIsotope) -> Self {
        crate::Element::Fm
    }
}
impl TryFrom<u16> for FermiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            241u16 => Ok(Self::Fm241),
            242u16 => Ok(Self::Fm242),
            243u16 => Ok(Self::Fm243),
            244u16 => Ok(Self::Fm244),
            245u16 => Ok(Self::Fm245),
            246u16 => Ok(Self::Fm246),
            247u16 => Ok(Self::Fm247),
            248u16 => Ok(Self::Fm248),
            249u16 => Ok(Self::Fm249),
            250u16 => Ok(Self::Fm250),
            251u16 => Ok(Self::Fm251),
            252u16 => Ok(Self::Fm252),
            253u16 => Ok(Self::Fm253),
            254u16 => Ok(Self::Fm254),
            255u16 => Ok(Self::Fm255),
            256u16 => Ok(Self::Fm256),
            257u16 => Ok(Self::Fm257),
            258u16 => Ok(Self::Fm258),
            259u16 => Ok(Self::Fm259),
            260u16 => Ok(Self::Fm260),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Fm, value)),
        }
    }
}
impl std::fmt::Display for FermiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fm241 => write!(f, "Fm241"),
            Self::Fm242 => write!(f, "Fm242"),
            Self::Fm243 => write!(f, "Fm243"),
            Self::Fm244 => write!(f, "Fm244"),
            Self::Fm245 => write!(f, "Fm245"),
            Self::Fm246 => write!(f, "Fm246"),
            Self::Fm247 => write!(f, "Fm247"),
            Self::Fm248 => write!(f, "Fm248"),
            Self::Fm249 => write!(f, "Fm249"),
            Self::Fm250 => write!(f, "Fm250"),
            Self::Fm251 => write!(f, "Fm251"),
            Self::Fm252 => write!(f, "Fm252"),
            Self::Fm253 => write!(f, "Fm253"),
            Self::Fm254 => write!(f, "Fm254"),
            Self::Fm255 => write!(f, "Fm255"),
            Self::Fm256 => write!(f, "Fm256"),
            Self::Fm257 => write!(f, "Fm257"),
            Self::Fm258 => write!(f, "Fm258"),
            Self::Fm259 => write!(f, "Fm259"),
            Self::Fm260 => write!(f, "Fm260"),
        }
    }
}
