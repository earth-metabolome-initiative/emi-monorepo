//! Isotopes of the element Manganese
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Manganese
pub enum ManganeseIsotope {
    /// Isotope Mn44 of Manganese
    Mn44,
    /// Isotope Mn45 of Manganese
    Mn45,
    /// Isotope Mn46 of Manganese
    Mn46,
    /// Isotope Mn47 of Manganese
    Mn47,
    /// Isotope Mn48 of Manganese
    Mn48,
    /// Isotope Mn49 of Manganese
    Mn49,
    /// Isotope Mn50 of Manganese
    Mn50,
    /// Isotope Mn51 of Manganese
    Mn51,
    /// Isotope Mn52 of Manganese
    Mn52,
    /// Isotope Mn53 of Manganese
    Mn53,
    /// Isotope Mn54 of Manganese
    Mn54,
    /// Isotope Mn55 of Manganese
    Mn55,
    /// Isotope Mn56 of Manganese
    Mn56,
    /// Isotope Mn57 of Manganese
    Mn57,
    /// Isotope Mn58 of Manganese
    Mn58,
    /// Isotope Mn59 of Manganese
    Mn59,
    /// Isotope Mn60 of Manganese
    Mn60,
    /// Isotope Mn61 of Manganese
    Mn61,
    /// Isotope Mn62 of Manganese
    Mn62,
    /// Isotope Mn63 of Manganese
    Mn63,
    /// Isotope Mn64 of Manganese
    Mn64,
    /// Isotope Mn65 of Manganese
    Mn65,
    /// Isotope Mn66 of Manganese
    Mn66,
    /// Isotope Mn67 of Manganese
    Mn67,
    /// Isotope Mn68 of Manganese
    Mn68,
    /// Isotope Mn69 of Manganese
    Mn69,
    /// Isotope Mn70 of Manganese
    Mn70,
    /// Isotope Mn71 of Manganese
    Mn71,
}
impl super::RelativeAtomicMass for ManganeseIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Mn44 => 44.00715f64,
            Self::Mn45 => 44.99449f64,
            Self::Mn46 => 45.98609f64,
            Self::Mn47 => 46.975775f64,
            Self::Mn48 => 47.96852f64,
            Self::Mn49 => 48.959595f64,
            Self::Mn50 => 49.95423778f64,
            Self::Mn51 => 50.94820847f64,
            Self::Mn52 => 51.9455639f64,
            Self::Mn53 => 52.94128889f64,
            Self::Mn54 => 53.9403576f64,
            Self::Mn55 => 54.93804391f64,
            Self::Mn56 => 55.93890369f64,
            Self::Mn57 => 56.9382861f64,
            Self::Mn58 => 57.9400666f64,
            Self::Mn59 => 58.9403911f64,
            Self::Mn60 => 59.9431366f64,
            Self::Mn61 => 60.9444525f64,
            Self::Mn62 => 61.94795f64,
            Self::Mn63 => 62.9496647f64,
            Self::Mn64 => 63.9538494f64,
            Self::Mn65 => 64.9560198f64,
            Self::Mn66 => 65.960547f64,
            Self::Mn67 => 66.96424f64,
            Self::Mn68 => 67.96962f64,
            Self::Mn69 => 68.97366f64,
            Self::Mn70 => 69.97937f64,
            Self::Mn71 => 70.98368f64,
        }
    }
}
impl super::ElementVariant for ManganeseIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Mn
    }
}
impl super::MassNumber for ManganeseIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Mn44 => 44u16,
            Self::Mn45 => 45u16,
            Self::Mn46 => 46u16,
            Self::Mn47 => 47u16,
            Self::Mn48 => 48u16,
            Self::Mn49 => 49u16,
            Self::Mn50 => 50u16,
            Self::Mn51 => 51u16,
            Self::Mn52 => 52u16,
            Self::Mn53 => 53u16,
            Self::Mn54 => 54u16,
            Self::Mn55 => 55u16,
            Self::Mn56 => 56u16,
            Self::Mn57 => 57u16,
            Self::Mn58 => 58u16,
            Self::Mn59 => 59u16,
            Self::Mn60 => 60u16,
            Self::Mn61 => 61u16,
            Self::Mn62 => 62u16,
            Self::Mn63 => 63u16,
            Self::Mn64 => 64u16,
            Self::Mn65 => 65u16,
            Self::Mn66 => 66u16,
            Self::Mn67 => 67u16,
            Self::Mn68 => 68u16,
            Self::Mn69 => 69u16,
            Self::Mn70 => 70u16,
            Self::Mn71 => 71u16,
        }
    }
}
impl super::IsotopicComposition for ManganeseIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Mn55 => Some(1f64),
            Self::Mn44
            | Self::Mn45
            | Self::Mn46
            | Self::Mn47
            | Self::Mn48
            | Self::Mn49
            | Self::Mn50
            | Self::Mn51
            | Self::Mn52
            | Self::Mn53
            | Self::Mn54
            | Self::Mn56
            | Self::Mn57
            | Self::Mn58
            | Self::Mn59
            | Self::Mn60
            | Self::Mn61
            | Self::Mn62
            | Self::Mn63
            | Self::Mn64
            | Self::Mn65
            | Self::Mn66
            | Self::Mn67
            | Self::Mn68
            | Self::Mn69
            | Self::Mn70
            | Self::Mn71 => None,
        }
    }
}
impl super::MostAbundantIsotope for ManganeseIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mn55
    }
}
impl From<ManganeseIsotope> for crate::Isotope {
    fn from(isotope: ManganeseIsotope) -> Self {
        crate::Isotope::Mn(isotope)
    }
}
impl From<ManganeseIsotope> for crate::Element {
    fn from(_isotope: ManganeseIsotope) -> Self {
        crate::Element::Mn
    }
}
impl TryFrom<u16> for ManganeseIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            44u16 => Ok(Self::Mn44),
            45u16 => Ok(Self::Mn45),
            46u16 => Ok(Self::Mn46),
            47u16 => Ok(Self::Mn47),
            48u16 => Ok(Self::Mn48),
            49u16 => Ok(Self::Mn49),
            50u16 => Ok(Self::Mn50),
            51u16 => Ok(Self::Mn51),
            52u16 => Ok(Self::Mn52),
            53u16 => Ok(Self::Mn53),
            54u16 => Ok(Self::Mn54),
            55u16 => Ok(Self::Mn55),
            56u16 => Ok(Self::Mn56),
            57u16 => Ok(Self::Mn57),
            58u16 => Ok(Self::Mn58),
            59u16 => Ok(Self::Mn59),
            60u16 => Ok(Self::Mn60),
            61u16 => Ok(Self::Mn61),
            62u16 => Ok(Self::Mn62),
            63u16 => Ok(Self::Mn63),
            64u16 => Ok(Self::Mn64),
            65u16 => Ok(Self::Mn65),
            66u16 => Ok(Self::Mn66),
            67u16 => Ok(Self::Mn67),
            68u16 => Ok(Self::Mn68),
            69u16 => Ok(Self::Mn69),
            70u16 => Ok(Self::Mn70),
            71u16 => Ok(Self::Mn71),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Mn, value)),
        }
    }
}
impl std::fmt::Display for ManganeseIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mn44 => write!(f, "Mn44"),
            Self::Mn45 => write!(f, "Mn45"),
            Self::Mn46 => write!(f, "Mn46"),
            Self::Mn47 => write!(f, "Mn47"),
            Self::Mn48 => write!(f, "Mn48"),
            Self::Mn49 => write!(f, "Mn49"),
            Self::Mn50 => write!(f, "Mn50"),
            Self::Mn51 => write!(f, "Mn51"),
            Self::Mn52 => write!(f, "Mn52"),
            Self::Mn53 => write!(f, "Mn53"),
            Self::Mn54 => write!(f, "Mn54"),
            Self::Mn55 => write!(f, "Mn55"),
            Self::Mn56 => write!(f, "Mn56"),
            Self::Mn57 => write!(f, "Mn57"),
            Self::Mn58 => write!(f, "Mn58"),
            Self::Mn59 => write!(f, "Mn59"),
            Self::Mn60 => write!(f, "Mn60"),
            Self::Mn61 => write!(f, "Mn61"),
            Self::Mn62 => write!(f, "Mn62"),
            Self::Mn63 => write!(f, "Mn63"),
            Self::Mn64 => write!(f, "Mn64"),
            Self::Mn65 => write!(f, "Mn65"),
            Self::Mn66 => write!(f, "Mn66"),
            Self::Mn67 => write!(f, "Mn67"),
            Self::Mn68 => write!(f, "Mn68"),
            Self::Mn69 => write!(f, "Mn69"),
            Self::Mn70 => write!(f, "Mn70"),
            Self::Mn71 => write!(f, "Mn71"),
        }
    }
}
