#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum CopperIsotope {
    Cu52,
    Cu53,
    Cu54,
    Cu55,
    Cu56,
    Cu57,
    Cu58,
    Cu59,
    Cu60,
    Cu61,
    Cu62,
    Cu63,
    Cu64,
    Cu65,
    Cu66,
    Cu67,
    Cu68,
    Cu69,
    Cu70,
    Cu71,
    Cu72,
    Cu73,
    Cu74,
    Cu75,
    Cu76,
    Cu77,
    Cu78,
    Cu79,
    Cu80,
    Cu81,
    Cu82,
}
impl super::RelativeAtomicMass for CopperIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cu52 => 51.99671f64,
            Self::Cu53 => 52.98459f64,
            Self::Cu54 => 53.97666f64,
            Self::Cu55 => 54.96604f64,
            Self::Cu56 => 55.95895f64,
            Self::Cu57 => 56.9492125f64,
            Self::Cu58 => 57.94453305f64,
            Self::Cu59 => 58.93949748f64,
            Self::Cu60 => 59.9373645f64,
            Self::Cu61 => 60.9334576f64,
            Self::Cu62 => 61.93259541f64,
            Self::Cu63 => 62.92959772f64,
            Self::Cu64 => 63.92976434f64,
            Self::Cu65 => 64.9277897f64,
            Self::Cu66 => 65.92886903f64,
            Self::Cu67 => 66.9277303f64,
            Self::Cu68 => 67.9296109f64,
            Self::Cu69 => 68.9294293f64,
            Self::Cu70 => 69.9323921f64,
            Self::Cu71 => 70.9326768f64,
            Self::Cu72 => 71.9358203f64,
            Self::Cu73 => 72.9366744f64,
            Self::Cu74 => 73.9398749f64,
            Self::Cu75 => 74.9415226f64,
            Self::Cu76 => 75.945275f64,
            Self::Cu77 => 76.94792f64,
            Self::Cu78 => 77.95223f64,
            Self::Cu79 => 78.95502f64,
            Self::Cu80 => 79.96089f64,
            Self::Cu81 => 80.96587f64,
            Self::Cu82 => 81.97244f64,
        }
    }
}
impl super::ElementVariant for CopperIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cu
    }
}
impl super::MassNumber for CopperIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cu52 => 52u16,
            Self::Cu53 => 53u16,
            Self::Cu54 => 54u16,
            Self::Cu55 => 55u16,
            Self::Cu56 => 56u16,
            Self::Cu57 => 57u16,
            Self::Cu58 => 58u16,
            Self::Cu59 => 59u16,
            Self::Cu60 => 60u16,
            Self::Cu61 => 61u16,
            Self::Cu62 => 62u16,
            Self::Cu63 => 63u16,
            Self::Cu64 => 64u16,
            Self::Cu65 => 65u16,
            Self::Cu66 => 66u16,
            Self::Cu67 => 67u16,
            Self::Cu68 => 68u16,
            Self::Cu69 => 69u16,
            Self::Cu70 => 70u16,
            Self::Cu71 => 71u16,
            Self::Cu72 => 72u16,
            Self::Cu73 => 73u16,
            Self::Cu74 => 74u16,
            Self::Cu75 => 75u16,
            Self::Cu76 => 76u16,
            Self::Cu77 => 77u16,
            Self::Cu78 => 78u16,
            Self::Cu79 => 79u16,
            Self::Cu80 => 80u16,
            Self::Cu81 => 81u16,
            Self::Cu82 => 82u16,
        }
    }
}
impl super::IsotopicComposition for CopperIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cu52 => None,
            Self::Cu53 => None,
            Self::Cu54 => None,
            Self::Cu55 => None,
            Self::Cu56 => None,
            Self::Cu57 => None,
            Self::Cu58 => None,
            Self::Cu59 => None,
            Self::Cu60 => None,
            Self::Cu61 => None,
            Self::Cu62 => None,
            Self::Cu63 => Some(0.6915f64),
            Self::Cu64 => None,
            Self::Cu65 => Some(0.3085f64),
            Self::Cu66 => None,
            Self::Cu67 => None,
            Self::Cu68 => None,
            Self::Cu69 => None,
            Self::Cu70 => None,
            Self::Cu71 => None,
            Self::Cu72 => None,
            Self::Cu73 => None,
            Self::Cu74 => None,
            Self::Cu75 => None,
            Self::Cu76 => None,
            Self::Cu77 => None,
            Self::Cu78 => None,
            Self::Cu79 => None,
            Self::Cu80 => None,
            Self::Cu81 => None,
            Self::Cu82 => None,
        }
    }
}
impl super::MostCommonIsotope for CopperIsotope {
    fn most_common_isotope() -> Self {
        Self::Cu63
    }
}
