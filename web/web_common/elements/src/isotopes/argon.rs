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
impl super::MostAbundantIsotope for ArgonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ar40
    }
}
impl TryFrom<u16> for ArgonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            30u16 => Ok(Self::Ar30),
            31u16 => Ok(Self::Ar31),
            32u16 => Ok(Self::Ar32),
            33u16 => Ok(Self::Ar33),
            34u16 => Ok(Self::Ar34),
            35u16 => Ok(Self::Ar35),
            36u16 => Ok(Self::Ar36),
            37u16 => Ok(Self::Ar37),
            38u16 => Ok(Self::Ar38),
            39u16 => Ok(Self::Ar39),
            40u16 => Ok(Self::Ar40),
            41u16 => Ok(Self::Ar41),
            42u16 => Ok(Self::Ar42),
            43u16 => Ok(Self::Ar43),
            44u16 => Ok(Self::Ar44),
            45u16 => Ok(Self::Ar45),
            46u16 => Ok(Self::Ar46),
            47u16 => Ok(Self::Ar47),
            48u16 => Ok(Self::Ar48),
            49u16 => Ok(Self::Ar49),
            50u16 => Ok(Self::Ar50),
            51u16 => Ok(Self::Ar51),
            52u16 => Ok(Self::Ar52),
            53u16 => Ok(Self::Ar53),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ar, value)),
        }
    }
}
impl std::fmt::Display for ArgonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ar30 => write!(f, "Ar30"),
            Self::Ar31 => write!(f, "Ar31"),
            Self::Ar32 => write!(f, "Ar32"),
            Self::Ar33 => write!(f, "Ar33"),
            Self::Ar34 => write!(f, "Ar34"),
            Self::Ar35 => write!(f, "Ar35"),
            Self::Ar36 => write!(f, "Ar36"),
            Self::Ar37 => write!(f, "Ar37"),
            Self::Ar38 => write!(f, "Ar38"),
            Self::Ar39 => write!(f, "Ar39"),
            Self::Ar40 => write!(f, "Ar40"),
            Self::Ar41 => write!(f, "Ar41"),
            Self::Ar42 => write!(f, "Ar42"),
            Self::Ar43 => write!(f, "Ar43"),
            Self::Ar44 => write!(f, "Ar44"),
            Self::Ar45 => write!(f, "Ar45"),
            Self::Ar46 => write!(f, "Ar46"),
            Self::Ar47 => write!(f, "Ar47"),
            Self::Ar48 => write!(f, "Ar48"),
            Self::Ar49 => write!(f, "Ar49"),
            Self::Ar50 => write!(f, "Ar50"),
            Self::Ar51 => write!(f, "Ar51"),
            Self::Ar52 => write!(f, "Ar52"),
            Self::Ar53 => write!(f, "Ar53"),
        }
    }
}
