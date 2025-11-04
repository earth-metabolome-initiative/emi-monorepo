//! Isotopes of the element Xenon
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Xenon
pub enum XenonIsotope {
    /// Isotope Xe109 of Xenon
    Xe109,
    /// Isotope Xe110 of Xenon
    Xe110,
    /// Isotope Xe111 of Xenon
    Xe111,
    /// Isotope Xe112 of Xenon
    Xe112,
    /// Isotope Xe113 of Xenon
    Xe113,
    /// Isotope Xe114 of Xenon
    Xe114,
    /// Isotope Xe115 of Xenon
    Xe115,
    /// Isotope Xe116 of Xenon
    Xe116,
    /// Isotope Xe117 of Xenon
    Xe117,
    /// Isotope Xe118 of Xenon
    Xe118,
    /// Isotope Xe119 of Xenon
    Xe119,
    /// Isotope Xe120 of Xenon
    Xe120,
    /// Isotope Xe121 of Xenon
    Xe121,
    /// Isotope Xe122 of Xenon
    Xe122,
    /// Isotope Xe123 of Xenon
    Xe123,
    /// Isotope Xe124 of Xenon
    Xe124,
    /// Isotope Xe125 of Xenon
    Xe125,
    /// Isotope Xe126 of Xenon
    Xe126,
    /// Isotope Xe127 of Xenon
    Xe127,
    /// Isotope Xe128 of Xenon
    Xe128,
    /// Isotope Xe129 of Xenon
    Xe129,
    /// Isotope Xe130 of Xenon
    Xe130,
    /// Isotope Xe131 of Xenon
    Xe131,
    /// Isotope Xe132 of Xenon
    Xe132,
    /// Isotope Xe133 of Xenon
    Xe133,
    /// Isotope Xe134 of Xenon
    Xe134,
    /// Isotope Xe135 of Xenon
    Xe135,
    /// Isotope Xe136 of Xenon
    Xe136,
    /// Isotope Xe137 of Xenon
    Xe137,
    /// Isotope Xe138 of Xenon
    Xe138,
    /// Isotope Xe139 of Xenon
    Xe139,
    /// Isotope Xe140 of Xenon
    Xe140,
    /// Isotope Xe141 of Xenon
    Xe141,
    /// Isotope Xe142 of Xenon
    Xe142,
    /// Isotope Xe143 of Xenon
    Xe143,
    /// Isotope Xe144 of Xenon
    Xe144,
    /// Isotope Xe145 of Xenon
    Xe145,
    /// Isotope Xe146 of Xenon
    Xe146,
    /// Isotope Xe147 of Xenon
    Xe147,
    /// Isotope Xe148 of Xenon
    Xe148,
}
impl super::RelativeAtomicMass for XenonIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Xe109 => 108.95043f64,
            Self::Xe110 => 109.94426f64,
            Self::Xe111 => 110.941607f64,
            Self::Xe112 => 111.935559f64,
            Self::Xe113 => 112.9332217f64,
            Self::Xe114 => 113.92798f64,
            Self::Xe115 => 114.926294f64,
            Self::Xe116 => 115.921581f64,
            Self::Xe117 => 116.920359f64,
            Self::Xe118 => 117.916179f64,
            Self::Xe119 => 118.915411f64,
            Self::Xe120 => 119.911784f64,
            Self::Xe121 => 120.911453f64,
            Self::Xe122 => 121.908368f64,
            Self::Xe123 => 122.908482f64,
            Self::Xe124 => 123.905892f64,
            Self::Xe125 => 124.9063944f64,
            Self::Xe126 => 125.9042983f64,
            Self::Xe127 => 126.9051829f64,
            Self::Xe128 => 127.903531f64,
            Self::Xe129 => 128.9047808611f64,
            Self::Xe130 => 129.903509349f64,
            Self::Xe131 => 130.90508406f64,
            Self::Xe132 => 131.9041550856f64,
            Self::Xe133 => 132.9059108f64,
            Self::Xe134 => 133.90539466f64,
            Self::Xe135 => 134.9072278f64,
            Self::Xe136 => 135.907214484f64,
            Self::Xe137 => 136.91155778f64,
            Self::Xe138 => 137.9141463f64,
            Self::Xe139 => 138.9187922f64,
            Self::Xe140 => 139.9216458f64,
            Self::Xe141 => 140.9267872f64,
            Self::Xe142 => 141.9299731f64,
            Self::Xe143 => 142.9353696f64,
            Self::Xe144 => 143.9389451f64,
            Self::Xe145 => 144.94472f64,
            Self::Xe146 => 145.948518f64,
            Self::Xe147 => 146.95426f64,
            Self::Xe148 => 147.95813f64,
        }
    }
}
impl super::ElementVariant for XenonIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Xe
    }
}
impl super::MassNumber for XenonIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Xe109 => 109u16,
            Self::Xe110 => 110u16,
            Self::Xe111 => 111u16,
            Self::Xe112 => 112u16,
            Self::Xe113 => 113u16,
            Self::Xe114 => 114u16,
            Self::Xe115 => 115u16,
            Self::Xe116 => 116u16,
            Self::Xe117 => 117u16,
            Self::Xe118 => 118u16,
            Self::Xe119 => 119u16,
            Self::Xe120 => 120u16,
            Self::Xe121 => 121u16,
            Self::Xe122 => 122u16,
            Self::Xe123 => 123u16,
            Self::Xe124 => 124u16,
            Self::Xe125 => 125u16,
            Self::Xe126 => 126u16,
            Self::Xe127 => 127u16,
            Self::Xe128 => 128u16,
            Self::Xe129 => 129u16,
            Self::Xe130 => 130u16,
            Self::Xe131 => 131u16,
            Self::Xe132 => 132u16,
            Self::Xe133 => 133u16,
            Self::Xe134 => 134u16,
            Self::Xe135 => 135u16,
            Self::Xe136 => 136u16,
            Self::Xe137 => 137u16,
            Self::Xe138 => 138u16,
            Self::Xe139 => 139u16,
            Self::Xe140 => 140u16,
            Self::Xe141 => 141u16,
            Self::Xe142 => 142u16,
            Self::Xe143 => 143u16,
            Self::Xe144 => 144u16,
            Self::Xe145 => 145u16,
            Self::Xe146 => 146u16,
            Self::Xe147 => 147u16,
            Self::Xe148 => 148u16,
        }
    }
}
impl super::IsotopicComposition for XenonIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Xe124 => Some(0.000952f64),
            Self::Xe126 => Some(0.00089f64),
            Self::Xe128 => Some(0.019102f64),
            Self::Xe129 => Some(0.264006f64),
            Self::Xe130 => Some(0.04071f64),
            Self::Xe131 => Some(0.212324f64),
            Self::Xe132 => Some(0.269086f64),
            Self::Xe134 => Some(0.104357f64),
            Self::Xe136 => Some(0.088573f64),
            Self::Xe109
            | Self::Xe110
            | Self::Xe111
            | Self::Xe112
            | Self::Xe113
            | Self::Xe114
            | Self::Xe115
            | Self::Xe116
            | Self::Xe117
            | Self::Xe118
            | Self::Xe119
            | Self::Xe120
            | Self::Xe121
            | Self::Xe122
            | Self::Xe123
            | Self::Xe125
            | Self::Xe127
            | Self::Xe133
            | Self::Xe135
            | Self::Xe137
            | Self::Xe138
            | Self::Xe139
            | Self::Xe140
            | Self::Xe141
            | Self::Xe142
            | Self::Xe143
            | Self::Xe144
            | Self::Xe145
            | Self::Xe146
            | Self::Xe147
            | Self::Xe148 => None,
        }
    }
}
impl super::MostAbundantIsotope for XenonIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Xe132
    }
}
impl From<XenonIsotope> for crate::Isotope {
    fn from(isotope: XenonIsotope) -> Self {
        crate::Isotope::Xe(isotope)
    }
}
impl From<XenonIsotope> for crate::Element {
    fn from(_isotope: XenonIsotope) -> Self {
        crate::Element::Xe
    }
}
impl TryFrom<u16> for XenonIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            109u16 => Ok(Self::Xe109),
            110u16 => Ok(Self::Xe110),
            111u16 => Ok(Self::Xe111),
            112u16 => Ok(Self::Xe112),
            113u16 => Ok(Self::Xe113),
            114u16 => Ok(Self::Xe114),
            115u16 => Ok(Self::Xe115),
            116u16 => Ok(Self::Xe116),
            117u16 => Ok(Self::Xe117),
            118u16 => Ok(Self::Xe118),
            119u16 => Ok(Self::Xe119),
            120u16 => Ok(Self::Xe120),
            121u16 => Ok(Self::Xe121),
            122u16 => Ok(Self::Xe122),
            123u16 => Ok(Self::Xe123),
            124u16 => Ok(Self::Xe124),
            125u16 => Ok(Self::Xe125),
            126u16 => Ok(Self::Xe126),
            127u16 => Ok(Self::Xe127),
            128u16 => Ok(Self::Xe128),
            129u16 => Ok(Self::Xe129),
            130u16 => Ok(Self::Xe130),
            131u16 => Ok(Self::Xe131),
            132u16 => Ok(Self::Xe132),
            133u16 => Ok(Self::Xe133),
            134u16 => Ok(Self::Xe134),
            135u16 => Ok(Self::Xe135),
            136u16 => Ok(Self::Xe136),
            137u16 => Ok(Self::Xe137),
            138u16 => Ok(Self::Xe138),
            139u16 => Ok(Self::Xe139),
            140u16 => Ok(Self::Xe140),
            141u16 => Ok(Self::Xe141),
            142u16 => Ok(Self::Xe142),
            143u16 => Ok(Self::Xe143),
            144u16 => Ok(Self::Xe144),
            145u16 => Ok(Self::Xe145),
            146u16 => Ok(Self::Xe146),
            147u16 => Ok(Self::Xe147),
            148u16 => Ok(Self::Xe148),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Xe, value)),
        }
    }
}
impl std::fmt::Display for XenonIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xe109 => write!(f, "Xe109"),
            Self::Xe110 => write!(f, "Xe110"),
            Self::Xe111 => write!(f, "Xe111"),
            Self::Xe112 => write!(f, "Xe112"),
            Self::Xe113 => write!(f, "Xe113"),
            Self::Xe114 => write!(f, "Xe114"),
            Self::Xe115 => write!(f, "Xe115"),
            Self::Xe116 => write!(f, "Xe116"),
            Self::Xe117 => write!(f, "Xe117"),
            Self::Xe118 => write!(f, "Xe118"),
            Self::Xe119 => write!(f, "Xe119"),
            Self::Xe120 => write!(f, "Xe120"),
            Self::Xe121 => write!(f, "Xe121"),
            Self::Xe122 => write!(f, "Xe122"),
            Self::Xe123 => write!(f, "Xe123"),
            Self::Xe124 => write!(f, "Xe124"),
            Self::Xe125 => write!(f, "Xe125"),
            Self::Xe126 => write!(f, "Xe126"),
            Self::Xe127 => write!(f, "Xe127"),
            Self::Xe128 => write!(f, "Xe128"),
            Self::Xe129 => write!(f, "Xe129"),
            Self::Xe130 => write!(f, "Xe130"),
            Self::Xe131 => write!(f, "Xe131"),
            Self::Xe132 => write!(f, "Xe132"),
            Self::Xe133 => write!(f, "Xe133"),
            Self::Xe134 => write!(f, "Xe134"),
            Self::Xe135 => write!(f, "Xe135"),
            Self::Xe136 => write!(f, "Xe136"),
            Self::Xe137 => write!(f, "Xe137"),
            Self::Xe138 => write!(f, "Xe138"),
            Self::Xe139 => write!(f, "Xe139"),
            Self::Xe140 => write!(f, "Xe140"),
            Self::Xe141 => write!(f, "Xe141"),
            Self::Xe142 => write!(f, "Xe142"),
            Self::Xe143 => write!(f, "Xe143"),
            Self::Xe144 => write!(f, "Xe144"),
            Self::Xe145 => write!(f, "Xe145"),
            Self::Xe146 => write!(f, "Xe146"),
            Self::Xe147 => write!(f, "Xe147"),
            Self::Xe148 => write!(f, "Xe148"),
        }
    }
}
