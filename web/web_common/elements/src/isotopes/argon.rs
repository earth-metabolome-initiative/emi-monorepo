#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ArgonIsotope {
    Ar30,
    Ar31,
    Ar32,
    Ar33,
    Ar34,
    Ar35,
    Ar36,
    Ar37,
    Ar38,
    Ar39,
    Ar40,
    Ar41,
    Ar42,
    Ar43,
    Ar44,
    Ar45,
    Ar46,
    Ar47,
    Ar48,
    Ar49,
    Ar50,
    Ar51,
    Ar52,
    Ar53,
}
impl super::RelativeAtomicMass for ArgonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ar30 => 30.02307f64,
            Self::Ar31 => 31.01212f64,
            Self::Ar32 => 31.9976378f64,
            Self::Ar33 => 32.98992555f64,
            Self::Ar34 => 33.98027009f64,
            Self::Ar35 => 34.97525759f64,
            Self::Ar36 => 35.967545105f64,
            Self::Ar37 => 36.96677633f64,
            Self::Ar38 => 37.96273211f64,
            Self::Ar39 => 38.964313f64,
            Self::Ar40 => 39.9623831237f64,
            Self::Ar41 => 40.96450057f64,
            Self::Ar42 => 41.9630457f64,
            Self::Ar43 => 42.9656361f64,
            Self::Ar44 => 43.9649238f64,
            Self::Ar45 => 44.96803973f64,
            Self::Ar46 => 45.968083f64,
            Self::Ar47 => 46.972935f64,
            Self::Ar48 => 47.97591f64,
            Self::Ar49 => 48.9819f64,
            Self::Ar50 => 49.98613f64,
            Self::Ar51 => 50.9937f64,
            Self::Ar52 => 51.99896f64,
            Self::Ar53 => 53.00729f64,
        }
    }
}
impl super::ElementVariant for ArgonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ar
    }
}
impl super::MassNumber for ArgonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ar30 => 30u16,
            Self::Ar31 => 31u16,
            Self::Ar32 => 32u16,
            Self::Ar33 => 33u16,
            Self::Ar34 => 34u16,
            Self::Ar35 => 35u16,
            Self::Ar36 => 36u16,
            Self::Ar37 => 37u16,
            Self::Ar38 => 38u16,
            Self::Ar39 => 39u16,
            Self::Ar40 => 40u16,
            Self::Ar41 => 41u16,
            Self::Ar42 => 42u16,
            Self::Ar43 => 43u16,
            Self::Ar44 => 44u16,
            Self::Ar45 => 45u16,
            Self::Ar46 => 46u16,
            Self::Ar47 => 47u16,
            Self::Ar48 => 48u16,
            Self::Ar49 => 49u16,
            Self::Ar50 => 50u16,
            Self::Ar51 => 51u16,
            Self::Ar52 => 52u16,
            Self::Ar53 => 53u16,
        }
    }
}
impl super::IsotopicComposition for ArgonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ar30 => None,
            Self::Ar31 => None,
            Self::Ar32 => None,
            Self::Ar33 => None,
            Self::Ar34 => None,
            Self::Ar35 => None,
            Self::Ar36 => Some(0.003336f64),
            Self::Ar37 => None,
            Self::Ar38 => Some(0.000629f64),
            Self::Ar39 => None,
            Self::Ar40 => Some(0.996035f64),
            Self::Ar41 => None,
            Self::Ar42 => None,
            Self::Ar43 => None,
            Self::Ar44 => None,
            Self::Ar45 => None,
            Self::Ar46 => None,
            Self::Ar47 => None,
            Self::Ar48 => None,
            Self::Ar49 => None,
            Self::Ar50 => None,
            Self::Ar51 => None,
            Self::Ar52 => None,
            Self::Ar53 => None,
        }
    }
}
impl super::MostCommonIsotope for ArgonIsotope {
    fn most_common_isotope() -> Self {
        Self::Ar40
    }
}
