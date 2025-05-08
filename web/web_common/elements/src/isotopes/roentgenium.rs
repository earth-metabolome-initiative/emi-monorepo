#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum RoentgeniumIsotope {
    Rg272,
    Rg273,
    Rg274,
    Rg275,
    Rg276,
    Rg277,
    Rg278,
    Rg279,
    Rg280,
    Rg281,
    Rg282,
    Rg283,
}
impl super::RelativeAtomicMass for RoentgeniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Rg272 => 272.15327f64,
            Self::Rg273 => 273.15313f64,
            Self::Rg274 => 274.15525f64,
            Self::Rg275 => 275.15594f64,
            Self::Rg276 => 276.15833f64,
            Self::Rg277 => 277.15907f64,
            Self::Rg278 => 278.16149f64,
            Self::Rg279 => 279.16272f64,
            Self::Rg280 => 280.16514f64,
            Self::Rg281 => 281.16636f64,
            Self::Rg282 => 282.16912f64,
            Self::Rg283 => 283.17054f64,
        }
    }
}
impl super::ElementVariant for RoentgeniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Rg
    }
}
impl super::MassNumber for RoentgeniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Rg272 => 272u16,
            Self::Rg273 => 273u16,
            Self::Rg274 => 274u16,
            Self::Rg275 => 275u16,
            Self::Rg276 => 276u16,
            Self::Rg277 => 277u16,
            Self::Rg278 => 278u16,
            Self::Rg279 => 279u16,
            Self::Rg280 => 280u16,
            Self::Rg281 => 281u16,
            Self::Rg282 => 282u16,
            Self::Rg283 => 283u16,
        }
    }
}
impl super::IsotopicComposition for RoentgeniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Rg272 => None,
            Self::Rg273 => None,
            Self::Rg274 => None,
            Self::Rg275 => None,
            Self::Rg276 => None,
            Self::Rg277 => None,
            Self::Rg278 => None,
            Self::Rg279 => None,
            Self::Rg280 => None,
            Self::Rg281 => None,
            Self::Rg282 => None,
            Self::Rg283 => None,
        }
    }
}
impl super::MostCommonIsotope for RoentgeniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Rg283
    }
}
