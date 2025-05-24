//! Isotopes of the element Boron
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Boron
pub enum BoronIsotope {
    /// Isotope B6 of Boron
    B6,
    /// Isotope B7 of Boron
    B7,
    /// Isotope B8 of Boron
    B8,
    /// Isotope B9 of Boron
    B9,
    /// Isotope B10 of Boron
    B10,
    /// Isotope B11 of Boron
    B11,
    /// Isotope B12 of Boron
    B12,
    /// Isotope B13 of Boron
    B13,
    /// Isotope B14 of Boron
    B14,
    /// Isotope B15 of Boron
    B15,
    /// Isotope B16 of Boron
    B16,
    /// Isotope B17 of Boron
    B17,
    /// Isotope B18 of Boron
    B18,
    /// Isotope B19 of Boron
    B19,
    /// Isotope B20 of Boron
    B20,
    /// Isotope B21 of Boron
    B21,
}
impl super::RelativeAtomicMass for BoronIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::B6 => 6.0508f64,
            Self::B7 => 7.029712f64,
            Self::B8 => 8.0246073f64,
            Self::B9 => 9.01332965f64,
            Self::B10 => 10.01293695f64,
            Self::B11 => 11.00930536f64,
            Self::B12 => 12.0143527f64,
            Self::B13 => 13.0177802f64,
            Self::B14 => 14.025404f64,
            Self::B15 => 15.031088f64,
            Self::B16 => 16.039842f64,
            Self::B17 => 17.04699f64,
            Self::B18 => 18.05566f64,
            Self::B19 => 19.0631f64,
            Self::B20 => 20.07207f64,
            Self::B21 => 21.08129f64,
        }
    }
}
impl super::ElementVariant for BoronIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::B
    }
}
impl super::MassNumber for BoronIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::B6 => 6u16,
            Self::B7 => 7u16,
            Self::B8 => 8u16,
            Self::B9 => 9u16,
            Self::B10 => 10u16,
            Self::B11 => 11u16,
            Self::B12 => 12u16,
            Self::B13 => 13u16,
            Self::B14 => 14u16,
            Self::B15 => 15u16,
            Self::B16 => 16u16,
            Self::B17 => 17u16,
            Self::B18 => 18u16,
            Self::B19 => 19u16,
            Self::B20 => 20u16,
            Self::B21 => 21u16,
        }
    }
}
impl super::IsotopicComposition for BoronIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::B10 => Some(0.199f64),
            Self::B11 => Some(0.801f64),
            Self::B6
            | Self::B7
            | Self::B8
            | Self::B9
            | Self::B12
            | Self::B13
            | Self::B14
            | Self::B15
            | Self::B16
            | Self::B17
            | Self::B18
            | Self::B19
            | Self::B20
            | Self::B21 => None,
        }
    }
}
impl super::MostAbundantIsotope for BoronIsotope {
    fn most_abundant_isotope() -> Self {
        Self::B11
    }
}
impl From<BoronIsotope> for crate::Isotope {
    fn from(isotope: BoronIsotope) -> Self {
        crate::Isotope::B(isotope)
    }
}
impl From<BoronIsotope> for crate::Element {
    fn from(_isotope: BoronIsotope) -> Self {
        crate::Element::B
    }
}
impl TryFrom<u16> for BoronIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            6u16 => Ok(Self::B6),
            7u16 => Ok(Self::B7),
            8u16 => Ok(Self::B8),
            9u16 => Ok(Self::B9),
            10u16 => Ok(Self::B10),
            11u16 => Ok(Self::B11),
            12u16 => Ok(Self::B12),
            13u16 => Ok(Self::B13),
            14u16 => Ok(Self::B14),
            15u16 => Ok(Self::B15),
            16u16 => Ok(Self::B16),
            17u16 => Ok(Self::B17),
            18u16 => Ok(Self::B18),
            19u16 => Ok(Self::B19),
            20u16 => Ok(Self::B20),
            21u16 => Ok(Self::B21),
            _ => Err(crate::errors::Error::Isotope(crate::Element::B, value)),
        }
    }
}
impl std::fmt::Display for BoronIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B6 => write!(f, "B6"),
            Self::B7 => write!(f, "B7"),
            Self::B8 => write!(f, "B8"),
            Self::B9 => write!(f, "B9"),
            Self::B10 => write!(f, "B10"),
            Self::B11 => write!(f, "B11"),
            Self::B12 => write!(f, "B12"),
            Self::B13 => write!(f, "B13"),
            Self::B14 => write!(f, "B14"),
            Self::B15 => write!(f, "B15"),
            Self::B16 => write!(f, "B16"),
            Self::B17 => write!(f, "B17"),
            Self::B18 => write!(f, "B18"),
            Self::B19 => write!(f, "B19"),
            Self::B20 => write!(f, "B20"),
            Self::B21 => write!(f, "B21"),
        }
    }
}
