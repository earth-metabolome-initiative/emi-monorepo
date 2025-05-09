#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum SodiumIsotope {
    Na18,
    Na19,
    Na20,
    Na21,
    Na22,
    Na23,
    Na24,
    Na25,
    Na26,
    Na27,
    Na28,
    Na29,
    Na30,
    Na31,
    Na32,
    Na33,
    Na34,
    Na35,
    Na36,
    Na37,
}
impl super::RelativeAtomicMass for SodiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Na18 => 18.02688f64,
            Self::Na19 => 19.01388f64,
            Self::Na20 => 20.0073544f64,
            Self::Na21 => 20.99765469f64,
            Self::Na22 => 21.99443741f64,
            Self::Na23 => 22.989769282f64,
            Self::Na24 => 23.99096295f64,
            Self::Na25 => 24.989954f64,
            Self::Na26 => 25.9926346f64,
            Self::Na27 => 26.9940765f64,
            Self::Na28 => 27.998939f64,
            Self::Na29 => 29.0028771f64,
            Self::Na30 => 30.0090979f64,
            Self::Na31 => 31.013163f64,
            Self::Na32 => 32.02019f64,
            Self::Na33 => 33.02573f64,
            Self::Na34 => 34.03359f64,
            Self::Na35 => 35.04062f64,
            Self::Na36 => 36.04929f64,
            Self::Na37 => 37.05705f64,
        }
    }
}
impl super::ElementVariant for SodiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Na
    }
}
impl super::MassNumber for SodiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Na18 => 18u16,
            Self::Na19 => 19u16,
            Self::Na20 => 20u16,
            Self::Na21 => 21u16,
            Self::Na22 => 22u16,
            Self::Na23 => 23u16,
            Self::Na24 => 24u16,
            Self::Na25 => 25u16,
            Self::Na26 => 26u16,
            Self::Na27 => 27u16,
            Self::Na28 => 28u16,
            Self::Na29 => 29u16,
            Self::Na30 => 30u16,
            Self::Na31 => 31u16,
            Self::Na32 => 32u16,
            Self::Na33 => 33u16,
            Self::Na34 => 34u16,
            Self::Na35 => 35u16,
            Self::Na36 => 36u16,
            Self::Na37 => 37u16,
        }
    }
}
impl super::IsotopicComposition for SodiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Na18 => None,
            Self::Na19 => None,
            Self::Na20 => None,
            Self::Na21 => None,
            Self::Na22 => None,
            Self::Na23 => Some(1f64),
            Self::Na24 => None,
            Self::Na25 => None,
            Self::Na26 => None,
            Self::Na27 => None,
            Self::Na28 => None,
            Self::Na29 => None,
            Self::Na30 => None,
            Self::Na31 => None,
            Self::Na32 => None,
            Self::Na33 => None,
            Self::Na34 => None,
            Self::Na35 => None,
            Self::Na36 => None,
            Self::Na37 => None,
        }
    }
}
impl super::MostAbundantIsotope for SodiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Na23
    }
}
impl TryFrom<u16> for SodiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            18u16 => Ok(Self::Na18),
            19u16 => Ok(Self::Na19),
            20u16 => Ok(Self::Na20),
            21u16 => Ok(Self::Na21),
            22u16 => Ok(Self::Na22),
            23u16 => Ok(Self::Na23),
            24u16 => Ok(Self::Na24),
            25u16 => Ok(Self::Na25),
            26u16 => Ok(Self::Na26),
            27u16 => Ok(Self::Na27),
            28u16 => Ok(Self::Na28),
            29u16 => Ok(Self::Na29),
            30u16 => Ok(Self::Na30),
            31u16 => Ok(Self::Na31),
            32u16 => Ok(Self::Na32),
            33u16 => Ok(Self::Na33),
            34u16 => Ok(Self::Na34),
            35u16 => Ok(Self::Na35),
            36u16 => Ok(Self::Na36),
            37u16 => Ok(Self::Na37),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Na, value)),
        }
    }
}
impl std::fmt::Display for SodiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Na18 => write!(f, "Na18"),
            Self::Na19 => write!(f, "Na19"),
            Self::Na20 => write!(f, "Na20"),
            Self::Na21 => write!(f, "Na21"),
            Self::Na22 => write!(f, "Na22"),
            Self::Na23 => write!(f, "Na23"),
            Self::Na24 => write!(f, "Na24"),
            Self::Na25 => write!(f, "Na25"),
            Self::Na26 => write!(f, "Na26"),
            Self::Na27 => write!(f, "Na27"),
            Self::Na28 => write!(f, "Na28"),
            Self::Na29 => write!(f, "Na29"),
            Self::Na30 => write!(f, "Na30"),
            Self::Na31 => write!(f, "Na31"),
            Self::Na32 => write!(f, "Na32"),
            Self::Na33 => write!(f, "Na33"),
            Self::Na34 => write!(f, "Na34"),
            Self::Na35 => write!(f, "Na35"),
            Self::Na36 => write!(f, "Na36"),
            Self::Na37 => write!(f, "Na37"),
        }
    }
}
