//! Isotopes of the element Nitrogen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Nitrogen
pub enum NitrogenIsotope {
    /// Isotope N10 of Nitrogen
    N10,
    /// Isotope N11 of Nitrogen
    N11,
    /// Isotope N12 of Nitrogen
    N12,
    /// Isotope N13 of Nitrogen
    N13,
    /// Isotope N14 of Nitrogen
    N14,
    /// Isotope N15 of Nitrogen
    N15,
    /// Isotope N16 of Nitrogen
    N16,
    /// Isotope N17 of Nitrogen
    N17,
    /// Isotope N18 of Nitrogen
    N18,
    /// Isotope N19 of Nitrogen
    N19,
    /// Isotope N20 of Nitrogen
    N20,
    /// Isotope N21 of Nitrogen
    N21,
    /// Isotope N22 of Nitrogen
    N22,
    /// Isotope N23 of Nitrogen
    N23,
    /// Isotope N24 of Nitrogen
    N24,
    /// Isotope N25 of Nitrogen
    N25,
}
impl super::RelativeAtomicMass for NitrogenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::N10 => 10.04165f64,
            Self::N11 => 11.026091f64,
            Self::N12 => 12.0186132f64,
            Self::N13 => 13.00573861f64,
            Self::N14 => 14.00307400443f64,
            Self::N15 => 15.00010889888f64,
            Self::N16 => 16.0061019f64,
            Self::N17 => 17.008449f64,
            Self::N18 => 18.014078f64,
            Self::N19 => 19.017022f64,
            Self::N20 => 20.023366f64,
            Self::N21 => 21.02711f64,
            Self::N22 => 22.03439f64,
            Self::N23 => 23.04114f64,
            Self::N24 => 24.05039f64,
            Self::N25 => 25.0601f64,
        }
    }
}
impl super::ElementVariant for NitrogenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::N
    }
}
impl super::MassNumber for NitrogenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::N10 => 10u16,
            Self::N11 => 11u16,
            Self::N12 => 12u16,
            Self::N13 => 13u16,
            Self::N14 => 14u16,
            Self::N15 => 15u16,
            Self::N16 => 16u16,
            Self::N17 => 17u16,
            Self::N18 => 18u16,
            Self::N19 => 19u16,
            Self::N20 => 20u16,
            Self::N21 => 21u16,
            Self::N22 => 22u16,
            Self::N23 => 23u16,
            Self::N24 => 24u16,
            Self::N25 => 25u16,
        }
    }
}
impl super::IsotopicComposition for NitrogenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::N14 => Some(0.99636f64),
            Self::N15 => Some(0.00364f64),
            Self::N10
            | Self::N11
            | Self::N12
            | Self::N13
            | Self::N16
            | Self::N17
            | Self::N18
            | Self::N19
            | Self::N20
            | Self::N21
            | Self::N22
            | Self::N23
            | Self::N24
            | Self::N25 => None,
        }
    }
}
impl super::MostAbundantIsotope for NitrogenIsotope {
    fn most_abundant_isotope() -> Self {
        Self::N14
    }
}
impl From<NitrogenIsotope> for crate::Isotope {
    fn from(isotope: NitrogenIsotope) -> Self {
        crate::Isotope::N(isotope)
    }
}
impl From<NitrogenIsotope> for crate::Element {
    fn from(_isotope: NitrogenIsotope) -> Self {
        crate::Element::N
    }
}
impl TryFrom<u16> for NitrogenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            10u16 => Ok(Self::N10),
            11u16 => Ok(Self::N11),
            12u16 => Ok(Self::N12),
            13u16 => Ok(Self::N13),
            14u16 => Ok(Self::N14),
            15u16 => Ok(Self::N15),
            16u16 => Ok(Self::N16),
            17u16 => Ok(Self::N17),
            18u16 => Ok(Self::N18),
            19u16 => Ok(Self::N19),
            20u16 => Ok(Self::N20),
            21u16 => Ok(Self::N21),
            22u16 => Ok(Self::N22),
            23u16 => Ok(Self::N23),
            24u16 => Ok(Self::N24),
            25u16 => Ok(Self::N25),
            _ => Err(crate::errors::Error::Isotope(crate::Element::N, value)),
        }
    }
}
impl std::fmt::Display for NitrogenIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N10 => write!(f, "N10"),
            Self::N11 => write!(f, "N11"),
            Self::N12 => write!(f, "N12"),
            Self::N13 => write!(f, "N13"),
            Self::N14 => write!(f, "N14"),
            Self::N15 => write!(f, "N15"),
            Self::N16 => write!(f, "N16"),
            Self::N17 => write!(f, "N17"),
            Self::N18 => write!(f, "N18"),
            Self::N19 => write!(f, "N19"),
            Self::N20 => write!(f, "N20"),
            Self::N21 => write!(f, "N21"),
            Self::N22 => write!(f, "N22"),
            Self::N23 => write!(f, "N23"),
            Self::N24 => write!(f, "N24"),
            Self::N25 => write!(f, "N25"),
        }
    }
}
