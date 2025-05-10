//! Isotopes of the element Europium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Europium
pub enum EuropiumIsotope {
    /// Isotope Eu130 of Europium
    Eu130,
    /// Isotope Eu131 of Europium
    Eu131,
    /// Isotope Eu132 of Europium
    Eu132,
    /// Isotope Eu133 of Europium
    Eu133,
    /// Isotope Eu134 of Europium
    Eu134,
    /// Isotope Eu135 of Europium
    Eu135,
    /// Isotope Eu136 of Europium
    Eu136,
    /// Isotope Eu137 of Europium
    Eu137,
    /// Isotope Eu138 of Europium
    Eu138,
    /// Isotope Eu139 of Europium
    Eu139,
    /// Isotope Eu140 of Europium
    Eu140,
    /// Isotope Eu141 of Europium
    Eu141,
    /// Isotope Eu142 of Europium
    Eu142,
    /// Isotope Eu143 of Europium
    Eu143,
    /// Isotope Eu144 of Europium
    Eu144,
    /// Isotope Eu145 of Europium
    Eu145,
    /// Isotope Eu146 of Europium
    Eu146,
    /// Isotope Eu147 of Europium
    Eu147,
    /// Isotope Eu148 of Europium
    Eu148,
    /// Isotope Eu149 of Europium
    Eu149,
    /// Isotope Eu150 of Europium
    Eu150,
    /// Isotope Eu151 of Europium
    Eu151,
    /// Isotope Eu152 of Europium
    Eu152,
    /// Isotope Eu153 of Europium
    Eu153,
    /// Isotope Eu154 of Europium
    Eu154,
    /// Isotope Eu155 of Europium
    Eu155,
    /// Isotope Eu156 of Europium
    Eu156,
    /// Isotope Eu157 of Europium
    Eu157,
    /// Isotope Eu158 of Europium
    Eu158,
    /// Isotope Eu159 of Europium
    Eu159,
    /// Isotope Eu160 of Europium
    Eu160,
    /// Isotope Eu161 of Europium
    Eu161,
    /// Isotope Eu162 of Europium
    Eu162,
    /// Isotope Eu163 of Europium
    Eu163,
    /// Isotope Eu164 of Europium
    Eu164,
    /// Isotope Eu165 of Europium
    Eu165,
    /// Isotope Eu166 of Europium
    Eu166,
    /// Isotope Eu167 of Europium
    Eu167,
}
impl super::RelativeAtomicMass for EuropiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Eu130 => 129.96369f64,
            Self::Eu131 => 130.95784f64,
            Self::Eu132 => 131.95467f64,
            Self::Eu133 => 132.94929f64,
            Self::Eu134 => 133.9464f64,
            Self::Eu135 => 134.94187f64,
            Self::Eu136 => 135.93962f64,
            Self::Eu137 => 136.93546f64,
            Self::Eu138 => 137.933709f64,
            Self::Eu139 => 138.929792f64,
            Self::Eu140 => 139.928088f64,
            Self::Eu141 => 140.924932f64,
            Self::Eu142 => 141.923442f64,
            Self::Eu143 => 142.920299f64,
            Self::Eu144 => 143.91882f64,
            Self::Eu145 => 144.9162726f64,
            Self::Eu146 => 145.917211f64,
            Self::Eu147 => 146.9167527f64,
            Self::Eu148 => 147.918089f64,
            Self::Eu149 => 148.9179378f64,
            Self::Eu150 => 149.9197077f64,
            Self::Eu151 => 150.9198578f64,
            Self::Eu152 => 151.9217522f64,
            Self::Eu153 => 152.921238f64,
            Self::Eu154 => 153.922987f64,
            Self::Eu155 => 154.9229011f64,
            Self::Eu156 => 155.9247605f64,
            Self::Eu157 => 156.9254334f64,
            Self::Eu158 => 157.927799f64,
            Self::Eu159 => 158.9291001f64,
            Self::Eu160 => 159.931851f64,
            Self::Eu161 => 160.933664f64,
            Self::Eu162 => 161.936989f64,
            Self::Eu163 => 162.939196f64,
            Self::Eu164 => 163.94274f64,
            Self::Eu165 => 164.94559f64,
            Self::Eu166 => 165.94962f64,
            Self::Eu167 => 166.95289f64,
        }
    }
}
impl super::ElementVariant for EuropiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Eu
    }
}
impl super::MassNumber for EuropiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Eu130 => 130u16,
            Self::Eu131 => 131u16,
            Self::Eu132 => 132u16,
            Self::Eu133 => 133u16,
            Self::Eu134 => 134u16,
            Self::Eu135 => 135u16,
            Self::Eu136 => 136u16,
            Self::Eu137 => 137u16,
            Self::Eu138 => 138u16,
            Self::Eu139 => 139u16,
            Self::Eu140 => 140u16,
            Self::Eu141 => 141u16,
            Self::Eu142 => 142u16,
            Self::Eu143 => 143u16,
            Self::Eu144 => 144u16,
            Self::Eu145 => 145u16,
            Self::Eu146 => 146u16,
            Self::Eu147 => 147u16,
            Self::Eu148 => 148u16,
            Self::Eu149 => 149u16,
            Self::Eu150 => 150u16,
            Self::Eu151 => 151u16,
            Self::Eu152 => 152u16,
            Self::Eu153 => 153u16,
            Self::Eu154 => 154u16,
            Self::Eu155 => 155u16,
            Self::Eu156 => 156u16,
            Self::Eu157 => 157u16,
            Self::Eu158 => 158u16,
            Self::Eu159 => 159u16,
            Self::Eu160 => 160u16,
            Self::Eu161 => 161u16,
            Self::Eu162 => 162u16,
            Self::Eu163 => 163u16,
            Self::Eu164 => 164u16,
            Self::Eu165 => 165u16,
            Self::Eu166 => 166u16,
            Self::Eu167 => 167u16,
        }
    }
}
impl super::IsotopicComposition for EuropiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Eu151 => Some(0.4781f64),
            Self::Eu153 => Some(0.5219f64),
            Self::Eu130
            | Self::Eu131
            | Self::Eu132
            | Self::Eu133
            | Self::Eu134
            | Self::Eu135
            | Self::Eu136
            | Self::Eu137
            | Self::Eu138
            | Self::Eu139
            | Self::Eu140
            | Self::Eu141
            | Self::Eu142
            | Self::Eu143
            | Self::Eu144
            | Self::Eu145
            | Self::Eu146
            | Self::Eu147
            | Self::Eu148
            | Self::Eu149
            | Self::Eu150
            | Self::Eu152
            | Self::Eu154
            | Self::Eu155
            | Self::Eu156
            | Self::Eu157
            | Self::Eu158
            | Self::Eu159
            | Self::Eu160
            | Self::Eu161
            | Self::Eu162
            | Self::Eu163
            | Self::Eu164
            | Self::Eu165
            | Self::Eu166
            | Self::Eu167 => None,
        }
    }
}
impl super::MostAbundantIsotope for EuropiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Eu153
    }
}
impl TryFrom<u16> for EuropiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            130u16 => Ok(Self::Eu130),
            131u16 => Ok(Self::Eu131),
            132u16 => Ok(Self::Eu132),
            133u16 => Ok(Self::Eu133),
            134u16 => Ok(Self::Eu134),
            135u16 => Ok(Self::Eu135),
            136u16 => Ok(Self::Eu136),
            137u16 => Ok(Self::Eu137),
            138u16 => Ok(Self::Eu138),
            139u16 => Ok(Self::Eu139),
            140u16 => Ok(Self::Eu140),
            141u16 => Ok(Self::Eu141),
            142u16 => Ok(Self::Eu142),
            143u16 => Ok(Self::Eu143),
            144u16 => Ok(Self::Eu144),
            145u16 => Ok(Self::Eu145),
            146u16 => Ok(Self::Eu146),
            147u16 => Ok(Self::Eu147),
            148u16 => Ok(Self::Eu148),
            149u16 => Ok(Self::Eu149),
            150u16 => Ok(Self::Eu150),
            151u16 => Ok(Self::Eu151),
            152u16 => Ok(Self::Eu152),
            153u16 => Ok(Self::Eu153),
            154u16 => Ok(Self::Eu154),
            155u16 => Ok(Self::Eu155),
            156u16 => Ok(Self::Eu156),
            157u16 => Ok(Self::Eu157),
            158u16 => Ok(Self::Eu158),
            159u16 => Ok(Self::Eu159),
            160u16 => Ok(Self::Eu160),
            161u16 => Ok(Self::Eu161),
            162u16 => Ok(Self::Eu162),
            163u16 => Ok(Self::Eu163),
            164u16 => Ok(Self::Eu164),
            165u16 => Ok(Self::Eu165),
            166u16 => Ok(Self::Eu166),
            167u16 => Ok(Self::Eu167),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Eu, value)),
        }
    }
}
impl std::fmt::Display for EuropiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eu130 => write!(f, "Eu130"),
            Self::Eu131 => write!(f, "Eu131"),
            Self::Eu132 => write!(f, "Eu132"),
            Self::Eu133 => write!(f, "Eu133"),
            Self::Eu134 => write!(f, "Eu134"),
            Self::Eu135 => write!(f, "Eu135"),
            Self::Eu136 => write!(f, "Eu136"),
            Self::Eu137 => write!(f, "Eu137"),
            Self::Eu138 => write!(f, "Eu138"),
            Self::Eu139 => write!(f, "Eu139"),
            Self::Eu140 => write!(f, "Eu140"),
            Self::Eu141 => write!(f, "Eu141"),
            Self::Eu142 => write!(f, "Eu142"),
            Self::Eu143 => write!(f, "Eu143"),
            Self::Eu144 => write!(f, "Eu144"),
            Self::Eu145 => write!(f, "Eu145"),
            Self::Eu146 => write!(f, "Eu146"),
            Self::Eu147 => write!(f, "Eu147"),
            Self::Eu148 => write!(f, "Eu148"),
            Self::Eu149 => write!(f, "Eu149"),
            Self::Eu150 => write!(f, "Eu150"),
            Self::Eu151 => write!(f, "Eu151"),
            Self::Eu152 => write!(f, "Eu152"),
            Self::Eu153 => write!(f, "Eu153"),
            Self::Eu154 => write!(f, "Eu154"),
            Self::Eu155 => write!(f, "Eu155"),
            Self::Eu156 => write!(f, "Eu156"),
            Self::Eu157 => write!(f, "Eu157"),
            Self::Eu158 => write!(f, "Eu158"),
            Self::Eu159 => write!(f, "Eu159"),
            Self::Eu160 => write!(f, "Eu160"),
            Self::Eu161 => write!(f, "Eu161"),
            Self::Eu162 => write!(f, "Eu162"),
            Self::Eu163 => write!(f, "Eu163"),
            Self::Eu164 => write!(f, "Eu164"),
            Self::Eu165 => write!(f, "Eu165"),
            Self::Eu166 => write!(f, "Eu166"),
            Self::Eu167 => write!(f, "Eu167"),
        }
    }
}
