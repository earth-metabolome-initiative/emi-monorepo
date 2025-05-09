#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ManganeseIsotope {
    Mn44,
    Mn45,
    Mn46,
    Mn47,
    Mn48,
    Mn49,
    Mn50,
    Mn51,
    Mn52,
    Mn53,
    Mn54,
    Mn55,
    Mn56,
    Mn57,
    Mn58,
    Mn59,
    Mn60,
    Mn61,
    Mn62,
    Mn63,
    Mn64,
    Mn65,
    Mn66,
    Mn67,
    Mn68,
    Mn69,
    Mn70,
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
            Self::Mn44 => None,
            Self::Mn45 => None,
            Self::Mn46 => None,
            Self::Mn47 => None,
            Self::Mn48 => None,
            Self::Mn49 => None,
            Self::Mn50 => None,
            Self::Mn51 => None,
            Self::Mn52 => None,
            Self::Mn53 => None,
            Self::Mn54 => None,
            Self::Mn55 => Some(1f64),
            Self::Mn56 => None,
            Self::Mn57 => None,
            Self::Mn58 => None,
            Self::Mn59 => None,
            Self::Mn60 => None,
            Self::Mn61 => None,
            Self::Mn62 => None,
            Self::Mn63 => None,
            Self::Mn64 => None,
            Self::Mn65 => None,
            Self::Mn66 => None,
            Self::Mn67 => None,
            Self::Mn68 => None,
            Self::Mn69 => None,
            Self::Mn70 => None,
            Self::Mn71 => None,
        }
    }
}
impl super::MostAbundantIsotope for ManganeseIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Mn55
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
