#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum SulfurIsotope {
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,
    S32,
    S33,
    S34,
    S35,
    S36,
    S37,
    S38,
    S39,
    S40,
    S41,
    S42,
    S43,
    S44,
    S45,
    S46,
    S47,
    S48,
    S49,
}
impl super::RelativeAtomicMass for SulfurIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::S26 => 26.02907f64,
            Self::S27 => 27.01828f64,
            Self::S28 => 28.00437f64,
            Self::S29 => 28.996611f64,
            Self::S30 => 29.98490703f64,
            Self::S31 => 30.97955701f64,
            Self::S32 => 31.9720711744f64,
            Self::S33 => 32.9714589098f64,
            Self::S34 => 33.967867004f64,
            Self::S35 => 34.96903231f64,
            Self::S36 => 35.96708071f64,
            Self::S37 => 36.97112551f64,
            Self::S38 => 37.9711633f64,
            Self::S39 => 38.975134f64,
            Self::S40 => 39.9754826f64,
            Self::S41 => 40.9795935f64,
            Self::S42 => 41.9810651f64,
            Self::S43 => 42.9869076f64,
            Self::S44 => 43.9901188f64,
            Self::S45 => 44.99572f64,
            Self::S46 => 46.00004f64,
            Self::S47 => 47.00795f64,
            Self::S48 => 48.0137f64,
            Self::S49 => 49.02276f64,
        }
    }
}
impl super::ElementVariant for SulfurIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::S
    }
}
impl super::MassNumber for SulfurIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::S26 => 26u16,
            Self::S27 => 27u16,
            Self::S28 => 28u16,
            Self::S29 => 29u16,
            Self::S30 => 30u16,
            Self::S31 => 31u16,
            Self::S32 => 32u16,
            Self::S33 => 33u16,
            Self::S34 => 34u16,
            Self::S35 => 35u16,
            Self::S36 => 36u16,
            Self::S37 => 37u16,
            Self::S38 => 38u16,
            Self::S39 => 39u16,
            Self::S40 => 40u16,
            Self::S41 => 41u16,
            Self::S42 => 42u16,
            Self::S43 => 43u16,
            Self::S44 => 44u16,
            Self::S45 => 45u16,
            Self::S46 => 46u16,
            Self::S47 => 47u16,
            Self::S48 => 48u16,
            Self::S49 => 49u16,
        }
    }
}
impl super::IsotopicComposition for SulfurIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::S26 => None,
            Self::S27 => None,
            Self::S28 => None,
            Self::S29 => None,
            Self::S30 => None,
            Self::S31 => None,
            Self::S32 => Some(0.9499f64),
            Self::S33 => Some(0.0075f64),
            Self::S34 => Some(0.0425f64),
            Self::S35 => None,
            Self::S36 => Some(0.0001f64),
            Self::S37 => None,
            Self::S38 => None,
            Self::S39 => None,
            Self::S40 => None,
            Self::S41 => None,
            Self::S42 => None,
            Self::S43 => None,
            Self::S44 => None,
            Self::S45 => None,
            Self::S46 => None,
            Self::S47 => None,
            Self::S48 => None,
            Self::S49 => None,
        }
    }
}
impl super::MostAbundantIsotope for SulfurIsotope {
    fn most_abundant_isotope() -> Self {
        Self::S32
    }
}
impl std::fmt::Display for SulfurIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S26 => write!(f, "S26"),
            Self::S27 => write!(f, "S27"),
            Self::S28 => write!(f, "S28"),
            Self::S29 => write!(f, "S29"),
            Self::S30 => write!(f, "S30"),
            Self::S31 => write!(f, "S31"),
            Self::S32 => write!(f, "S32"),
            Self::S33 => write!(f, "S33"),
            Self::S34 => write!(f, "S34"),
            Self::S35 => write!(f, "S35"),
            Self::S36 => write!(f, "S36"),
            Self::S37 => write!(f, "S37"),
            Self::S38 => write!(f, "S38"),
            Self::S39 => write!(f, "S39"),
            Self::S40 => write!(f, "S40"),
            Self::S41 => write!(f, "S41"),
            Self::S42 => write!(f, "S42"),
            Self::S43 => write!(f, "S43"),
            Self::S44 => write!(f, "S44"),
            Self::S45 => write!(f, "S45"),
            Self::S46 => write!(f, "S46"),
            Self::S47 => write!(f, "S47"),
            Self::S48 => write!(f, "S48"),
            Self::S49 => write!(f, "S49"),
        }
    }
}
