//! Isotopes of the element Antimony
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Antimony
pub enum AntimonyIsotope {
    /// Isotope Sb103 of Antimony
    Sb103,
    /// Isotope Sb104 of Antimony
    Sb104,
    /// Isotope Sb105 of Antimony
    Sb105,
    /// Isotope Sb106 of Antimony
    Sb106,
    /// Isotope Sb107 of Antimony
    Sb107,
    /// Isotope Sb108 of Antimony
    Sb108,
    /// Isotope Sb109 of Antimony
    Sb109,
    /// Isotope Sb110 of Antimony
    Sb110,
    /// Isotope Sb111 of Antimony
    Sb111,
    /// Isotope Sb112 of Antimony
    Sb112,
    /// Isotope Sb113 of Antimony
    Sb113,
    /// Isotope Sb114 of Antimony
    Sb114,
    /// Isotope Sb115 of Antimony
    Sb115,
    /// Isotope Sb116 of Antimony
    Sb116,
    /// Isotope Sb117 of Antimony
    Sb117,
    /// Isotope Sb118 of Antimony
    Sb118,
    /// Isotope Sb119 of Antimony
    Sb119,
    /// Isotope Sb120 of Antimony
    Sb120,
    /// Isotope Sb121 of Antimony
    Sb121,
    /// Isotope Sb122 of Antimony
    Sb122,
    /// Isotope Sb123 of Antimony
    Sb123,
    /// Isotope Sb124 of Antimony
    Sb124,
    /// Isotope Sb125 of Antimony
    Sb125,
    /// Isotope Sb126 of Antimony
    Sb126,
    /// Isotope Sb127 of Antimony
    Sb127,
    /// Isotope Sb128 of Antimony
    Sb128,
    /// Isotope Sb129 of Antimony
    Sb129,
    /// Isotope Sb130 of Antimony
    Sb130,
    /// Isotope Sb131 of Antimony
    Sb131,
    /// Isotope Sb132 of Antimony
    Sb132,
    /// Isotope Sb133 of Antimony
    Sb133,
    /// Isotope Sb134 of Antimony
    Sb134,
    /// Isotope Sb135 of Antimony
    Sb135,
    /// Isotope Sb136 of Antimony
    Sb136,
    /// Isotope Sb137 of Antimony
    Sb137,
    /// Isotope Sb138 of Antimony
    Sb138,
    /// Isotope Sb139 of Antimony
    Sb139,
    /// Isotope Sb140 of Antimony
    Sb140,
}
impl super::RelativeAtomicMass for AntimonyIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sb103 => 102.93969f64,
            Self::Sb104 => 103.93648f64,
            Self::Sb105 => 104.931276f64,
            Self::Sb106 => 105.928638f64,
            Self::Sb107 => 106.9241506f64,
            Self::Sb108 => 107.9222267f64,
            Self::Sb109 => 108.9181411f64,
            Self::Sb110 => 109.9168543f64,
            Self::Sb111 => 110.9132182f64,
            Self::Sb112 => 111.9124f64,
            Self::Sb113 => 112.909375f64,
            Self::Sb114 => 113.90929f64,
            Self::Sb115 => 114.906598f64,
            Self::Sb116 => 115.9067931f64,
            Self::Sb117 => 116.9048415f64,
            Self::Sb118 => 117.9055321f64,
            Self::Sb119 => 118.9039455f64,
            Self::Sb120 => 119.9050794f64,
            Self::Sb121 => 120.903812f64,
            Self::Sb122 => 121.9051699f64,
            Self::Sb123 => 122.9042132f64,
            Self::Sb124 => 123.905935f64,
            Self::Sb125 => 124.905253f64,
            Self::Sb126 => 125.907253f64,
            Self::Sb127 => 126.9069243f64,
            Self::Sb128 => 127.909146f64,
            Self::Sb129 => 128.909147f64,
            Self::Sb130 => 129.911662f64,
            Self::Sb131 => 130.9119888f64,
            Self::Sb132 => 131.9145077f64,
            Self::Sb133 => 132.9152732f64,
            Self::Sb134 => 133.9205357f64,
            Self::Sb135 => 134.9251851f64,
            Self::Sb136 => 135.9307459f64,
            Self::Sb137 => 136.93555f64,
            Self::Sb138 => 137.94145f64,
            Self::Sb139 => 138.94655f64,
            Self::Sb140 => 139.95283f64,
        }
    }
}
impl super::ElementVariant for AntimonyIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sb
    }
}
impl super::MassNumber for AntimonyIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sb103 => 103u16,
            Self::Sb104 => 104u16,
            Self::Sb105 => 105u16,
            Self::Sb106 => 106u16,
            Self::Sb107 => 107u16,
            Self::Sb108 => 108u16,
            Self::Sb109 => 109u16,
            Self::Sb110 => 110u16,
            Self::Sb111 => 111u16,
            Self::Sb112 => 112u16,
            Self::Sb113 => 113u16,
            Self::Sb114 => 114u16,
            Self::Sb115 => 115u16,
            Self::Sb116 => 116u16,
            Self::Sb117 => 117u16,
            Self::Sb118 => 118u16,
            Self::Sb119 => 119u16,
            Self::Sb120 => 120u16,
            Self::Sb121 => 121u16,
            Self::Sb122 => 122u16,
            Self::Sb123 => 123u16,
            Self::Sb124 => 124u16,
            Self::Sb125 => 125u16,
            Self::Sb126 => 126u16,
            Self::Sb127 => 127u16,
            Self::Sb128 => 128u16,
            Self::Sb129 => 129u16,
            Self::Sb130 => 130u16,
            Self::Sb131 => 131u16,
            Self::Sb132 => 132u16,
            Self::Sb133 => 133u16,
            Self::Sb134 => 134u16,
            Self::Sb135 => 135u16,
            Self::Sb136 => 136u16,
            Self::Sb137 => 137u16,
            Self::Sb138 => 138u16,
            Self::Sb139 => 139u16,
            Self::Sb140 => 140u16,
        }
    }
}
impl super::IsotopicComposition for AntimonyIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sb121 => Some(0.5721f64),
            Self::Sb123 => Some(0.4279f64),
            Self::Sb103
            | Self::Sb104
            | Self::Sb105
            | Self::Sb106
            | Self::Sb107
            | Self::Sb108
            | Self::Sb109
            | Self::Sb110
            | Self::Sb111
            | Self::Sb112
            | Self::Sb113
            | Self::Sb114
            | Self::Sb115
            | Self::Sb116
            | Self::Sb117
            | Self::Sb118
            | Self::Sb119
            | Self::Sb120
            | Self::Sb122
            | Self::Sb124
            | Self::Sb125
            | Self::Sb126
            | Self::Sb127
            | Self::Sb128
            | Self::Sb129
            | Self::Sb130
            | Self::Sb131
            | Self::Sb132
            | Self::Sb133
            | Self::Sb134
            | Self::Sb135
            | Self::Sb136
            | Self::Sb137
            | Self::Sb138
            | Self::Sb139
            | Self::Sb140 => None,
        }
    }
}
impl super::MostAbundantIsotope for AntimonyIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sb121
    }
}
impl From<AntimonyIsotope> for crate::Isotope {
    fn from(isotope: AntimonyIsotope) -> Self {
        crate::Isotope::Sb(isotope)
    }
}
impl From<AntimonyIsotope> for crate::Element {
    fn from(_isotope: AntimonyIsotope) -> Self {
        crate::Element::Sb
    }
}
impl TryFrom<u16> for AntimonyIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            103u16 => Ok(Self::Sb103),
            104u16 => Ok(Self::Sb104),
            105u16 => Ok(Self::Sb105),
            106u16 => Ok(Self::Sb106),
            107u16 => Ok(Self::Sb107),
            108u16 => Ok(Self::Sb108),
            109u16 => Ok(Self::Sb109),
            110u16 => Ok(Self::Sb110),
            111u16 => Ok(Self::Sb111),
            112u16 => Ok(Self::Sb112),
            113u16 => Ok(Self::Sb113),
            114u16 => Ok(Self::Sb114),
            115u16 => Ok(Self::Sb115),
            116u16 => Ok(Self::Sb116),
            117u16 => Ok(Self::Sb117),
            118u16 => Ok(Self::Sb118),
            119u16 => Ok(Self::Sb119),
            120u16 => Ok(Self::Sb120),
            121u16 => Ok(Self::Sb121),
            122u16 => Ok(Self::Sb122),
            123u16 => Ok(Self::Sb123),
            124u16 => Ok(Self::Sb124),
            125u16 => Ok(Self::Sb125),
            126u16 => Ok(Self::Sb126),
            127u16 => Ok(Self::Sb127),
            128u16 => Ok(Self::Sb128),
            129u16 => Ok(Self::Sb129),
            130u16 => Ok(Self::Sb130),
            131u16 => Ok(Self::Sb131),
            132u16 => Ok(Self::Sb132),
            133u16 => Ok(Self::Sb133),
            134u16 => Ok(Self::Sb134),
            135u16 => Ok(Self::Sb135),
            136u16 => Ok(Self::Sb136),
            137u16 => Ok(Self::Sb137),
            138u16 => Ok(Self::Sb138),
            139u16 => Ok(Self::Sb139),
            140u16 => Ok(Self::Sb140),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sb, value)),
        }
    }
}
impl std::fmt::Display for AntimonyIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sb103 => write!(f, "Sb103"),
            Self::Sb104 => write!(f, "Sb104"),
            Self::Sb105 => write!(f, "Sb105"),
            Self::Sb106 => write!(f, "Sb106"),
            Self::Sb107 => write!(f, "Sb107"),
            Self::Sb108 => write!(f, "Sb108"),
            Self::Sb109 => write!(f, "Sb109"),
            Self::Sb110 => write!(f, "Sb110"),
            Self::Sb111 => write!(f, "Sb111"),
            Self::Sb112 => write!(f, "Sb112"),
            Self::Sb113 => write!(f, "Sb113"),
            Self::Sb114 => write!(f, "Sb114"),
            Self::Sb115 => write!(f, "Sb115"),
            Self::Sb116 => write!(f, "Sb116"),
            Self::Sb117 => write!(f, "Sb117"),
            Self::Sb118 => write!(f, "Sb118"),
            Self::Sb119 => write!(f, "Sb119"),
            Self::Sb120 => write!(f, "Sb120"),
            Self::Sb121 => write!(f, "Sb121"),
            Self::Sb122 => write!(f, "Sb122"),
            Self::Sb123 => write!(f, "Sb123"),
            Self::Sb124 => write!(f, "Sb124"),
            Self::Sb125 => write!(f, "Sb125"),
            Self::Sb126 => write!(f, "Sb126"),
            Self::Sb127 => write!(f, "Sb127"),
            Self::Sb128 => write!(f, "Sb128"),
            Self::Sb129 => write!(f, "Sb129"),
            Self::Sb130 => write!(f, "Sb130"),
            Self::Sb131 => write!(f, "Sb131"),
            Self::Sb132 => write!(f, "Sb132"),
            Self::Sb133 => write!(f, "Sb133"),
            Self::Sb134 => write!(f, "Sb134"),
            Self::Sb135 => write!(f, "Sb135"),
            Self::Sb136 => write!(f, "Sb136"),
            Self::Sb137 => write!(f, "Sb137"),
            Self::Sb138 => write!(f, "Sb138"),
            Self::Sb139 => write!(f, "Sb139"),
            Self::Sb140 => write!(f, "Sb140"),
        }
    }
}
