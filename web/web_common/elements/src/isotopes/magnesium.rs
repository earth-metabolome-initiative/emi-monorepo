#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum MagnesiumIsotope {
    Mg19,
    Mg20,
    Mg21,
    Mg22,
    Mg23,
    Mg24,
    Mg25,
    Mg26,
    Mg27,
    Mg28,
    Mg29,
    Mg30,
    Mg31,
    Mg32,
    Mg33,
    Mg34,
    Mg35,
    Mg36,
    Mg37,
    Mg38,
    Mg39,
    Mg40,
}
impl super::RelativeAtomicMass for MagnesiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mg19 => 19.034169f64,
            Self::Mg20 => 20.01885f64,
            Self::Mg21 => 21.011716f64,
            Self::Mg22 => 21.99957065f64,
            Self::Mg23 => 22.99412421f64,
            Self::Mg24 => 23.985041697f64,
            Self::Mg25 => 24.985836976f64,
            Self::Mg26 => 25.982592968f64,
            Self::Mg27 => 26.984340624f64,
            Self::Mg28 => 27.9838767f64,
            Self::Mg29 => 28.988617f64,
            Self::Mg30 => 29.9904629f64,
            Self::Mg31 => 30.996648f64,
            Self::Mg32 => 31.9991102f64,
            Self::Mg33 => 33.0053271f64,
            Self::Mg34 => 34.008935f64,
            Self::Mg35 => 35.01679f64,
            Self::Mg36 => 36.02188f64,
            Self::Mg37 => 37.03037f64,
            Self::Mg38 => 38.03658f64,
            Self::Mg39 => 39.04538f64,
            Self::Mg40 => 40.05218f64,
        }
    }
}
impl super::ElementVariant for MagnesiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mg
    }
}
impl super::MassNumber for MagnesiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mg19 => 19u16,
            Self::Mg20 => 20u16,
            Self::Mg21 => 21u16,
            Self::Mg22 => 22u16,
            Self::Mg23 => 23u16,
            Self::Mg24 => 24u16,
            Self::Mg25 => 25u16,
            Self::Mg26 => 26u16,
            Self::Mg27 => 27u16,
            Self::Mg28 => 28u16,
            Self::Mg29 => 29u16,
            Self::Mg30 => 30u16,
            Self::Mg31 => 31u16,
            Self::Mg32 => 32u16,
            Self::Mg33 => 33u16,
            Self::Mg34 => 34u16,
            Self::Mg35 => 35u16,
            Self::Mg36 => 36u16,
            Self::Mg37 => 37u16,
            Self::Mg38 => 38u16,
            Self::Mg39 => 39u16,
            Self::Mg40 => 40u16,
        }
    }
}
impl super::IsotopicComposition for MagnesiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mg19 => None,
            Self::Mg20 => None,
            Self::Mg21 => None,
            Self::Mg22 => None,
            Self::Mg23 => None,
            Self::Mg24 => Some(0.7899f64),
            Self::Mg25 => Some(0.1f64),
            Self::Mg26 => Some(0.1101f64),
            Self::Mg27 => None,
            Self::Mg28 => None,
            Self::Mg29 => None,
            Self::Mg30 => None,
            Self::Mg31 => None,
            Self::Mg32 => None,
            Self::Mg33 => None,
            Self::Mg34 => None,
            Self::Mg35 => None,
            Self::Mg36 => None,
            Self::Mg37 => None,
            Self::Mg38 => None,
            Self::Mg39 => None,
            Self::Mg40 => None,
        }
    }
}
impl super::MostCommonIsotope for MagnesiumIsotope {
    fn most_common_isotope() -> Self {
        Self::Mg24
    }
}
