//! Isotopes of the element Barium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Barium
pub enum BariumIsotope {
    /// Isotope Ba114 of Barium
    Ba114,
    /// Isotope Ba115 of Barium
    Ba115,
    /// Isotope Ba116 of Barium
    Ba116,
    /// Isotope Ba117 of Barium
    Ba117,
    /// Isotope Ba118 of Barium
    Ba118,
    /// Isotope Ba119 of Barium
    Ba119,
    /// Isotope Ba120 of Barium
    Ba120,
    /// Isotope Ba121 of Barium
    Ba121,
    /// Isotope Ba122 of Barium
    Ba122,
    /// Isotope Ba123 of Barium
    Ba123,
    /// Isotope Ba124 of Barium
    Ba124,
    /// Isotope Ba125 of Barium
    Ba125,
    /// Isotope Ba126 of Barium
    Ba126,
    /// Isotope Ba127 of Barium
    Ba127,
    /// Isotope Ba128 of Barium
    Ba128,
    /// Isotope Ba129 of Barium
    Ba129,
    /// Isotope Ba130 of Barium
    Ba130,
    /// Isotope Ba131 of Barium
    Ba131,
    /// Isotope Ba132 of Barium
    Ba132,
    /// Isotope Ba133 of Barium
    Ba133,
    /// Isotope Ba134 of Barium
    Ba134,
    /// Isotope Ba135 of Barium
    Ba135,
    /// Isotope Ba136 of Barium
    Ba136,
    /// Isotope Ba137 of Barium
    Ba137,
    /// Isotope Ba138 of Barium
    Ba138,
    /// Isotope Ba139 of Barium
    Ba139,
    /// Isotope Ba140 of Barium
    Ba140,
    /// Isotope Ba141 of Barium
    Ba141,
    /// Isotope Ba142 of Barium
    Ba142,
    /// Isotope Ba143 of Barium
    Ba143,
    /// Isotope Ba144 of Barium
    Ba144,
    /// Isotope Ba145 of Barium
    Ba145,
    /// Isotope Ba146 of Barium
    Ba146,
    /// Isotope Ba147 of Barium
    Ba147,
    /// Isotope Ba148 of Barium
    Ba148,
    /// Isotope Ba149 of Barium
    Ba149,
    /// Isotope Ba150 of Barium
    Ba150,
    /// Isotope Ba151 of Barium
    Ba151,
    /// Isotope Ba152 of Barium
    Ba152,
    /// Isotope Ba153 of Barium
    Ba153,
}
impl super::RelativeAtomicMass for BariumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ba114 => 113.95066f64,
            Self::Ba115 => 114.94737f64,
            Self::Ba116 => 115.94128f64,
            Self::Ba117 => 116.93814f64,
            Self::Ba118 => 117.93306f64,
            Self::Ba119 => 118.93066f64,
            Self::Ba120 => 119.92605f64,
            Self::Ba121 => 120.92405f64,
            Self::Ba122 => 121.919904f64,
            Self::Ba123 => 122.918781f64,
            Self::Ba124 => 123.915094f64,
            Self::Ba125 => 124.914472f64,
            Self::Ba126 => 125.91125f64,
            Self::Ba127 => 126.911091f64,
            Self::Ba128 => 127.908342f64,
            Self::Ba129 => 128.908681f64,
            Self::Ba130 => 129.9063207f64,
            Self::Ba131 => 130.906941f64,
            Self::Ba132 => 131.9050611f64,
            Self::Ba133 => 132.9060074f64,
            Self::Ba134 => 133.90450818f64,
            Self::Ba135 => 134.90568838f64,
            Self::Ba136 => 135.90457573f64,
            Self::Ba137 => 136.90582714f64,
            Self::Ba138 => 137.905247f64,
            Self::Ba139 => 138.9088411f64,
            Self::Ba140 => 139.9106057f64,
            Self::Ba141 => 140.9144033f64,
            Self::Ba142 => 141.9164324f64,
            Self::Ba143 => 142.9206253f64,
            Self::Ba144 => 143.9229549f64,
            Self::Ba145 => 144.9275184f64,
            Self::Ba146 => 145.930284f64,
            Self::Ba147 => 146.935304f64,
            Self::Ba148 => 147.938171f64,
            Self::Ba149 => 148.94308f64,
            Self::Ba150 => 149.94605f64,
            Self::Ba151 => 150.95127f64,
            Self::Ba152 => 151.95481f64,
            Self::Ba153 => 152.96036f64,
        }
    }
}
impl super::ElementVariant for BariumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ba
    }
}
impl super::MassNumber for BariumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ba114 => 114u16,
            Self::Ba115 => 115u16,
            Self::Ba116 => 116u16,
            Self::Ba117 => 117u16,
            Self::Ba118 => 118u16,
            Self::Ba119 => 119u16,
            Self::Ba120 => 120u16,
            Self::Ba121 => 121u16,
            Self::Ba122 => 122u16,
            Self::Ba123 => 123u16,
            Self::Ba124 => 124u16,
            Self::Ba125 => 125u16,
            Self::Ba126 => 126u16,
            Self::Ba127 => 127u16,
            Self::Ba128 => 128u16,
            Self::Ba129 => 129u16,
            Self::Ba130 => 130u16,
            Self::Ba131 => 131u16,
            Self::Ba132 => 132u16,
            Self::Ba133 => 133u16,
            Self::Ba134 => 134u16,
            Self::Ba135 => 135u16,
            Self::Ba136 => 136u16,
            Self::Ba137 => 137u16,
            Self::Ba138 => 138u16,
            Self::Ba139 => 139u16,
            Self::Ba140 => 140u16,
            Self::Ba141 => 141u16,
            Self::Ba142 => 142u16,
            Self::Ba143 => 143u16,
            Self::Ba144 => 144u16,
            Self::Ba145 => 145u16,
            Self::Ba146 => 146u16,
            Self::Ba147 => 147u16,
            Self::Ba148 => 148u16,
            Self::Ba149 => 149u16,
            Self::Ba150 => 150u16,
            Self::Ba151 => 151u16,
            Self::Ba152 => 152u16,
            Self::Ba153 => 153u16,
        }
    }
}
impl super::IsotopicComposition for BariumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ba130 => Some(0.00106f64),
            Self::Ba132 => Some(0.00101f64),
            Self::Ba134 => Some(0.02417f64),
            Self::Ba135 => Some(0.06592f64),
            Self::Ba136 => Some(0.07854f64),
            Self::Ba137 => Some(0.11232f64),
            Self::Ba138 => Some(0.71698f64),
            Self::Ba114
            | Self::Ba115
            | Self::Ba116
            | Self::Ba117
            | Self::Ba118
            | Self::Ba119
            | Self::Ba120
            | Self::Ba121
            | Self::Ba122
            | Self::Ba123
            | Self::Ba124
            | Self::Ba125
            | Self::Ba126
            | Self::Ba127
            | Self::Ba128
            | Self::Ba129
            | Self::Ba131
            | Self::Ba133
            | Self::Ba139
            | Self::Ba140
            | Self::Ba141
            | Self::Ba142
            | Self::Ba143
            | Self::Ba144
            | Self::Ba145
            | Self::Ba146
            | Self::Ba147
            | Self::Ba148
            | Self::Ba149
            | Self::Ba150
            | Self::Ba151
            | Self::Ba152
            | Self::Ba153 => None,
        }
    }
}
impl super::MostAbundantIsotope for BariumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ba138
    }
}
impl From<BariumIsotope> for crate::Isotope {
    fn from(isotope: BariumIsotope) -> Self {
        crate::Isotope::Ba(isotope)
    }
}
impl From<BariumIsotope> for crate::Element {
    fn from(_isotope: BariumIsotope) -> Self {
        crate::Element::Ba
    }
}
impl TryFrom<u16> for BariumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            114u16 => Ok(Self::Ba114),
            115u16 => Ok(Self::Ba115),
            116u16 => Ok(Self::Ba116),
            117u16 => Ok(Self::Ba117),
            118u16 => Ok(Self::Ba118),
            119u16 => Ok(Self::Ba119),
            120u16 => Ok(Self::Ba120),
            121u16 => Ok(Self::Ba121),
            122u16 => Ok(Self::Ba122),
            123u16 => Ok(Self::Ba123),
            124u16 => Ok(Self::Ba124),
            125u16 => Ok(Self::Ba125),
            126u16 => Ok(Self::Ba126),
            127u16 => Ok(Self::Ba127),
            128u16 => Ok(Self::Ba128),
            129u16 => Ok(Self::Ba129),
            130u16 => Ok(Self::Ba130),
            131u16 => Ok(Self::Ba131),
            132u16 => Ok(Self::Ba132),
            133u16 => Ok(Self::Ba133),
            134u16 => Ok(Self::Ba134),
            135u16 => Ok(Self::Ba135),
            136u16 => Ok(Self::Ba136),
            137u16 => Ok(Self::Ba137),
            138u16 => Ok(Self::Ba138),
            139u16 => Ok(Self::Ba139),
            140u16 => Ok(Self::Ba140),
            141u16 => Ok(Self::Ba141),
            142u16 => Ok(Self::Ba142),
            143u16 => Ok(Self::Ba143),
            144u16 => Ok(Self::Ba144),
            145u16 => Ok(Self::Ba145),
            146u16 => Ok(Self::Ba146),
            147u16 => Ok(Self::Ba147),
            148u16 => Ok(Self::Ba148),
            149u16 => Ok(Self::Ba149),
            150u16 => Ok(Self::Ba150),
            151u16 => Ok(Self::Ba151),
            152u16 => Ok(Self::Ba152),
            153u16 => Ok(Self::Ba153),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ba, value)),
        }
    }
}
impl std::fmt::Display for BariumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ba114 => write!(f, "Ba114"),
            Self::Ba115 => write!(f, "Ba115"),
            Self::Ba116 => write!(f, "Ba116"),
            Self::Ba117 => write!(f, "Ba117"),
            Self::Ba118 => write!(f, "Ba118"),
            Self::Ba119 => write!(f, "Ba119"),
            Self::Ba120 => write!(f, "Ba120"),
            Self::Ba121 => write!(f, "Ba121"),
            Self::Ba122 => write!(f, "Ba122"),
            Self::Ba123 => write!(f, "Ba123"),
            Self::Ba124 => write!(f, "Ba124"),
            Self::Ba125 => write!(f, "Ba125"),
            Self::Ba126 => write!(f, "Ba126"),
            Self::Ba127 => write!(f, "Ba127"),
            Self::Ba128 => write!(f, "Ba128"),
            Self::Ba129 => write!(f, "Ba129"),
            Self::Ba130 => write!(f, "Ba130"),
            Self::Ba131 => write!(f, "Ba131"),
            Self::Ba132 => write!(f, "Ba132"),
            Self::Ba133 => write!(f, "Ba133"),
            Self::Ba134 => write!(f, "Ba134"),
            Self::Ba135 => write!(f, "Ba135"),
            Self::Ba136 => write!(f, "Ba136"),
            Self::Ba137 => write!(f, "Ba137"),
            Self::Ba138 => write!(f, "Ba138"),
            Self::Ba139 => write!(f, "Ba139"),
            Self::Ba140 => write!(f, "Ba140"),
            Self::Ba141 => write!(f, "Ba141"),
            Self::Ba142 => write!(f, "Ba142"),
            Self::Ba143 => write!(f, "Ba143"),
            Self::Ba144 => write!(f, "Ba144"),
            Self::Ba145 => write!(f, "Ba145"),
            Self::Ba146 => write!(f, "Ba146"),
            Self::Ba147 => write!(f, "Ba147"),
            Self::Ba148 => write!(f, "Ba148"),
            Self::Ba149 => write!(f, "Ba149"),
            Self::Ba150 => write!(f, "Ba150"),
            Self::Ba151 => write!(f, "Ba151"),
            Self::Ba152 => write!(f, "Ba152"),
            Self::Ba153 => write!(f, "Ba153"),
        }
    }
}
