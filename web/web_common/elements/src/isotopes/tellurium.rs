//! Isotopes of the element Tellurium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Tellurium
pub enum TelluriumIsotope {
    /// Isotope Te105 of Tellurium
    Te105,
    /// Isotope Te106 of Tellurium
    Te106,
    /// Isotope Te107 of Tellurium
    Te107,
    /// Isotope Te108 of Tellurium
    Te108,
    /// Isotope Te109 of Tellurium
    Te109,
    /// Isotope Te110 of Tellurium
    Te110,
    /// Isotope Te111 of Tellurium
    Te111,
    /// Isotope Te112 of Tellurium
    Te112,
    /// Isotope Te113 of Tellurium
    Te113,
    /// Isotope Te114 of Tellurium
    Te114,
    /// Isotope Te115 of Tellurium
    Te115,
    /// Isotope Te116 of Tellurium
    Te116,
    /// Isotope Te117 of Tellurium
    Te117,
    /// Isotope Te118 of Tellurium
    Te118,
    /// Isotope Te119 of Tellurium
    Te119,
    /// Isotope Te120 of Tellurium
    Te120,
    /// Isotope Te121 of Tellurium
    Te121,
    /// Isotope Te122 of Tellurium
    Te122,
    /// Isotope Te123 of Tellurium
    Te123,
    /// Isotope Te124 of Tellurium
    Te124,
    /// Isotope Te125 of Tellurium
    Te125,
    /// Isotope Te126 of Tellurium
    Te126,
    /// Isotope Te127 of Tellurium
    Te127,
    /// Isotope Te128 of Tellurium
    Te128,
    /// Isotope Te129 of Tellurium
    Te129,
    /// Isotope Te130 of Tellurium
    Te130,
    /// Isotope Te131 of Tellurium
    Te131,
    /// Isotope Te132 of Tellurium
    Te132,
    /// Isotope Te133 of Tellurium
    Te133,
    /// Isotope Te134 of Tellurium
    Te134,
    /// Isotope Te135 of Tellurium
    Te135,
    /// Isotope Te136 of Tellurium
    Te136,
    /// Isotope Te137 of Tellurium
    Te137,
    /// Isotope Te138 of Tellurium
    Te138,
    /// Isotope Te139 of Tellurium
    Te139,
    /// Isotope Te140 of Tellurium
    Te140,
    /// Isotope Te141 of Tellurium
    Te141,
    /// Isotope Te142 of Tellurium
    Te142,
    /// Isotope Te143 of Tellurium
    Te143,
}
impl super::RelativeAtomicMass for TelluriumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Te105 => 104.9433f64,
            Self::Te106 => 105.9375f64,
            Self::Te107 => 106.935012f64,
            Self::Te108 => 107.9293805f64,
            Self::Te109 => 108.9273045f64,
            Self::Te110 => 109.9224581f64,
            Self::Te111 => 110.9210006f64,
            Self::Te112 => 111.9167279f64,
            Self::Te113 => 112.915891f64,
            Self::Te114 => 113.912089f64,
            Self::Te115 => 114.911902f64,
            Self::Te116 => 115.90846f64,
            Self::Te117 => 116.908646f64,
            Self::Te118 => 117.905854f64,
            Self::Te119 => 118.9064071f64,
            Self::Te120 => 119.9040593f64,
            Self::Te121 => 120.904944f64,
            Self::Te122 => 121.9030435f64,
            Self::Te123 => 122.9042698f64,
            Self::Te124 => 123.9028171f64,
            Self::Te125 => 124.9044299f64,
            Self::Te126 => 125.9033109f64,
            Self::Te127 => 126.9052257f64,
            Self::Te128 => 127.90446128f64,
            Self::Te129 => 128.90659646f64,
            Self::Te130 => 129.906222748f64,
            Self::Te131 => 130.908522213f64,
            Self::Te132 => 131.9085467f64,
            Self::Te133 => 132.9109688f64,
            Self::Te134 => 133.911394f64,
            Self::Te135 => 134.9165557f64,
            Self::Te136 => 135.9201006f64,
            Self::Te137 => 136.9255989f64,
            Self::Te138 => 137.9294722f64,
            Self::Te139 => 138.9353672f64,
            Self::Te140 => 139.939499f64,
            Self::Te141 => 140.9458f64,
            Self::Te142 => 141.95022f64,
            Self::Te143 => 142.95676f64,
        }
    }
}
impl super::ElementVariant for TelluriumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Te
    }
}
impl super::MassNumber for TelluriumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Te105 => 105u16,
            Self::Te106 => 106u16,
            Self::Te107 => 107u16,
            Self::Te108 => 108u16,
            Self::Te109 => 109u16,
            Self::Te110 => 110u16,
            Self::Te111 => 111u16,
            Self::Te112 => 112u16,
            Self::Te113 => 113u16,
            Self::Te114 => 114u16,
            Self::Te115 => 115u16,
            Self::Te116 => 116u16,
            Self::Te117 => 117u16,
            Self::Te118 => 118u16,
            Self::Te119 => 119u16,
            Self::Te120 => 120u16,
            Self::Te121 => 121u16,
            Self::Te122 => 122u16,
            Self::Te123 => 123u16,
            Self::Te124 => 124u16,
            Self::Te125 => 125u16,
            Self::Te126 => 126u16,
            Self::Te127 => 127u16,
            Self::Te128 => 128u16,
            Self::Te129 => 129u16,
            Self::Te130 => 130u16,
            Self::Te131 => 131u16,
            Self::Te132 => 132u16,
            Self::Te133 => 133u16,
            Self::Te134 => 134u16,
            Self::Te135 => 135u16,
            Self::Te136 => 136u16,
            Self::Te137 => 137u16,
            Self::Te138 => 138u16,
            Self::Te139 => 139u16,
            Self::Te140 => 140u16,
            Self::Te141 => 141u16,
            Self::Te142 => 142u16,
            Self::Te143 => 143u16,
        }
    }
}
impl super::IsotopicComposition for TelluriumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Te120 => Some(0.0009f64),
            Self::Te122 => Some(0.0255f64),
            Self::Te123 => Some(0.0089f64),
            Self::Te124 => Some(0.0474f64),
            Self::Te125 => Some(0.0707f64),
            Self::Te126 => Some(0.1884f64),
            Self::Te128 => Some(0.3174f64),
            Self::Te130 => Some(0.3408f64),
            Self::Te105
            | Self::Te106
            | Self::Te107
            | Self::Te108
            | Self::Te109
            | Self::Te110
            | Self::Te111
            | Self::Te112
            | Self::Te113
            | Self::Te114
            | Self::Te115
            | Self::Te116
            | Self::Te117
            | Self::Te118
            | Self::Te119
            | Self::Te121
            | Self::Te127
            | Self::Te129
            | Self::Te131
            | Self::Te132
            | Self::Te133
            | Self::Te134
            | Self::Te135
            | Self::Te136
            | Self::Te137
            | Self::Te138
            | Self::Te139
            | Self::Te140
            | Self::Te141
            | Self::Te142
            | Self::Te143 => None,
        }
    }
}
impl super::MostAbundantIsotope for TelluriumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Te130
    }
}
impl TryFrom<u16> for TelluriumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            105u16 => Ok(Self::Te105),
            106u16 => Ok(Self::Te106),
            107u16 => Ok(Self::Te107),
            108u16 => Ok(Self::Te108),
            109u16 => Ok(Self::Te109),
            110u16 => Ok(Self::Te110),
            111u16 => Ok(Self::Te111),
            112u16 => Ok(Self::Te112),
            113u16 => Ok(Self::Te113),
            114u16 => Ok(Self::Te114),
            115u16 => Ok(Self::Te115),
            116u16 => Ok(Self::Te116),
            117u16 => Ok(Self::Te117),
            118u16 => Ok(Self::Te118),
            119u16 => Ok(Self::Te119),
            120u16 => Ok(Self::Te120),
            121u16 => Ok(Self::Te121),
            122u16 => Ok(Self::Te122),
            123u16 => Ok(Self::Te123),
            124u16 => Ok(Self::Te124),
            125u16 => Ok(Self::Te125),
            126u16 => Ok(Self::Te126),
            127u16 => Ok(Self::Te127),
            128u16 => Ok(Self::Te128),
            129u16 => Ok(Self::Te129),
            130u16 => Ok(Self::Te130),
            131u16 => Ok(Self::Te131),
            132u16 => Ok(Self::Te132),
            133u16 => Ok(Self::Te133),
            134u16 => Ok(Self::Te134),
            135u16 => Ok(Self::Te135),
            136u16 => Ok(Self::Te136),
            137u16 => Ok(Self::Te137),
            138u16 => Ok(Self::Te138),
            139u16 => Ok(Self::Te139),
            140u16 => Ok(Self::Te140),
            141u16 => Ok(Self::Te141),
            142u16 => Ok(Self::Te142),
            143u16 => Ok(Self::Te143),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Te, value)),
        }
    }
}
impl std::fmt::Display for TelluriumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Te105 => write!(f, "Te105"),
            Self::Te106 => write!(f, "Te106"),
            Self::Te107 => write!(f, "Te107"),
            Self::Te108 => write!(f, "Te108"),
            Self::Te109 => write!(f, "Te109"),
            Self::Te110 => write!(f, "Te110"),
            Self::Te111 => write!(f, "Te111"),
            Self::Te112 => write!(f, "Te112"),
            Self::Te113 => write!(f, "Te113"),
            Self::Te114 => write!(f, "Te114"),
            Self::Te115 => write!(f, "Te115"),
            Self::Te116 => write!(f, "Te116"),
            Self::Te117 => write!(f, "Te117"),
            Self::Te118 => write!(f, "Te118"),
            Self::Te119 => write!(f, "Te119"),
            Self::Te120 => write!(f, "Te120"),
            Self::Te121 => write!(f, "Te121"),
            Self::Te122 => write!(f, "Te122"),
            Self::Te123 => write!(f, "Te123"),
            Self::Te124 => write!(f, "Te124"),
            Self::Te125 => write!(f, "Te125"),
            Self::Te126 => write!(f, "Te126"),
            Self::Te127 => write!(f, "Te127"),
            Self::Te128 => write!(f, "Te128"),
            Self::Te129 => write!(f, "Te129"),
            Self::Te130 => write!(f, "Te130"),
            Self::Te131 => write!(f, "Te131"),
            Self::Te132 => write!(f, "Te132"),
            Self::Te133 => write!(f, "Te133"),
            Self::Te134 => write!(f, "Te134"),
            Self::Te135 => write!(f, "Te135"),
            Self::Te136 => write!(f, "Te136"),
            Self::Te137 => write!(f, "Te137"),
            Self::Te138 => write!(f, "Te138"),
            Self::Te139 => write!(f, "Te139"),
            Self::Te140 => write!(f, "Te140"),
            Self::Te141 => write!(f, "Te141"),
            Self::Te142 => write!(f, "Te142"),
            Self::Te143 => write!(f, "Te143"),
        }
    }
}
