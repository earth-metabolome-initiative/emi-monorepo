#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum BerylliumIsotope {
    Be5,
    Be6,
    Be7,
    Be8,
    Be9,
    Be10,
    Be11,
    Be12,
    Be13,
    Be14,
    Be15,
    Be16,
}
impl super::RelativeAtomicMass for BerylliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Be5 => 5.0399f64,
            Self::Be6 => 6.0197264f64,
            Self::Be7 => 7.016928717f64,
            Self::Be8 => 8.005305102f64,
            Self::Be9 => 9.012183065f64,
            Self::Be10 => 10.013534695f64,
            Self::Be11 => 11.02166108f64,
            Self::Be12 => 12.0269221f64,
            Self::Be13 => 13.036135f64,
            Self::Be14 => 14.04289f64,
            Self::Be15 => 15.05342f64,
            Self::Be16 => 16.06167f64,
        }
    }
}
impl super::ElementVariant for BerylliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Be
    }
}
impl super::MassNumber for BerylliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Be5 => 5u16,
            Self::Be6 => 6u16,
            Self::Be7 => 7u16,
            Self::Be8 => 8u16,
            Self::Be9 => 9u16,
            Self::Be10 => 10u16,
            Self::Be11 => 11u16,
            Self::Be12 => 12u16,
            Self::Be13 => 13u16,
            Self::Be14 => 14u16,
            Self::Be15 => 15u16,
            Self::Be16 => 16u16,
        }
    }
}
impl super::IsotopicComposition for BerylliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Be5 => None,
            Self::Be6 => None,
            Self::Be7 => None,
            Self::Be8 => None,
            Self::Be9 => Some(1f64),
            Self::Be10 => None,
            Self::Be11 => None,
            Self::Be12 => None,
            Self::Be13 => None,
            Self::Be14 => None,
            Self::Be15 => None,
            Self::Be16 => None,
        }
    }
}
impl super::MostCommonIsotope for BerylliumIsotope {
    fn most_common_isotope() -> Self {
        Self::Be9
    }
}
