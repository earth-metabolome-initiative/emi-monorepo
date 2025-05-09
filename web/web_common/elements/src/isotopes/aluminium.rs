#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum AluminiumIsotope {
    Al21,
    Al22,
    Al23,
    Al24,
    Al25,
    Al26,
    Al27,
    Al28,
    Al29,
    Al30,
    Al31,
    Al32,
    Al33,
    Al34,
    Al35,
    Al36,
    Al37,
    Al38,
    Al39,
    Al40,
    Al41,
    Al42,
    Al43,
}
impl super::RelativeAtomicMass for AluminiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Al21 => 21.02897f64,
            Self::Al22 => 22.01954f64,
            Self::Al23 => 23.00724435f64,
            Self::Al24 => 23.9999489f64,
            Self::Al25 => 24.9904281f64,
            Self::Al26 => 25.986891904f64,
            Self::Al27 => 26.98153853f64,
            Self::Al28 => 27.98191021f64,
            Self::Al29 => 28.9804565f64,
            Self::Al30 => 29.98296f64,
            Self::Al31 => 30.983945f64,
            Self::Al32 => 31.988085f64,
            Self::Al33 => 32.990909f64,
            Self::Al34 => 33.996705f64,
            Self::Al35 => 34.999764f64,
            Self::Al36 => 36.00639f64,
            Self::Al37 => 37.01053f64,
            Self::Al38 => 38.0174f64,
            Self::Al39 => 39.02254f64,
            Self::Al40 => 40.03003f64,
            Self::Al41 => 41.03638f64,
            Self::Al42 => 42.04384f64,
            Self::Al43 => 43.05147f64,
        }
    }
}
impl super::ElementVariant for AluminiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Al
    }
}
impl super::MassNumber for AluminiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Al21 => 21u16,
            Self::Al22 => 22u16,
            Self::Al23 => 23u16,
            Self::Al24 => 24u16,
            Self::Al25 => 25u16,
            Self::Al26 => 26u16,
            Self::Al27 => 27u16,
            Self::Al28 => 28u16,
            Self::Al29 => 29u16,
            Self::Al30 => 30u16,
            Self::Al31 => 31u16,
            Self::Al32 => 32u16,
            Self::Al33 => 33u16,
            Self::Al34 => 34u16,
            Self::Al35 => 35u16,
            Self::Al36 => 36u16,
            Self::Al37 => 37u16,
            Self::Al38 => 38u16,
            Self::Al39 => 39u16,
            Self::Al40 => 40u16,
            Self::Al41 => 41u16,
            Self::Al42 => 42u16,
            Self::Al43 => 43u16,
        }
    }
}
impl super::IsotopicComposition for AluminiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Al21 => None,
            Self::Al22 => None,
            Self::Al23 => None,
            Self::Al24 => None,
            Self::Al25 => None,
            Self::Al26 => None,
            Self::Al27 => Some(1f64),
            Self::Al28 => None,
            Self::Al29 => None,
            Self::Al30 => None,
            Self::Al31 => None,
            Self::Al32 => None,
            Self::Al33 => None,
            Self::Al34 => None,
            Self::Al35 => None,
            Self::Al36 => None,
            Self::Al37 => None,
            Self::Al38 => None,
            Self::Al39 => None,
            Self::Al40 => None,
            Self::Al41 => None,
            Self::Al42 => None,
            Self::Al43 => None,
        }
    }
}
impl super::MostAbundantIsotope for AluminiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Al27
    }
}
impl std::fmt::Display for AluminiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Al21 => write!(f, "Al21"),
            Self::Al22 => write!(f, "Al22"),
            Self::Al23 => write!(f, "Al23"),
            Self::Al24 => write!(f, "Al24"),
            Self::Al25 => write!(f, "Al25"),
            Self::Al26 => write!(f, "Al26"),
            Self::Al27 => write!(f, "Al27"),
            Self::Al28 => write!(f, "Al28"),
            Self::Al29 => write!(f, "Al29"),
            Self::Al30 => write!(f, "Al30"),
            Self::Al31 => write!(f, "Al31"),
            Self::Al32 => write!(f, "Al32"),
            Self::Al33 => write!(f, "Al33"),
            Self::Al34 => write!(f, "Al34"),
            Self::Al35 => write!(f, "Al35"),
            Self::Al36 => write!(f, "Al36"),
            Self::Al37 => write!(f, "Al37"),
            Self::Al38 => write!(f, "Al38"),
            Self::Al39 => write!(f, "Al39"),
            Self::Al40 => write!(f, "Al40"),
            Self::Al41 => write!(f, "Al41"),
            Self::Al42 => write!(f, "Al42"),
            Self::Al43 => write!(f, "Al43"),
        }
    }
}
