//! Isotopes of the element Cadmium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Cadmium
pub enum CadmiumIsotope {
    /// Isotope Cd95 of Cadmium
    Cd95,
    /// Isotope Cd96 of Cadmium
    Cd96,
    /// Isotope Cd97 of Cadmium
    Cd97,
    /// Isotope Cd98 of Cadmium
    Cd98,
    /// Isotope Cd99 of Cadmium
    Cd99,
    /// Isotope Cd100 of Cadmium
    Cd100,
    /// Isotope Cd101 of Cadmium
    Cd101,
    /// Isotope Cd102 of Cadmium
    Cd102,
    /// Isotope Cd103 of Cadmium
    Cd103,
    /// Isotope Cd104 of Cadmium
    Cd104,
    /// Isotope Cd105 of Cadmium
    Cd105,
    /// Isotope Cd106 of Cadmium
    Cd106,
    /// Isotope Cd107 of Cadmium
    Cd107,
    /// Isotope Cd108 of Cadmium
    Cd108,
    /// Isotope Cd109 of Cadmium
    Cd109,
    /// Isotope Cd110 of Cadmium
    Cd110,
    /// Isotope Cd111 of Cadmium
    Cd111,
    /// Isotope Cd112 of Cadmium
    Cd112,
    /// Isotope Cd113 of Cadmium
    Cd113,
    /// Isotope Cd114 of Cadmium
    Cd114,
    /// Isotope Cd115 of Cadmium
    Cd115,
    /// Isotope Cd116 of Cadmium
    Cd116,
    /// Isotope Cd117 of Cadmium
    Cd117,
    /// Isotope Cd118 of Cadmium
    Cd118,
    /// Isotope Cd119 of Cadmium
    Cd119,
    /// Isotope Cd120 of Cadmium
    Cd120,
    /// Isotope Cd121 of Cadmium
    Cd121,
    /// Isotope Cd122 of Cadmium
    Cd122,
    /// Isotope Cd123 of Cadmium
    Cd123,
    /// Isotope Cd124 of Cadmium
    Cd124,
    /// Isotope Cd125 of Cadmium
    Cd125,
    /// Isotope Cd126 of Cadmium
    Cd126,
    /// Isotope Cd127 of Cadmium
    Cd127,
    /// Isotope Cd128 of Cadmium
    Cd128,
    /// Isotope Cd129 of Cadmium
    Cd129,
    /// Isotope Cd130 of Cadmium
    Cd130,
    /// Isotope Cd131 of Cadmium
    Cd131,
    /// Isotope Cd132 of Cadmium
    Cd132,
    /// Isotope Cd133 of Cadmium
    Cd133,
}
impl super::RelativeAtomicMass for CadmiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Cd95 => 94.94994f64,
            Self::Cd96 => 95.94034f64,
            Self::Cd97 => 96.9351f64,
            Self::Cd98 => 97.927389f64,
            Self::Cd99 => 98.9249258f64,
            Self::Cd100 => 99.9203488f64,
            Self::Cd101 => 100.9185862f64,
            Self::Cd102 => 101.914482f64,
            Self::Cd103 => 102.9134165f64,
            Self::Cd104 => 103.9098564f64,
            Self::Cd105 => 104.9094639f64,
            Self::Cd106 => 105.9064599f64,
            Self::Cd107 => 106.9066121f64,
            Self::Cd108 => 107.9041834f64,
            Self::Cd109 => 108.9049867f64,
            Self::Cd110 => 109.90300661f64,
            Self::Cd111 => 110.90418287f64,
            Self::Cd112 => 111.90276287f64,
            Self::Cd113 => 112.90440813f64,
            Self::Cd114 => 113.90336509f64,
            Self::Cd115 => 114.90543751f64,
            Self::Cd116 => 115.90476315f64,
            Self::Cd117 => 116.907226f64,
            Self::Cd118 => 117.906922f64,
            Self::Cd119 => 118.909847f64,
            Self::Cd120 => 119.9098681f64,
            Self::Cd121 => 120.9129637f64,
            Self::Cd122 => 121.9134591f64,
            Self::Cd123 => 122.9168925f64,
            Self::Cd124 => 123.9176574f64,
            Self::Cd125 => 124.9212576f64,
            Self::Cd126 => 125.9224291f64,
            Self::Cd127 => 126.926472f64,
            Self::Cd128 => 127.9278129f64,
            Self::Cd129 => 128.93182f64,
            Self::Cd130 => 129.93394f64,
            Self::Cd131 => 130.9406f64,
            Self::Cd132 => 131.94604f64,
            Self::Cd133 => 132.95285f64,
        }
    }
}
impl super::ElementVariant for CadmiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Cd
    }
}
impl super::MassNumber for CadmiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Cd95 => 95u16,
            Self::Cd96 => 96u16,
            Self::Cd97 => 97u16,
            Self::Cd98 => 98u16,
            Self::Cd99 => 99u16,
            Self::Cd100 => 100u16,
            Self::Cd101 => 101u16,
            Self::Cd102 => 102u16,
            Self::Cd103 => 103u16,
            Self::Cd104 => 104u16,
            Self::Cd105 => 105u16,
            Self::Cd106 => 106u16,
            Self::Cd107 => 107u16,
            Self::Cd108 => 108u16,
            Self::Cd109 => 109u16,
            Self::Cd110 => 110u16,
            Self::Cd111 => 111u16,
            Self::Cd112 => 112u16,
            Self::Cd113 => 113u16,
            Self::Cd114 => 114u16,
            Self::Cd115 => 115u16,
            Self::Cd116 => 116u16,
            Self::Cd117 => 117u16,
            Self::Cd118 => 118u16,
            Self::Cd119 => 119u16,
            Self::Cd120 => 120u16,
            Self::Cd121 => 121u16,
            Self::Cd122 => 122u16,
            Self::Cd123 => 123u16,
            Self::Cd124 => 124u16,
            Self::Cd125 => 125u16,
            Self::Cd126 => 126u16,
            Self::Cd127 => 127u16,
            Self::Cd128 => 128u16,
            Self::Cd129 => 129u16,
            Self::Cd130 => 130u16,
            Self::Cd131 => 131u16,
            Self::Cd132 => 132u16,
            Self::Cd133 => 133u16,
        }
    }
}
impl super::IsotopicComposition for CadmiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Cd106 => Some(0.0125f64),
            Self::Cd108 => Some(0.0089f64),
            Self::Cd110 => Some(0.1249f64),
            Self::Cd111 => Some(0.128f64),
            Self::Cd112 => Some(0.2413f64),
            Self::Cd113 => Some(0.1222f64),
            Self::Cd114 => Some(0.2873f64),
            Self::Cd116 => Some(0.0749f64),
            Self::Cd95
            | Self::Cd96
            | Self::Cd97
            | Self::Cd98
            | Self::Cd99
            | Self::Cd100
            | Self::Cd101
            | Self::Cd102
            | Self::Cd103
            | Self::Cd104
            | Self::Cd105
            | Self::Cd107
            | Self::Cd109
            | Self::Cd115
            | Self::Cd117
            | Self::Cd118
            | Self::Cd119
            | Self::Cd120
            | Self::Cd121
            | Self::Cd122
            | Self::Cd123
            | Self::Cd124
            | Self::Cd125
            | Self::Cd126
            | Self::Cd127
            | Self::Cd128
            | Self::Cd129
            | Self::Cd130
            | Self::Cd131
            | Self::Cd132
            | Self::Cd133 => None,
        }
    }
}
impl super::MostAbundantIsotope for CadmiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Cd114
    }
}
impl From<CadmiumIsotope> for crate::Isotope {
    fn from(isotope: CadmiumIsotope) -> Self {
        crate::Isotope::Cd(isotope)
    }
}
impl From<CadmiumIsotope> for crate::Element {
    fn from(_isotope: CadmiumIsotope) -> Self {
        crate::Element::Cd
    }
}
impl TryFrom<u16> for CadmiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            95u16 => Ok(Self::Cd95),
            96u16 => Ok(Self::Cd96),
            97u16 => Ok(Self::Cd97),
            98u16 => Ok(Self::Cd98),
            99u16 => Ok(Self::Cd99),
            100u16 => Ok(Self::Cd100),
            101u16 => Ok(Self::Cd101),
            102u16 => Ok(Self::Cd102),
            103u16 => Ok(Self::Cd103),
            104u16 => Ok(Self::Cd104),
            105u16 => Ok(Self::Cd105),
            106u16 => Ok(Self::Cd106),
            107u16 => Ok(Self::Cd107),
            108u16 => Ok(Self::Cd108),
            109u16 => Ok(Self::Cd109),
            110u16 => Ok(Self::Cd110),
            111u16 => Ok(Self::Cd111),
            112u16 => Ok(Self::Cd112),
            113u16 => Ok(Self::Cd113),
            114u16 => Ok(Self::Cd114),
            115u16 => Ok(Self::Cd115),
            116u16 => Ok(Self::Cd116),
            117u16 => Ok(Self::Cd117),
            118u16 => Ok(Self::Cd118),
            119u16 => Ok(Self::Cd119),
            120u16 => Ok(Self::Cd120),
            121u16 => Ok(Self::Cd121),
            122u16 => Ok(Self::Cd122),
            123u16 => Ok(Self::Cd123),
            124u16 => Ok(Self::Cd124),
            125u16 => Ok(Self::Cd125),
            126u16 => Ok(Self::Cd126),
            127u16 => Ok(Self::Cd127),
            128u16 => Ok(Self::Cd128),
            129u16 => Ok(Self::Cd129),
            130u16 => Ok(Self::Cd130),
            131u16 => Ok(Self::Cd131),
            132u16 => Ok(Self::Cd132),
            133u16 => Ok(Self::Cd133),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Cd, value)),
        }
    }
}
impl std::fmt::Display for CadmiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cd95 => write!(f, "Cd95"),
            Self::Cd96 => write!(f, "Cd96"),
            Self::Cd97 => write!(f, "Cd97"),
            Self::Cd98 => write!(f, "Cd98"),
            Self::Cd99 => write!(f, "Cd99"),
            Self::Cd100 => write!(f, "Cd100"),
            Self::Cd101 => write!(f, "Cd101"),
            Self::Cd102 => write!(f, "Cd102"),
            Self::Cd103 => write!(f, "Cd103"),
            Self::Cd104 => write!(f, "Cd104"),
            Self::Cd105 => write!(f, "Cd105"),
            Self::Cd106 => write!(f, "Cd106"),
            Self::Cd107 => write!(f, "Cd107"),
            Self::Cd108 => write!(f, "Cd108"),
            Self::Cd109 => write!(f, "Cd109"),
            Self::Cd110 => write!(f, "Cd110"),
            Self::Cd111 => write!(f, "Cd111"),
            Self::Cd112 => write!(f, "Cd112"),
            Self::Cd113 => write!(f, "Cd113"),
            Self::Cd114 => write!(f, "Cd114"),
            Self::Cd115 => write!(f, "Cd115"),
            Self::Cd116 => write!(f, "Cd116"),
            Self::Cd117 => write!(f, "Cd117"),
            Self::Cd118 => write!(f, "Cd118"),
            Self::Cd119 => write!(f, "Cd119"),
            Self::Cd120 => write!(f, "Cd120"),
            Self::Cd121 => write!(f, "Cd121"),
            Self::Cd122 => write!(f, "Cd122"),
            Self::Cd123 => write!(f, "Cd123"),
            Self::Cd124 => write!(f, "Cd124"),
            Self::Cd125 => write!(f, "Cd125"),
            Self::Cd126 => write!(f, "Cd126"),
            Self::Cd127 => write!(f, "Cd127"),
            Self::Cd128 => write!(f, "Cd128"),
            Self::Cd129 => write!(f, "Cd129"),
            Self::Cd130 => write!(f, "Cd130"),
            Self::Cd131 => write!(f, "Cd131"),
            Self::Cd132 => write!(f, "Cd132"),
            Self::Cd133 => write!(f, "Cd133"),
        }
    }
}
