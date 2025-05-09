#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum PhosphorusIsotope {
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
    P31,
    P32,
    P33,
    P34,
    P35,
    P36,
    P37,
    P38,
    P39,
    P40,
    P41,
    P42,
    P43,
    P44,
    P45,
    P46,
    P47,
}
impl super::RelativeAtomicMass for PhosphorusIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::P24 => 24.03577f64,
            Self::P25 => 25.02119f64,
            Self::P26 => 26.01178f64,
            Self::P27 => 26.999224f64,
            Self::P28 => 27.9923266f64,
            Self::P29 => 28.98180079f64,
            Self::P30 => 29.97831375f64,
            Self::P31 => 30.97376199842f64,
            Self::P32 => 31.973907643f64,
            Self::P33 => 32.9717257f64,
            Self::P34 => 33.97364589f64,
            Self::P35 => 34.9733141f64,
            Self::P36 => 35.97826f64,
            Self::P37 => 36.979607f64,
            Self::P38 => 37.984252f64,
            Self::P39 => 38.986227f64,
            Self::P40 => 39.99133f64,
            Self::P41 => 40.994654f64,
            Self::P42 => 42.00108f64,
            Self::P43 => 43.00502f64,
            Self::P44 => 44.01121f64,
            Self::P45 => 45.01645f64,
            Self::P46 => 46.02446f64,
            Self::P47 => 47.03139f64,
        }
    }
}
impl super::ElementVariant for PhosphorusIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::P
    }
}
impl super::MassNumber for PhosphorusIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::P24 => 24u16,
            Self::P25 => 25u16,
            Self::P26 => 26u16,
            Self::P27 => 27u16,
            Self::P28 => 28u16,
            Self::P29 => 29u16,
            Self::P30 => 30u16,
            Self::P31 => 31u16,
            Self::P32 => 32u16,
            Self::P33 => 33u16,
            Self::P34 => 34u16,
            Self::P35 => 35u16,
            Self::P36 => 36u16,
            Self::P37 => 37u16,
            Self::P38 => 38u16,
            Self::P39 => 39u16,
            Self::P40 => 40u16,
            Self::P41 => 41u16,
            Self::P42 => 42u16,
            Self::P43 => 43u16,
            Self::P44 => 44u16,
            Self::P45 => 45u16,
            Self::P46 => 46u16,
            Self::P47 => 47u16,
        }
    }
}
impl super::IsotopicComposition for PhosphorusIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::P24 => None,
            Self::P25 => None,
            Self::P26 => None,
            Self::P27 => None,
            Self::P28 => None,
            Self::P29 => None,
            Self::P30 => None,
            Self::P31 => Some(1f64),
            Self::P32 => None,
            Self::P33 => None,
            Self::P34 => None,
            Self::P35 => None,
            Self::P36 => None,
            Self::P37 => None,
            Self::P38 => None,
            Self::P39 => None,
            Self::P40 => None,
            Self::P41 => None,
            Self::P42 => None,
            Self::P43 => None,
            Self::P44 => None,
            Self::P45 => None,
            Self::P46 => None,
            Self::P47 => None,
        }
    }
}
impl super::MostAbundantIsotope for PhosphorusIsotope {
    fn most_abundant_isotope() -> Self {
        Self::P31
    }
}
impl TryFrom<u16> for PhosphorusIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            24u16 => Ok(Self::P24),
            25u16 => Ok(Self::P25),
            26u16 => Ok(Self::P26),
            27u16 => Ok(Self::P27),
            28u16 => Ok(Self::P28),
            29u16 => Ok(Self::P29),
            30u16 => Ok(Self::P30),
            31u16 => Ok(Self::P31),
            32u16 => Ok(Self::P32),
            33u16 => Ok(Self::P33),
            34u16 => Ok(Self::P34),
            35u16 => Ok(Self::P35),
            36u16 => Ok(Self::P36),
            37u16 => Ok(Self::P37),
            38u16 => Ok(Self::P38),
            39u16 => Ok(Self::P39),
            40u16 => Ok(Self::P40),
            41u16 => Ok(Self::P41),
            42u16 => Ok(Self::P42),
            43u16 => Ok(Self::P43),
            44u16 => Ok(Self::P44),
            45u16 => Ok(Self::P45),
            46u16 => Ok(Self::P46),
            47u16 => Ok(Self::P47),
            _ => Err(crate::errors::Error::Isotope(crate::Element::P, value)),
        }
    }
}
impl std::fmt::Display for PhosphorusIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P24 => write!(f, "P24"),
            Self::P25 => write!(f, "P25"),
            Self::P26 => write!(f, "P26"),
            Self::P27 => write!(f, "P27"),
            Self::P28 => write!(f, "P28"),
            Self::P29 => write!(f, "P29"),
            Self::P30 => write!(f, "P30"),
            Self::P31 => write!(f, "P31"),
            Self::P32 => write!(f, "P32"),
            Self::P33 => write!(f, "P33"),
            Self::P34 => write!(f, "P34"),
            Self::P35 => write!(f, "P35"),
            Self::P36 => write!(f, "P36"),
            Self::P37 => write!(f, "P37"),
            Self::P38 => write!(f, "P38"),
            Self::P39 => write!(f, "P39"),
            Self::P40 => write!(f, "P40"),
            Self::P41 => write!(f, "P41"),
            Self::P42 => write!(f, "P42"),
            Self::P43 => write!(f, "P43"),
            Self::P44 => write!(f, "P44"),
            Self::P45 => write!(f, "P45"),
            Self::P46 => write!(f, "P46"),
            Self::P47 => write!(f, "P47"),
        }
    }
}
