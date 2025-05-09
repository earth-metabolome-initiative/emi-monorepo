#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum FluorineIsotope {
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
}
impl super::RelativeAtomicMass for FluorineIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::F14 => 14.034315f64,
            Self::F15 => 15.018043f64,
            Self::F16 => 16.0114657f64,
            Self::F17 => 17.00209524f64,
            Self::F18 => 18.00093733f64,
            Self::F19 => 18.99840316273f64,
            Self::F20 => 19.999981252f64,
            Self::F21 => 20.9999489f64,
            Self::F22 => 22.002999f64,
            Self::F23 => 23.003557f64,
            Self::F24 => 24.008115f64,
            Self::F25 => 25.012199f64,
            Self::F26 => 26.020038f64,
            Self::F27 => 27.02644f64,
            Self::F28 => 28.03534f64,
            Self::F29 => 29.04254f64,
            Self::F30 => 30.05165f64,
            Self::F31 => 31.05971f64,
        }
    }
}
impl super::ElementVariant for FluorineIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::F
    }
}
impl super::MassNumber for FluorineIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::F14 => 14u16,
            Self::F15 => 15u16,
            Self::F16 => 16u16,
            Self::F17 => 17u16,
            Self::F18 => 18u16,
            Self::F19 => 19u16,
            Self::F20 => 20u16,
            Self::F21 => 21u16,
            Self::F22 => 22u16,
            Self::F23 => 23u16,
            Self::F24 => 24u16,
            Self::F25 => 25u16,
            Self::F26 => 26u16,
            Self::F27 => 27u16,
            Self::F28 => 28u16,
            Self::F29 => 29u16,
            Self::F30 => 30u16,
            Self::F31 => 31u16,
        }
    }
}
impl super::IsotopicComposition for FluorineIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::F14 => None,
            Self::F15 => None,
            Self::F16 => None,
            Self::F17 => None,
            Self::F18 => None,
            Self::F19 => Some(1f64),
            Self::F20 => None,
            Self::F21 => None,
            Self::F22 => None,
            Self::F23 => None,
            Self::F24 => None,
            Self::F25 => None,
            Self::F26 => None,
            Self::F27 => None,
            Self::F28 => None,
            Self::F29 => None,
            Self::F30 => None,
            Self::F31 => None,
        }
    }
}
impl super::MostAbundantIsotope for FluorineIsotope {
    fn most_abundant_isotope() -> Self {
        Self::F19
    }
}
impl TryFrom<u16> for FluorineIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            14u16 => Ok(Self::F14),
            15u16 => Ok(Self::F15),
            16u16 => Ok(Self::F16),
            17u16 => Ok(Self::F17),
            18u16 => Ok(Self::F18),
            19u16 => Ok(Self::F19),
            20u16 => Ok(Self::F20),
            21u16 => Ok(Self::F21),
            22u16 => Ok(Self::F22),
            23u16 => Ok(Self::F23),
            24u16 => Ok(Self::F24),
            25u16 => Ok(Self::F25),
            26u16 => Ok(Self::F26),
            27u16 => Ok(Self::F27),
            28u16 => Ok(Self::F28),
            29u16 => Ok(Self::F29),
            30u16 => Ok(Self::F30),
            31u16 => Ok(Self::F31),
            _ => Err(crate::errors::Error::Isotope(crate::Element::F, value)),
        }
    }
}
impl std::fmt::Display for FluorineIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::F14 => write!(f, "F14"),
            Self::F15 => write!(f, "F15"),
            Self::F16 => write!(f, "F16"),
            Self::F17 => write!(f, "F17"),
            Self::F18 => write!(f, "F18"),
            Self::F19 => write!(f, "F19"),
            Self::F20 => write!(f, "F20"),
            Self::F21 => write!(f, "F21"),
            Self::F22 => write!(f, "F22"),
            Self::F23 => write!(f, "F23"),
            Self::F24 => write!(f, "F24"),
            Self::F25 => write!(f, "F25"),
            Self::F26 => write!(f, "F26"),
            Self::F27 => write!(f, "F27"),
            Self::F28 => write!(f, "F28"),
            Self::F29 => write!(f, "F29"),
            Self::F30 => write!(f, "F30"),
            Self::F31 => write!(f, "F31"),
        }
    }
}
