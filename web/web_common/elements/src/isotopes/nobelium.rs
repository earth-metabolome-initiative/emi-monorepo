#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum NobeliumIsotope {
    No248,
    No249,
    No250,
    No251,
    No252,
    No253,
    No254,
    No255,
    No256,
    No257,
    No258,
    No259,
    No260,
    No261,
    No262,
    No263,
    No264,
}
impl super::RelativeAtomicMass for NobeliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::No248 => 248.08655f64,
            Self::No249 => 249.0878f64,
            Self::No250 => 250.08756f64,
            Self::No251 => 251.08894f64,
            Self::No252 => 252.088967f64,
            Self::No253 => 253.0905641f64,
            Self::No254 => 254.090956f64,
            Self::No255 => 255.093191f64,
            Self::No256 => 256.0942829f64,
            Self::No257 => 257.0968878f64,
            Self::No258 => 258.09821f64,
            Self::No259 => 259.10103f64,
            Self::No260 => 260.10264f64,
            Self::No261 => 261.1057f64,
            Self::No262 => 262.10746f64,
            Self::No263 => 263.11071f64,
            Self::No264 => 264.11273f64,
        }
    }
}
impl super::ElementVariant for NobeliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::No
    }
}
impl super::MassNumber for NobeliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::No248 => 248u16,
            Self::No249 => 249u16,
            Self::No250 => 250u16,
            Self::No251 => 251u16,
            Self::No252 => 252u16,
            Self::No253 => 253u16,
            Self::No254 => 254u16,
            Self::No255 => 255u16,
            Self::No256 => 256u16,
            Self::No257 => 257u16,
            Self::No258 => 258u16,
            Self::No259 => 259u16,
            Self::No260 => 260u16,
            Self::No261 => 261u16,
            Self::No262 => 262u16,
            Self::No263 => 263u16,
            Self::No264 => 264u16,
        }
    }
}
impl super::IsotopicComposition for NobeliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::No248 => None,
            Self::No249 => None,
            Self::No250 => None,
            Self::No251 => None,
            Self::No252 => None,
            Self::No253 => None,
            Self::No254 => None,
            Self::No255 => None,
            Self::No256 => None,
            Self::No257 => None,
            Self::No258 => None,
            Self::No259 => None,
            Self::No260 => None,
            Self::No261 => None,
            Self::No262 => None,
            Self::No263 => None,
            Self::No264 => None,
        }
    }
}
impl super::MostCommonIsotope for NobeliumIsotope {
    fn most_common_isotope() -> Self {
        Self::No264
    }
}
