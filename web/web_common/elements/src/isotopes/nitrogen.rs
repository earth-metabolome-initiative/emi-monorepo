#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum NitrogenIsotope {
    N10,
    N11,
    N12,
    N13,
    N14,
    N15,
    N16,
    N17,
    N18,
    N19,
    N20,
    N21,
    N22,
    N23,
    N24,
    N25,
}
impl super::RelativeAtomicMass for NitrogenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::N10 => 10.04165f64,
            Self::N11 => 11.026091f64,
            Self::N12 => 12.0186132f64,
            Self::N13 => 13.00573861f64,
            Self::N14 => 14.00307400443f64,
            Self::N15 => 15.00010889888f64,
            Self::N16 => 16.0061019f64,
            Self::N17 => 17.008449f64,
            Self::N18 => 18.014078f64,
            Self::N19 => 19.017022f64,
            Self::N20 => 20.023366f64,
            Self::N21 => 21.02711f64,
            Self::N22 => 22.03439f64,
            Self::N23 => 23.04114f64,
            Self::N24 => 24.05039f64,
            Self::N25 => 25.0601f64,
        }
    }
}
impl super::ElementVariant for NitrogenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::N
    }
}
impl super::MassNumber for NitrogenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::N10 => 10u16,
            Self::N11 => 11u16,
            Self::N12 => 12u16,
            Self::N13 => 13u16,
            Self::N14 => 14u16,
            Self::N15 => 15u16,
            Self::N16 => 16u16,
            Self::N17 => 17u16,
            Self::N18 => 18u16,
            Self::N19 => 19u16,
            Self::N20 => 20u16,
            Self::N21 => 21u16,
            Self::N22 => 22u16,
            Self::N23 => 23u16,
            Self::N24 => 24u16,
            Self::N25 => 25u16,
        }
    }
}
impl super::IsotopicComposition for NitrogenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::N10 => None,
            Self::N11 => None,
            Self::N12 => None,
            Self::N13 => None,
            Self::N14 => Some(0.99636f64),
            Self::N15 => Some(0.00364f64),
            Self::N16 => None,
            Self::N17 => None,
            Self::N18 => None,
            Self::N19 => None,
            Self::N20 => None,
            Self::N21 => None,
            Self::N22 => None,
            Self::N23 => None,
            Self::N24 => None,
            Self::N25 => None,
        }
    }
}
impl super::MostCommonIsotope for NitrogenIsotope {
    fn most_common_isotope() -> Self {
        Self::N14
    }
}
