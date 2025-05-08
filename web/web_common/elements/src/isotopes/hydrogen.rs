#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum HydrogenIsotope {
    H1,
    D2,
    T3,
    H4,
    H5,
    H6,
    H7,
}
impl super::RelativeAtomicMass for HydrogenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::H1 => 1.00782503223f64,
            Self::D2 => 2.01410177812f64,
            Self::T3 => 3.0160492779f64,
            Self::H4 => 4.02643f64,
            Self::H5 => 5.035311f64,
            Self::H6 => 6.04496f64,
            Self::H7 => 7.0527f64,
        }
    }
}
impl super::ElementVariant for HydrogenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::H
    }
}
impl super::MassNumber for HydrogenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::H1 => 1u16,
            Self::D2 => 2u16,
            Self::T3 => 3u16,
            Self::H4 => 4u16,
            Self::H5 => 5u16,
            Self::H6 => 6u16,
            Self::H7 => 7u16,
        }
    }
}
impl super::IsotopicComposition for HydrogenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::H1 => Some(0.999885f64),
            Self::D2 => Some(0.000115f64),
            Self::T3 => None,
            Self::H4 => None,
            Self::H5 => None,
            Self::H6 => None,
            Self::H7 => None,
        }
    }
}
impl super::MostCommonIsotope for HydrogenIsotope {
    fn most_common_isotope() -> Self {
        Self::H1
    }
}
