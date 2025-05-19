//! Isotopes of the element Magnesium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Magnesium
pub enum MagnesiumIsotope {
    /// Isotope Mg19 of Magnesium
    Mg19,
    /// Isotope Mg20 of Magnesium
    Mg20,
    /// Isotope Mg21 of Magnesium
    Mg21,
    /// Isotope Mg22 of Magnesium
    Mg22,
    /// Isotope Mg23 of Magnesium
    Mg23,
    /// Isotope Mg24 of Magnesium
    Mg24,
    /// Isotope Mg25 of Magnesium
    Mg25,
    /// Isotope Mg26 of Magnesium
    Mg26,
    /// Isotope Mg27 of Magnesium
    Mg27,
    /// Isotope Mg28 of Magnesium
    Mg28,
    /// Isotope Mg29 of Magnesium
    Mg29,
    /// Isotope Mg30 of Magnesium
    Mg30,
    /// Isotope Mg31 of Magnesium
    Mg31,
    /// Isotope Mg32 of Magnesium
    Mg32,
    /// Isotope Mg33 of Magnesium
    Mg33,
    /// Isotope Mg34 of Magnesium
    Mg34,
    /// Isotope Mg35 of Magnesium
    Mg35,
    /// Isotope Mg36 of Magnesium
    Mg36,
    /// Isotope Mg37 of Magnesium
    Mg37,
    /// Isotope Mg38 of Magnesium
    Mg38,
    /// Isotope Mg39 of Magnesium
    Mg39,
    /// Isotope Mg40 of Magnesium
    Mg40,
}
impl super::RelativeAtomicMass for MagnesiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mg19 => 19.034169f64,
            Self::Mg20 => 20.01885f64,
            Self::Mg21 => 21.011716f64,
            Self::Mg22 => 21.99957065f64,
            Self::Mg23 => 22.99412421f64,
            Self::Mg24 => 23.985041697f64,
            Self::Mg25 => 24.985836976f64,
            Self::Mg26 => 25.982592968f64,
            Self::Mg27 => 26.984340624f64,
            Self::Mg28 => 27.9838767f64,
            Self::Mg29 => 28.988617f64,
            Self::Mg30 => 29.9904629f64,
            Self::Mg31 => 30.996648f64,
            Self::Mg32 => 31.9991102f64,
            Self::Mg33 => 33.0053271f64,
            Self::Mg34 => 34.008935f64,
            Self::Mg35 => 35.01679f64,
            Self::Mg36 => 36.02188f64,
            Self::Mg37 => 37.03037f64,
            Self::Mg38 => 38.03658f64,
            Self::Mg39 => 39.04538f64,
            Self::Mg40 => 40.05218f64,
        }
    }
}
impl super::ElementVariant for MagnesiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mg
    }
}
impl super::MassNumber for MagnesiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mg19 => 19u16,
            Self::Mg20 => 20u16,
            Self::Mg21 => 21u16,
            Self::Mg22 => 22u16,
            Self::Mg23 => 23u16,
            Self::Mg24 => 24u16,
            Self::Mg25 => 25u16,
            Self::Mg26 => 26u16,
            Self::Mg27 => 27u16,
            Self::Mg28 => 28u16,
            Self::Mg29 => 29u16,
            Self::Mg30 => 30u16,
            Self::Mg31 => 31u16,
            Self::Mg32 => 32u16,
            Self::Mg33 => 33u16,
            Self::Mg34 => 34u16,
            Self::Mg35 => 35u16,
            Self::Mg36 => 36u16,
            Self::Mg37 => 37u16,
            Self::Mg38 => 38u16,
            Self::Mg39 => 39u16,
            Self::Mg40 => 40u16,
        }
    }
}
impl super::IsotopicComposition for MagnesiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mg24 => Some(0.7899f64),
            Self::Mg25 => Some(0.1f64),
            Self::Mg26 => Some(0.1101f64),
            Self::Mg19
            | Self::Mg20
            | Self::Mg21
            | Self::Mg22
            | Self::Mg23
            | Self::Mg27
            | Self::Mg28
            | Self::Mg29
            | Self::Mg30
            | Self::Mg31
            | Self::Mg32
            | Self::Mg33
            | Self::Mg34
            | Self::Mg35
            | Self::Mg36
            | Self::Mg37
            | Self::Mg38
            | Self::Mg39
            | Self::Mg40 => None,
        }
    }
}
impl super::MostAbundantIsotope for MagnesiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mg24
    }
}
impl TryFrom<u16> for MagnesiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            19u16 => Ok(Self::Mg19),
            20u16 => Ok(Self::Mg20),
            21u16 => Ok(Self::Mg21),
            22u16 => Ok(Self::Mg22),
            23u16 => Ok(Self::Mg23),
            24u16 => Ok(Self::Mg24),
            25u16 => Ok(Self::Mg25),
            26u16 => Ok(Self::Mg26),
            27u16 => Ok(Self::Mg27),
            28u16 => Ok(Self::Mg28),
            29u16 => Ok(Self::Mg29),
            30u16 => Ok(Self::Mg30),
            31u16 => Ok(Self::Mg31),
            32u16 => Ok(Self::Mg32),
            33u16 => Ok(Self::Mg33),
            34u16 => Ok(Self::Mg34),
            35u16 => Ok(Self::Mg35),
            36u16 => Ok(Self::Mg36),
            37u16 => Ok(Self::Mg37),
            38u16 => Ok(Self::Mg38),
            39u16 => Ok(Self::Mg39),
            40u16 => Ok(Self::Mg40),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Mg, value)),
        }
    }
}
impl std::fmt::Display for MagnesiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mg19 => write!(f, "Mg19"),
            Self::Mg20 => write!(f, "Mg20"),
            Self::Mg21 => write!(f, "Mg21"),
            Self::Mg22 => write!(f, "Mg22"),
            Self::Mg23 => write!(f, "Mg23"),
            Self::Mg24 => write!(f, "Mg24"),
            Self::Mg25 => write!(f, "Mg25"),
            Self::Mg26 => write!(f, "Mg26"),
            Self::Mg27 => write!(f, "Mg27"),
            Self::Mg28 => write!(f, "Mg28"),
            Self::Mg29 => write!(f, "Mg29"),
            Self::Mg30 => write!(f, "Mg30"),
            Self::Mg31 => write!(f, "Mg31"),
            Self::Mg32 => write!(f, "Mg32"),
            Self::Mg33 => write!(f, "Mg33"),
            Self::Mg34 => write!(f, "Mg34"),
            Self::Mg35 => write!(f, "Mg35"),
            Self::Mg36 => write!(f, "Mg36"),
            Self::Mg37 => write!(f, "Mg37"),
            Self::Mg38 => write!(f, "Mg38"),
            Self::Mg39 => write!(f, "Mg39"),
            Self::Mg40 => write!(f, "Mg40"),
        }
    }
}
