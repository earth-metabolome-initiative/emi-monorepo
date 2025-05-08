#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum LithiumIsotope {
    Li3,
    Li4,
    Li5,
    Li6,
    Li7,
    Li8,
    Li9,
    Li10,
    Li11,
    Li12,
    Li13,
}
impl super::RelativeAtomicMass for LithiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Li3 => 3.0308f64,
            Self::Li4 => 4.02719f64,
            Self::Li5 => 5.012538f64,
            Self::Li6 => 6.0151228874f64,
            Self::Li7 => 7.0160034366f64,
            Self::Li8 => 8.022486246f64,
            Self::Li9 => 9.02679019f64,
            Self::Li10 => 10.035483f64,
            Self::Li11 => 11.04372358f64,
            Self::Li12 => 12.052517f64,
            Self::Li13 => 13.06263f64,
        }
    }
}
impl super::ElementVariant for LithiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Li
    }
}
impl super::MassNumber for LithiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Li3 => 3u16,
            Self::Li4 => 4u16,
            Self::Li5 => 5u16,
            Self::Li6 => 6u16,
            Self::Li7 => 7u16,
            Self::Li8 => 8u16,
            Self::Li9 => 9u16,
            Self::Li10 => 10u16,
            Self::Li11 => 11u16,
            Self::Li12 => 12u16,
            Self::Li13 => 13u16,
        }
    }
}
impl super::IsotopicComposition for LithiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Li3 => None,
            Self::Li4 => None,
            Self::Li5 => None,
            Self::Li6 => Some(0.0759f64),
            Self::Li7 => Some(0.9241f64),
            Self::Li8 => None,
            Self::Li9 => None,
            Self::Li10 => None,
            Self::Li11 => None,
            Self::Li12 => None,
            Self::Li13 => None,
        }
    }
}
impl super::MostCommonIsotope for LithiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Li7
    }
}
