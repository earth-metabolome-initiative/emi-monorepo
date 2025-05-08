#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum HeliumIsotope {
    He3,
    He4,
    He5,
    He6,
    He7,
    He8,
    He9,
    He10,
}
impl super::RelativeAtomicMass for HeliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::He3 => 3.0160293201f64,
            Self::He4 => 4.00260325413f64,
            Self::He5 => 5.012057f64,
            Self::He6 => 6.018885891f64,
            Self::He7 => 7.0279907f64,
            Self::He8 => 8.03393439f64,
            Self::He9 => 9.043946f64,
            Self::He10 => 10.05279f64,
        }
    }
}
impl super::ElementVariant for HeliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::He
    }
}
impl super::MassNumber for HeliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::He3 => 3u16,
            Self::He4 => 4u16,
            Self::He5 => 5u16,
            Self::He6 => 6u16,
            Self::He7 => 7u16,
            Self::He8 => 8u16,
            Self::He9 => 9u16,
            Self::He10 => 10u16,
        }
    }
}
impl super::IsotopicComposition for HeliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::He3 => Some(0.00000134f64),
            Self::He4 => Some(0.99999866f64),
            Self::He5 => None,
            Self::He6 => None,
            Self::He7 => None,
            Self::He8 => None,
            Self::He9 => None,
            Self::He10 => None,
        }
    }
}
impl super::MostCommonIsotope for HeliumIsotope {
    fn most_common_isotope() -> Self {
        Self::He4
    }
}
