//! Isotopes of the element Palladium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Palladium
pub enum PalladiumIsotope {
    /// Isotope Pd91 of Palladium
    Pd91,
    /// Isotope Pd92 of Palladium
    Pd92,
    /// Isotope Pd93 of Palladium
    Pd93,
    /// Isotope Pd94 of Palladium
    Pd94,
    /// Isotope Pd95 of Palladium
    Pd95,
    /// Isotope Pd96 of Palladium
    Pd96,
    /// Isotope Pd97 of Palladium
    Pd97,
    /// Isotope Pd98 of Palladium
    Pd98,
    /// Isotope Pd99 of Palladium
    Pd99,
    /// Isotope Pd100 of Palladium
    Pd100,
    /// Isotope Pd101 of Palladium
    Pd101,
    /// Isotope Pd102 of Palladium
    Pd102,
    /// Isotope Pd103 of Palladium
    Pd103,
    /// Isotope Pd104 of Palladium
    Pd104,
    /// Isotope Pd105 of Palladium
    Pd105,
    /// Isotope Pd106 of Palladium
    Pd106,
    /// Isotope Pd107 of Palladium
    Pd107,
    /// Isotope Pd108 of Palladium
    Pd108,
    /// Isotope Pd109 of Palladium
    Pd109,
    /// Isotope Pd110 of Palladium
    Pd110,
    /// Isotope Pd111 of Palladium
    Pd111,
    /// Isotope Pd112 of Palladium
    Pd112,
    /// Isotope Pd113 of Palladium
    Pd113,
    /// Isotope Pd114 of Palladium
    Pd114,
    /// Isotope Pd115 of Palladium
    Pd115,
    /// Isotope Pd116 of Palladium
    Pd116,
    /// Isotope Pd117 of Palladium
    Pd117,
    /// Isotope Pd118 of Palladium
    Pd118,
    /// Isotope Pd119 of Palladium
    Pd119,
    /// Isotope Pd120 of Palladium
    Pd120,
    /// Isotope Pd121 of Palladium
    Pd121,
    /// Isotope Pd122 of Palladium
    Pd122,
    /// Isotope Pd123 of Palladium
    Pd123,
    /// Isotope Pd124 of Palladium
    Pd124,
    /// Isotope Pd125 of Palladium
    Pd125,
    /// Isotope Pd126 of Palladium
    Pd126,
    /// Isotope Pd127 of Palladium
    Pd127,
    /// Isotope Pd128 of Palladium
    Pd128,
}
impl super::RelativeAtomicMass for PalladiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pd91 => 90.95032f64,
            Self::Pd92 => 91.94088f64,
            Self::Pd93 => 92.93651f64,
            Self::Pd94 => 93.9290376f64,
            Self::Pd95 => 94.9248898f64,
            Self::Pd96 => 95.9182151f64,
            Self::Pd97 => 96.916472f64,
            Self::Pd98 => 97.9126983f64,
            Self::Pd99 => 98.9117748f64,
            Self::Pd100 => 99.908505f64,
            Self::Pd101 => 100.9082864f64,
            Self::Pd102 => 101.9056022f64,
            Self::Pd103 => 102.9060809f64,
            Self::Pd104 => 103.9040305f64,
            Self::Pd105 => 104.9050796f64,
            Self::Pd106 => 105.9034804f64,
            Self::Pd107 => 106.9051282f64,
            Self::Pd108 => 107.9038916f64,
            Self::Pd109 => 108.9059504f64,
            Self::Pd110 => 109.9051722f64,
            Self::Pd111 => 110.90768968f64,
            Self::Pd112 => 111.9073297f64,
            Self::Pd113 => 112.910261f64,
            Self::Pd114 => 113.9103686f64,
            Self::Pd115 => 114.913659f64,
            Self::Pd116 => 115.914297f64,
            Self::Pd117 => 116.9179547f64,
            Self::Pd118 => 117.9190667f64,
            Self::Pd119 => 118.9233402f64,
            Self::Pd120 => 119.9245511f64,
            Self::Pd121 => 120.9289503f64,
            Self::Pd122 => 121.930632f64,
            Self::Pd123 => 122.93514f64,
            Self::Pd124 => 123.93714f64,
            Self::Pd125 => 124.94179f64,
            Self::Pd126 => 125.94416f64,
            Self::Pd127 => 126.94907f64,
            Self::Pd128 => 127.95183f64,
        }
    }
}
impl super::ElementVariant for PalladiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pd
    }
}
impl super::MassNumber for PalladiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pd91 => 91u16,
            Self::Pd92 => 92u16,
            Self::Pd93 => 93u16,
            Self::Pd94 => 94u16,
            Self::Pd95 => 95u16,
            Self::Pd96 => 96u16,
            Self::Pd97 => 97u16,
            Self::Pd98 => 98u16,
            Self::Pd99 => 99u16,
            Self::Pd100 => 100u16,
            Self::Pd101 => 101u16,
            Self::Pd102 => 102u16,
            Self::Pd103 => 103u16,
            Self::Pd104 => 104u16,
            Self::Pd105 => 105u16,
            Self::Pd106 => 106u16,
            Self::Pd107 => 107u16,
            Self::Pd108 => 108u16,
            Self::Pd109 => 109u16,
            Self::Pd110 => 110u16,
            Self::Pd111 => 111u16,
            Self::Pd112 => 112u16,
            Self::Pd113 => 113u16,
            Self::Pd114 => 114u16,
            Self::Pd115 => 115u16,
            Self::Pd116 => 116u16,
            Self::Pd117 => 117u16,
            Self::Pd118 => 118u16,
            Self::Pd119 => 119u16,
            Self::Pd120 => 120u16,
            Self::Pd121 => 121u16,
            Self::Pd122 => 122u16,
            Self::Pd123 => 123u16,
            Self::Pd124 => 124u16,
            Self::Pd125 => 125u16,
            Self::Pd126 => 126u16,
            Self::Pd127 => 127u16,
            Self::Pd128 => 128u16,
        }
    }
}
impl super::IsotopicComposition for PalladiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Pd102 => Some(0.0102f64),
            Self::Pd104 => Some(0.1114f64),
            Self::Pd105 => Some(0.2233f64),
            Self::Pd106 => Some(0.2733f64),
            Self::Pd108 => Some(0.2646f64),
            Self::Pd110 => Some(0.1172f64),
            Self::Pd91
            | Self::Pd92
            | Self::Pd93
            | Self::Pd94
            | Self::Pd95
            | Self::Pd96
            | Self::Pd97
            | Self::Pd98
            | Self::Pd99
            | Self::Pd100
            | Self::Pd101
            | Self::Pd103
            | Self::Pd107
            | Self::Pd109
            | Self::Pd111
            | Self::Pd112
            | Self::Pd113
            | Self::Pd114
            | Self::Pd115
            | Self::Pd116
            | Self::Pd117
            | Self::Pd118
            | Self::Pd119
            | Self::Pd120
            | Self::Pd121
            | Self::Pd122
            | Self::Pd123
            | Self::Pd124
            | Self::Pd125
            | Self::Pd126
            | Self::Pd127
            | Self::Pd128 => None,
        }
    }
}
impl super::MostAbundantIsotope for PalladiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pd106
    }
}
impl From<PalladiumIsotope> for crate::Isotope {
    fn from(isotope: PalladiumIsotope) -> Self {
        crate::Isotope::Pd(isotope)
    }
}
impl From<PalladiumIsotope> for crate::Element {
    fn from(_isotope: PalladiumIsotope) -> Self {
        crate::Element::Pd
    }
}
impl TryFrom<u16> for PalladiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            91u16 => Ok(Self::Pd91),
            92u16 => Ok(Self::Pd92),
            93u16 => Ok(Self::Pd93),
            94u16 => Ok(Self::Pd94),
            95u16 => Ok(Self::Pd95),
            96u16 => Ok(Self::Pd96),
            97u16 => Ok(Self::Pd97),
            98u16 => Ok(Self::Pd98),
            99u16 => Ok(Self::Pd99),
            100u16 => Ok(Self::Pd100),
            101u16 => Ok(Self::Pd101),
            102u16 => Ok(Self::Pd102),
            103u16 => Ok(Self::Pd103),
            104u16 => Ok(Self::Pd104),
            105u16 => Ok(Self::Pd105),
            106u16 => Ok(Self::Pd106),
            107u16 => Ok(Self::Pd107),
            108u16 => Ok(Self::Pd108),
            109u16 => Ok(Self::Pd109),
            110u16 => Ok(Self::Pd110),
            111u16 => Ok(Self::Pd111),
            112u16 => Ok(Self::Pd112),
            113u16 => Ok(Self::Pd113),
            114u16 => Ok(Self::Pd114),
            115u16 => Ok(Self::Pd115),
            116u16 => Ok(Self::Pd116),
            117u16 => Ok(Self::Pd117),
            118u16 => Ok(Self::Pd118),
            119u16 => Ok(Self::Pd119),
            120u16 => Ok(Self::Pd120),
            121u16 => Ok(Self::Pd121),
            122u16 => Ok(Self::Pd122),
            123u16 => Ok(Self::Pd123),
            124u16 => Ok(Self::Pd124),
            125u16 => Ok(Self::Pd125),
            126u16 => Ok(Self::Pd126),
            127u16 => Ok(Self::Pd127),
            128u16 => Ok(Self::Pd128),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pd, value)),
        }
    }
}
impl std::fmt::Display for PalladiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pd91 => write!(f, "Pd91"),
            Self::Pd92 => write!(f, "Pd92"),
            Self::Pd93 => write!(f, "Pd93"),
            Self::Pd94 => write!(f, "Pd94"),
            Self::Pd95 => write!(f, "Pd95"),
            Self::Pd96 => write!(f, "Pd96"),
            Self::Pd97 => write!(f, "Pd97"),
            Self::Pd98 => write!(f, "Pd98"),
            Self::Pd99 => write!(f, "Pd99"),
            Self::Pd100 => write!(f, "Pd100"),
            Self::Pd101 => write!(f, "Pd101"),
            Self::Pd102 => write!(f, "Pd102"),
            Self::Pd103 => write!(f, "Pd103"),
            Self::Pd104 => write!(f, "Pd104"),
            Self::Pd105 => write!(f, "Pd105"),
            Self::Pd106 => write!(f, "Pd106"),
            Self::Pd107 => write!(f, "Pd107"),
            Self::Pd108 => write!(f, "Pd108"),
            Self::Pd109 => write!(f, "Pd109"),
            Self::Pd110 => write!(f, "Pd110"),
            Self::Pd111 => write!(f, "Pd111"),
            Self::Pd112 => write!(f, "Pd112"),
            Self::Pd113 => write!(f, "Pd113"),
            Self::Pd114 => write!(f, "Pd114"),
            Self::Pd115 => write!(f, "Pd115"),
            Self::Pd116 => write!(f, "Pd116"),
            Self::Pd117 => write!(f, "Pd117"),
            Self::Pd118 => write!(f, "Pd118"),
            Self::Pd119 => write!(f, "Pd119"),
            Self::Pd120 => write!(f, "Pd120"),
            Self::Pd121 => write!(f, "Pd121"),
            Self::Pd122 => write!(f, "Pd122"),
            Self::Pd123 => write!(f, "Pd123"),
            Self::Pd124 => write!(f, "Pd124"),
            Self::Pd125 => write!(f, "Pd125"),
            Self::Pd126 => write!(f, "Pd126"),
            Self::Pd127 => write!(f, "Pd127"),
            Self::Pd128 => write!(f, "Pd128"),
        }
    }
}
