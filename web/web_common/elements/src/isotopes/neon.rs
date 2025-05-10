//! Isotopes of the element Neon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Neon
pub enum NeonIsotope {
    /// Isotope Ne16 of Neon
    Ne16,
    /// Isotope Ne17 of Neon
    Ne17,
    /// Isotope Ne18 of Neon
    Ne18,
    /// Isotope Ne19 of Neon
    Ne19,
    /// Isotope Ne20 of Neon
    Ne20,
    /// Isotope Ne21 of Neon
    Ne21,
    /// Isotope Ne22 of Neon
    Ne22,
    /// Isotope Ne23 of Neon
    Ne23,
    /// Isotope Ne24 of Neon
    Ne24,
    /// Isotope Ne25 of Neon
    Ne25,
    /// Isotope Ne26 of Neon
    Ne26,
    /// Isotope Ne27 of Neon
    Ne27,
    /// Isotope Ne28 of Neon
    Ne28,
    /// Isotope Ne29 of Neon
    Ne29,
    /// Isotope Ne30 of Neon
    Ne30,
    /// Isotope Ne31 of Neon
    Ne31,
    /// Isotope Ne32 of Neon
    Ne32,
    /// Isotope Ne33 of Neon
    Ne33,
    /// Isotope Ne34 of Neon
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
            Self::Ne20 => Some(0.9048f64),
            Self::Ne21 => Some(0.0027f64),
            Self::Ne22 => Some(0.0925f64),
            Self::Ne16
            | Self::Ne17
            | Self::Ne18
            | Self::Ne19
            | Self::Ne23
            | Self::Ne24
            | Self::Ne25
            | Self::Ne26
            | Self::Ne27
            | Self::Ne28
            | Self::Ne29
            | Self::Ne30
            | Self::Ne31
            | Self::Ne32
            | Self::Ne33
            | Self::Ne34 => None,
        }
    }
}
impl super::MostAbundantIsotope for NeonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ne20
    }
}
impl TryFrom<u16> for NeonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            16u16 => Ok(Self::Ne16),
            17u16 => Ok(Self::Ne17),
            18u16 => Ok(Self::Ne18),
            19u16 => Ok(Self::Ne19),
            20u16 => Ok(Self::Ne20),
            21u16 => Ok(Self::Ne21),
            22u16 => Ok(Self::Ne22),
            23u16 => Ok(Self::Ne23),
            24u16 => Ok(Self::Ne24),
            25u16 => Ok(Self::Ne25),
            26u16 => Ok(Self::Ne26),
            27u16 => Ok(Self::Ne27),
            28u16 => Ok(Self::Ne28),
            29u16 => Ok(Self::Ne29),
            30u16 => Ok(Self::Ne30),
            31u16 => Ok(Self::Ne31),
            32u16 => Ok(Self::Ne32),
            33u16 => Ok(Self::Ne33),
            34u16 => Ok(Self::Ne34),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ne, value)),
        }
    }
}
impl std::fmt::Display for NeonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ne16 => write!(f, "Ne16"),
            Self::Ne17 => write!(f, "Ne17"),
            Self::Ne18 => write!(f, "Ne18"),
            Self::Ne19 => write!(f, "Ne19"),
            Self::Ne20 => write!(f, "Ne20"),
            Self::Ne21 => write!(f, "Ne21"),
            Self::Ne22 => write!(f, "Ne22"),
            Self::Ne23 => write!(f, "Ne23"),
            Self::Ne24 => write!(f, "Ne24"),
            Self::Ne25 => write!(f, "Ne25"),
            Self::Ne26 => write!(f, "Ne26"),
            Self::Ne27 => write!(f, "Ne27"),
            Self::Ne28 => write!(f, "Ne28"),
            Self::Ne29 => write!(f, "Ne29"),
            Self::Ne30 => write!(f, "Ne30"),
            Self::Ne31 => write!(f, "Ne31"),
            Self::Ne32 => write!(f, "Ne32"),
            Self::Ne33 => write!(f, "Ne33"),
            Self::Ne34 => write!(f, "Ne34"),
        }
    }
}
