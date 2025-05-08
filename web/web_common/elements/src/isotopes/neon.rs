#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum NeonIsotope {
    Ne16,
    Ne17,
    Ne18,
    Ne19,
    Ne20,
    Ne21,
    Ne22,
    Ne23,
    Ne24,
    Ne25,
    Ne26,
    Ne27,
    Ne28,
    Ne29,
    Ne30,
    Ne31,
    Ne32,
    Ne33,
    Ne34,
}
impl super::RelativeAtomicMass for NeonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ne16 => 16.02575f64,
            Self::Ne17 => 17.01771396f64,
            Self::Ne18 => 18.0057087f64,
            Self::Ne19 => 19.00188091f64,
            Self::Ne20 => 19.9924401762f64,
            Self::Ne21 => 20.993846685f64,
            Self::Ne22 => 21.991385114f64,
            Self::Ne23 => 22.99446691f64,
            Self::Ne24 => 23.99361065f64,
            Self::Ne25 => 24.997789f64,
            Self::Ne26 => 26.000515f64,
            Self::Ne27 => 27.007553f64,
            Self::Ne28 => 28.01212f64,
            Self::Ne29 => 29.01975f64,
            Self::Ne30 => 30.02473f64,
            Self::Ne31 => 31.0331f64,
            Self::Ne32 => 32.03972f64,
            Self::Ne33 => 33.04938f64,
            Self::Ne34 => 34.05673f64,
        }
    }
}
impl super::ElementVariant for NeonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ne
    }
}
impl super::MassNumber for NeonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ne16 => 16u16,
            Self::Ne17 => 17u16,
            Self::Ne18 => 18u16,
            Self::Ne19 => 19u16,
            Self::Ne20 => 20u16,
            Self::Ne21 => 21u16,
            Self::Ne22 => 22u16,
            Self::Ne23 => 23u16,
            Self::Ne24 => 24u16,
            Self::Ne25 => 25u16,
            Self::Ne26 => 26u16,
            Self::Ne27 => 27u16,
            Self::Ne28 => 28u16,
            Self::Ne29 => 29u16,
            Self::Ne30 => 30u16,
            Self::Ne31 => 31u16,
            Self::Ne32 => 32u16,
            Self::Ne33 => 33u16,
            Self::Ne34 => 34u16,
        }
    }
}
impl super::IsotopicComposition for NeonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ne16 => None,
            Self::Ne17 => None,
            Self::Ne18 => None,
            Self::Ne19 => None,
            Self::Ne20 => Some(0.9048f64),
            Self::Ne21 => Some(0.0027f64),
            Self::Ne22 => Some(0.0925f64),
            Self::Ne23 => None,
            Self::Ne24 => None,
            Self::Ne25 => None,
            Self::Ne26 => None,
            Self::Ne27 => None,
            Self::Ne28 => None,
            Self::Ne29 => None,
            Self::Ne30 => None,
            Self::Ne31 => None,
            Self::Ne32 => None,
            Self::Ne33 => None,
            Self::Ne34 => None,
        }
    }
}
impl super::MostCommonIsotope for NeonIsotope {
    fn most_common_isotope() -> Self {
        Self::Ne20
    }
}
