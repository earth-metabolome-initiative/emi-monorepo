//! Isotopes of the element Lanthanum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Lanthanum
pub enum LanthanumIsotope {
    /// Isotope La116 of Lanthanum
    La116,
    /// Isotope La117 of Lanthanum
    La117,
    /// Isotope La118 of Lanthanum
    La118,
    /// Isotope La119 of Lanthanum
    La119,
    /// Isotope La120 of Lanthanum
    La120,
    /// Isotope La121 of Lanthanum
    La121,
    /// Isotope La122 of Lanthanum
    La122,
    /// Isotope La123 of Lanthanum
    La123,
    /// Isotope La124 of Lanthanum
    La124,
    /// Isotope La125 of Lanthanum
    La125,
    /// Isotope La126 of Lanthanum
    La126,
    /// Isotope La127 of Lanthanum
    La127,
    /// Isotope La128 of Lanthanum
    La128,
    /// Isotope La129 of Lanthanum
    La129,
    /// Isotope La130 of Lanthanum
    La130,
    /// Isotope La131 of Lanthanum
    La131,
    /// Isotope La132 of Lanthanum
    La132,
    /// Isotope La133 of Lanthanum
    La133,
    /// Isotope La134 of Lanthanum
    La134,
    /// Isotope La135 of Lanthanum
    La135,
    /// Isotope La136 of Lanthanum
    La136,
    /// Isotope La137 of Lanthanum
    La137,
    /// Isotope La138 of Lanthanum
    La138,
    /// Isotope La139 of Lanthanum
    La139,
    /// Isotope La140 of Lanthanum
    La140,
    /// Isotope La141 of Lanthanum
    La141,
    /// Isotope La142 of Lanthanum
    La142,
    /// Isotope La143 of Lanthanum
    La143,
    /// Isotope La144 of Lanthanum
    La144,
    /// Isotope La145 of Lanthanum
    La145,
    /// Isotope La146 of Lanthanum
    La146,
    /// Isotope La147 of Lanthanum
    La147,
    /// Isotope La148 of Lanthanum
    La148,
    /// Isotope La149 of Lanthanum
    La149,
    /// Isotope La150 of Lanthanum
    La150,
    /// Isotope La151 of Lanthanum
    La151,
    /// Isotope La152 of Lanthanum
    La152,
    /// Isotope La153 of Lanthanum
    La153,
    /// Isotope La154 of Lanthanum
    La154,
    /// Isotope La155 of Lanthanum
    La155,
}
impl super::RelativeAtomicMass for LanthanumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::La116 => 115.9563f64,
            Self::La117 => 116.94999f64,
            Self::La118 => 117.94673f64,
            Self::La119 => 118.94099f64,
            Self::La120 => 119.93807f64,
            Self::La121 => 120.93315f64,
            Self::La122 => 121.93071f64,
            Self::La123 => 122.9263f64,
            Self::La124 => 123.924574f64,
            Self::La125 => 124.920816f64,
            Self::La126 => 125.919513f64,
            Self::La127 => 126.916375f64,
            Self::La128 => 127.915592f64,
            Self::La129 => 128.912694f64,
            Self::La130 => 129.912369f64,
            Self::La131 => 130.91007f64,
            Self::La132 => 131.910119f64,
            Self::La133 => 132.908218f64,
            Self::La134 => 133.908514f64,
            Self::La135 => 134.906984f64,
            Self::La136 => 135.907635f64,
            Self::La137 => 136.9064504f64,
            Self::La138 => 137.9071149f64,
            Self::La139 => 138.9063563f64,
            Self::La140 => 139.9094806f64,
            Self::La141 => 140.910966f64,
            Self::La142 => 141.9140909f64,
            Self::La143 => 142.9160795f64,
            Self::La144 => 143.919646f64,
            Self::La145 => 144.921808f64,
            Self::La146 => 145.925875f64,
            Self::La147 => 146.928418f64,
            Self::La148 => 147.932679f64,
            Self::La149 => 148.93535f64,
            Self::La150 => 149.93947f64,
            Self::La151 => 150.94232f64,
            Self::La152 => 151.94682f64,
            Self::La153 => 152.95036f64,
            Self::La154 => 153.95517f64,
            Self::La155 => 154.95901f64,
        }
    }
}
impl super::ElementVariant for LanthanumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::La
    }
}
impl super::MassNumber for LanthanumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::La116 => 116u16,
            Self::La117 => 117u16,
            Self::La118 => 118u16,
            Self::La119 => 119u16,
            Self::La120 => 120u16,
            Self::La121 => 121u16,
            Self::La122 => 122u16,
            Self::La123 => 123u16,
            Self::La124 => 124u16,
            Self::La125 => 125u16,
            Self::La126 => 126u16,
            Self::La127 => 127u16,
            Self::La128 => 128u16,
            Self::La129 => 129u16,
            Self::La130 => 130u16,
            Self::La131 => 131u16,
            Self::La132 => 132u16,
            Self::La133 => 133u16,
            Self::La134 => 134u16,
            Self::La135 => 135u16,
            Self::La136 => 136u16,
            Self::La137 => 137u16,
            Self::La138 => 138u16,
            Self::La139 => 139u16,
            Self::La140 => 140u16,
            Self::La141 => 141u16,
            Self::La142 => 142u16,
            Self::La143 => 143u16,
            Self::La144 => 144u16,
            Self::La145 => 145u16,
            Self::La146 => 146u16,
            Self::La147 => 147u16,
            Self::La148 => 148u16,
            Self::La149 => 149u16,
            Self::La150 => 150u16,
            Self::La151 => 151u16,
            Self::La152 => 152u16,
            Self::La153 => 153u16,
            Self::La154 => 154u16,
            Self::La155 => 155u16,
        }
    }
}
impl super::IsotopicComposition for LanthanumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::La138 => Some(0.0008881f64),
            Self::La139 => Some(0.9991119f64),
            Self::La116
            | Self::La117
            | Self::La118
            | Self::La119
            | Self::La120
            | Self::La121
            | Self::La122
            | Self::La123
            | Self::La124
            | Self::La125
            | Self::La126
            | Self::La127
            | Self::La128
            | Self::La129
            | Self::La130
            | Self::La131
            | Self::La132
            | Self::La133
            | Self::La134
            | Self::La135
            | Self::La136
            | Self::La137
            | Self::La140
            | Self::La141
            | Self::La142
            | Self::La143
            | Self::La144
            | Self::La145
            | Self::La146
            | Self::La147
            | Self::La148
            | Self::La149
            | Self::La150
            | Self::La151
            | Self::La152
            | Self::La153
            | Self::La154
            | Self::La155 => None,
        }
    }
}
impl super::MostAbundantIsotope for LanthanumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::La139
    }
}
impl From<LanthanumIsotope> for crate::Isotope {
    fn from(isotope: LanthanumIsotope) -> Self {
        crate::Isotope::La(isotope)
    }
}
impl From<LanthanumIsotope> for crate::Element {
    fn from(_isotope: LanthanumIsotope) -> Self {
        crate::Element::La
    }
}
impl TryFrom<u16> for LanthanumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            116u16 => Ok(Self::La116),
            117u16 => Ok(Self::La117),
            118u16 => Ok(Self::La118),
            119u16 => Ok(Self::La119),
            120u16 => Ok(Self::La120),
            121u16 => Ok(Self::La121),
            122u16 => Ok(Self::La122),
            123u16 => Ok(Self::La123),
            124u16 => Ok(Self::La124),
            125u16 => Ok(Self::La125),
            126u16 => Ok(Self::La126),
            127u16 => Ok(Self::La127),
            128u16 => Ok(Self::La128),
            129u16 => Ok(Self::La129),
            130u16 => Ok(Self::La130),
            131u16 => Ok(Self::La131),
            132u16 => Ok(Self::La132),
            133u16 => Ok(Self::La133),
            134u16 => Ok(Self::La134),
            135u16 => Ok(Self::La135),
            136u16 => Ok(Self::La136),
            137u16 => Ok(Self::La137),
            138u16 => Ok(Self::La138),
            139u16 => Ok(Self::La139),
            140u16 => Ok(Self::La140),
            141u16 => Ok(Self::La141),
            142u16 => Ok(Self::La142),
            143u16 => Ok(Self::La143),
            144u16 => Ok(Self::La144),
            145u16 => Ok(Self::La145),
            146u16 => Ok(Self::La146),
            147u16 => Ok(Self::La147),
            148u16 => Ok(Self::La148),
            149u16 => Ok(Self::La149),
            150u16 => Ok(Self::La150),
            151u16 => Ok(Self::La151),
            152u16 => Ok(Self::La152),
            153u16 => Ok(Self::La153),
            154u16 => Ok(Self::La154),
            155u16 => Ok(Self::La155),
            _ => Err(crate::errors::Error::Isotope(crate::Element::La, value)),
        }
    }
}
impl std::fmt::Display for LanthanumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::La116 => write!(f, "La116"),
            Self::La117 => write!(f, "La117"),
            Self::La118 => write!(f, "La118"),
            Self::La119 => write!(f, "La119"),
            Self::La120 => write!(f, "La120"),
            Self::La121 => write!(f, "La121"),
            Self::La122 => write!(f, "La122"),
            Self::La123 => write!(f, "La123"),
            Self::La124 => write!(f, "La124"),
            Self::La125 => write!(f, "La125"),
            Self::La126 => write!(f, "La126"),
            Self::La127 => write!(f, "La127"),
            Self::La128 => write!(f, "La128"),
            Self::La129 => write!(f, "La129"),
            Self::La130 => write!(f, "La130"),
            Self::La131 => write!(f, "La131"),
            Self::La132 => write!(f, "La132"),
            Self::La133 => write!(f, "La133"),
            Self::La134 => write!(f, "La134"),
            Self::La135 => write!(f, "La135"),
            Self::La136 => write!(f, "La136"),
            Self::La137 => write!(f, "La137"),
            Self::La138 => write!(f, "La138"),
            Self::La139 => write!(f, "La139"),
            Self::La140 => write!(f, "La140"),
            Self::La141 => write!(f, "La141"),
            Self::La142 => write!(f, "La142"),
            Self::La143 => write!(f, "La143"),
            Self::La144 => write!(f, "La144"),
            Self::La145 => write!(f, "La145"),
            Self::La146 => write!(f, "La146"),
            Self::La147 => write!(f, "La147"),
            Self::La148 => write!(f, "La148"),
            Self::La149 => write!(f, "La149"),
            Self::La150 => write!(f, "La150"),
            Self::La151 => write!(f, "La151"),
            Self::La152 => write!(f, "La152"),
            Self::La153 => write!(f, "La153"),
            Self::La154 => write!(f, "La154"),
            Self::La155 => write!(f, "La155"),
        }
    }
}
