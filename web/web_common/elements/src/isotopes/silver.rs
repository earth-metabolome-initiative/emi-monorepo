//! Isotopes of the element Silver
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Silver
pub enum SilverIsotope {
    /// Isotope Ag93 of Silver
    Ag93,
    /// Isotope Ag94 of Silver
    Ag94,
    /// Isotope Ag95 of Silver
    Ag95,
    /// Isotope Ag96 of Silver
    Ag96,
    /// Isotope Ag97 of Silver
    Ag97,
    /// Isotope Ag98 of Silver
    Ag98,
    /// Isotope Ag99 of Silver
    Ag99,
    /// Isotope Ag100 of Silver
    Ag100,
    /// Isotope Ag101 of Silver
    Ag101,
    /// Isotope Ag102 of Silver
    Ag102,
    /// Isotope Ag103 of Silver
    Ag103,
    /// Isotope Ag104 of Silver
    Ag104,
    /// Isotope Ag105 of Silver
    Ag105,
    /// Isotope Ag106 of Silver
    Ag106,
    /// Isotope Ag107 of Silver
    Ag107,
    /// Isotope Ag108 of Silver
    Ag108,
    /// Isotope Ag109 of Silver
    Ag109,
    /// Isotope Ag110 of Silver
    Ag110,
    /// Isotope Ag111 of Silver
    Ag111,
    /// Isotope Ag112 of Silver
    Ag112,
    /// Isotope Ag113 of Silver
    Ag113,
    /// Isotope Ag114 of Silver
    Ag114,
    /// Isotope Ag115 of Silver
    Ag115,
    /// Isotope Ag116 of Silver
    Ag116,
    /// Isotope Ag117 of Silver
    Ag117,
    /// Isotope Ag118 of Silver
    Ag118,
    /// Isotope Ag119 of Silver
    Ag119,
    /// Isotope Ag120 of Silver
    Ag120,
    /// Isotope Ag121 of Silver
    Ag121,
    /// Isotope Ag122 of Silver
    Ag122,
    /// Isotope Ag123 of Silver
    Ag123,
    /// Isotope Ag124 of Silver
    Ag124,
    /// Isotope Ag125 of Silver
    Ag125,
    /// Isotope Ag126 of Silver
    Ag126,
    /// Isotope Ag127 of Silver
    Ag127,
    /// Isotope Ag128 of Silver
    Ag128,
    /// Isotope Ag129 of Silver
    Ag129,
    /// Isotope Ag130 of Silver
    Ag130,
}
impl super::RelativeAtomicMass for SilverIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ag93 => 92.95033f64,
            Self::Ag94 => 93.94373f64,
            Self::Ag95 => 94.93602f64,
            Self::Ag96 => 95.930744f64,
            Self::Ag97 => 96.92397f64,
            Self::Ag98 => 97.92156f64,
            Self::Ag99 => 98.9176458f64,
            Self::Ag100 => 99.9161154f64,
            Self::Ag101 => 100.912684f64,
            Self::Ag102 => 101.9117047f64,
            Self::Ag103 => 102.9089631f64,
            Self::Ag104 => 103.9086239f64,
            Self::Ag105 => 104.9065256f64,
            Self::Ag106 => 105.9066636f64,
            Self::Ag107 => 106.9050916f64,
            Self::Ag108 => 107.9059503f64,
            Self::Ag109 => 108.9047553f64,
            Self::Ag110 => 109.9061102f64,
            Self::Ag111 => 110.9052959f64,
            Self::Ag112 => 111.9070486f64,
            Self::Ag113 => 112.906573f64,
            Self::Ag114 => 113.908823f64,
            Self::Ag115 => 114.908767f64,
            Self::Ag116 => 115.9113868f64,
            Self::Ag117 => 116.911774f64,
            Self::Ag118 => 117.9145955f64,
            Self::Ag119 => 118.91557f64,
            Self::Ag120 => 119.9187848f64,
            Self::Ag121 => 120.920125f64,
            Self::Ag122 => 121.923664f64,
            Self::Ag123 => 122.925337f64,
            Self::Ag124 => 123.92893f64,
            Self::Ag125 => 124.93105f64,
            Self::Ag126 => 125.93475f64,
            Self::Ag127 => 126.93711f64,
            Self::Ag128 => 127.94106f64,
            Self::Ag129 => 128.94395f64,
            Self::Ag130 => 129.9507f64,
        }
    }
}
impl super::ElementVariant for SilverIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ag
    }
}
impl super::MassNumber for SilverIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ag93 => 93u16,
            Self::Ag94 => 94u16,
            Self::Ag95 => 95u16,
            Self::Ag96 => 96u16,
            Self::Ag97 => 97u16,
            Self::Ag98 => 98u16,
            Self::Ag99 => 99u16,
            Self::Ag100 => 100u16,
            Self::Ag101 => 101u16,
            Self::Ag102 => 102u16,
            Self::Ag103 => 103u16,
            Self::Ag104 => 104u16,
            Self::Ag105 => 105u16,
            Self::Ag106 => 106u16,
            Self::Ag107 => 107u16,
            Self::Ag108 => 108u16,
            Self::Ag109 => 109u16,
            Self::Ag110 => 110u16,
            Self::Ag111 => 111u16,
            Self::Ag112 => 112u16,
            Self::Ag113 => 113u16,
            Self::Ag114 => 114u16,
            Self::Ag115 => 115u16,
            Self::Ag116 => 116u16,
            Self::Ag117 => 117u16,
            Self::Ag118 => 118u16,
            Self::Ag119 => 119u16,
            Self::Ag120 => 120u16,
            Self::Ag121 => 121u16,
            Self::Ag122 => 122u16,
            Self::Ag123 => 123u16,
            Self::Ag124 => 124u16,
            Self::Ag125 => 125u16,
            Self::Ag126 => 126u16,
            Self::Ag127 => 127u16,
            Self::Ag128 => 128u16,
            Self::Ag129 => 129u16,
            Self::Ag130 => 130u16,
        }
    }
}
impl super::IsotopicComposition for SilverIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ag107 => Some(0.51839f64),
            Self::Ag109 => Some(0.48161f64),
            Self::Ag93
            | Self::Ag94
            | Self::Ag95
            | Self::Ag96
            | Self::Ag97
            | Self::Ag98
            | Self::Ag99
            | Self::Ag100
            | Self::Ag101
            | Self::Ag102
            | Self::Ag103
            | Self::Ag104
            | Self::Ag105
            | Self::Ag106
            | Self::Ag108
            | Self::Ag110
            | Self::Ag111
            | Self::Ag112
            | Self::Ag113
            | Self::Ag114
            | Self::Ag115
            | Self::Ag116
            | Self::Ag117
            | Self::Ag118
            | Self::Ag119
            | Self::Ag120
            | Self::Ag121
            | Self::Ag122
            | Self::Ag123
            | Self::Ag124
            | Self::Ag125
            | Self::Ag126
            | Self::Ag127
            | Self::Ag128
            | Self::Ag129
            | Self::Ag130 => None,
        }
    }
}
impl super::MostAbundantIsotope for SilverIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ag107
    }
}
impl TryFrom<u16> for SilverIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            93u16 => Ok(Self::Ag93),
            94u16 => Ok(Self::Ag94),
            95u16 => Ok(Self::Ag95),
            96u16 => Ok(Self::Ag96),
            97u16 => Ok(Self::Ag97),
            98u16 => Ok(Self::Ag98),
            99u16 => Ok(Self::Ag99),
            100u16 => Ok(Self::Ag100),
            101u16 => Ok(Self::Ag101),
            102u16 => Ok(Self::Ag102),
            103u16 => Ok(Self::Ag103),
            104u16 => Ok(Self::Ag104),
            105u16 => Ok(Self::Ag105),
            106u16 => Ok(Self::Ag106),
            107u16 => Ok(Self::Ag107),
            108u16 => Ok(Self::Ag108),
            109u16 => Ok(Self::Ag109),
            110u16 => Ok(Self::Ag110),
            111u16 => Ok(Self::Ag111),
            112u16 => Ok(Self::Ag112),
            113u16 => Ok(Self::Ag113),
            114u16 => Ok(Self::Ag114),
            115u16 => Ok(Self::Ag115),
            116u16 => Ok(Self::Ag116),
            117u16 => Ok(Self::Ag117),
            118u16 => Ok(Self::Ag118),
            119u16 => Ok(Self::Ag119),
            120u16 => Ok(Self::Ag120),
            121u16 => Ok(Self::Ag121),
            122u16 => Ok(Self::Ag122),
            123u16 => Ok(Self::Ag123),
            124u16 => Ok(Self::Ag124),
            125u16 => Ok(Self::Ag125),
            126u16 => Ok(Self::Ag126),
            127u16 => Ok(Self::Ag127),
            128u16 => Ok(Self::Ag128),
            129u16 => Ok(Self::Ag129),
            130u16 => Ok(Self::Ag130),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ag, value)),
        }
    }
}
impl std::fmt::Display for SilverIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ag93 => write!(f, "Ag93"),
            Self::Ag94 => write!(f, "Ag94"),
            Self::Ag95 => write!(f, "Ag95"),
            Self::Ag96 => write!(f, "Ag96"),
            Self::Ag97 => write!(f, "Ag97"),
            Self::Ag98 => write!(f, "Ag98"),
            Self::Ag99 => write!(f, "Ag99"),
            Self::Ag100 => write!(f, "Ag100"),
            Self::Ag101 => write!(f, "Ag101"),
            Self::Ag102 => write!(f, "Ag102"),
            Self::Ag103 => write!(f, "Ag103"),
            Self::Ag104 => write!(f, "Ag104"),
            Self::Ag105 => write!(f, "Ag105"),
            Self::Ag106 => write!(f, "Ag106"),
            Self::Ag107 => write!(f, "Ag107"),
            Self::Ag108 => write!(f, "Ag108"),
            Self::Ag109 => write!(f, "Ag109"),
            Self::Ag110 => write!(f, "Ag110"),
            Self::Ag111 => write!(f, "Ag111"),
            Self::Ag112 => write!(f, "Ag112"),
            Self::Ag113 => write!(f, "Ag113"),
            Self::Ag114 => write!(f, "Ag114"),
            Self::Ag115 => write!(f, "Ag115"),
            Self::Ag116 => write!(f, "Ag116"),
            Self::Ag117 => write!(f, "Ag117"),
            Self::Ag118 => write!(f, "Ag118"),
            Self::Ag119 => write!(f, "Ag119"),
            Self::Ag120 => write!(f, "Ag120"),
            Self::Ag121 => write!(f, "Ag121"),
            Self::Ag122 => write!(f, "Ag122"),
            Self::Ag123 => write!(f, "Ag123"),
            Self::Ag124 => write!(f, "Ag124"),
            Self::Ag125 => write!(f, "Ag125"),
            Self::Ag126 => write!(f, "Ag126"),
            Self::Ag127 => write!(f, "Ag127"),
            Self::Ag128 => write!(f, "Ag128"),
            Self::Ag129 => write!(f, "Ag129"),
            Self::Ag130 => write!(f, "Ag130"),
        }
    }
}
