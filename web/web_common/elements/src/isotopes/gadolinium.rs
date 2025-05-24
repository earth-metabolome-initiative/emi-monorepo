//! Isotopes of the element Gadolinium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Gadolinium
pub enum GadoliniumIsotope {
    /// Isotope Gd133 of Gadolinium
    Gd133,
    /// Isotope Gd134 of Gadolinium
    Gd134,
    /// Isotope Gd135 of Gadolinium
    Gd135,
    /// Isotope Gd136 of Gadolinium
    Gd136,
    /// Isotope Gd137 of Gadolinium
    Gd137,
    /// Isotope Gd138 of Gadolinium
    Gd138,
    /// Isotope Gd139 of Gadolinium
    Gd139,
    /// Isotope Gd140 of Gadolinium
    Gd140,
    /// Isotope Gd141 of Gadolinium
    Gd141,
    /// Isotope Gd142 of Gadolinium
    Gd142,
    /// Isotope Gd143 of Gadolinium
    Gd143,
    /// Isotope Gd144 of Gadolinium
    Gd144,
    /// Isotope Gd145 of Gadolinium
    Gd145,
    /// Isotope Gd146 of Gadolinium
    Gd146,
    /// Isotope Gd147 of Gadolinium
    Gd147,
    /// Isotope Gd148 of Gadolinium
    Gd148,
    /// Isotope Gd149 of Gadolinium
    Gd149,
    /// Isotope Gd150 of Gadolinium
    Gd150,
    /// Isotope Gd151 of Gadolinium
    Gd151,
    /// Isotope Gd152 of Gadolinium
    Gd152,
    /// Isotope Gd153 of Gadolinium
    Gd153,
    /// Isotope Gd154 of Gadolinium
    Gd154,
    /// Isotope Gd155 of Gadolinium
    Gd155,
    /// Isotope Gd156 of Gadolinium
    Gd156,
    /// Isotope Gd157 of Gadolinium
    Gd157,
    /// Isotope Gd158 of Gadolinium
    Gd158,
    /// Isotope Gd159 of Gadolinium
    Gd159,
    /// Isotope Gd160 of Gadolinium
    Gd160,
    /// Isotope Gd161 of Gadolinium
    Gd161,
    /// Isotope Gd162 of Gadolinium
    Gd162,
    /// Isotope Gd163 of Gadolinium
    Gd163,
    /// Isotope Gd164 of Gadolinium
    Gd164,
    /// Isotope Gd165 of Gadolinium
    Gd165,
    /// Isotope Gd166 of Gadolinium
    Gd166,
    /// Isotope Gd167 of Gadolinium
    Gd167,
    /// Isotope Gd168 of Gadolinium
    Gd168,
    /// Isotope Gd169 of Gadolinium
    Gd169,
}
impl super::RelativeAtomicMass for GadoliniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Gd133 => 132.96133f64,
            Self::Gd134 => 133.95566f64,
            Self::Gd135 => 134.95245f64,
            Self::Gd136 => 135.9473f64,
            Self::Gd137 => 136.94502f64,
            Self::Gd138 => 137.94025f64,
            Self::Gd139 => 138.93813f64,
            Self::Gd140 => 139.933674f64,
            Self::Gd141 => 140.932126f64,
            Self::Gd142 => 141.928116f64,
            Self::Gd143 => 142.92675f64,
            Self::Gd144 => 143.922963f64,
            Self::Gd145 => 144.921713f64,
            Self::Gd146 => 145.9183188f64,
            Self::Gd147 => 146.9191014f64,
            Self::Gd148 => 147.9181215f64,
            Self::Gd149 => 148.9193481f64,
            Self::Gd150 => 149.9186644f64,
            Self::Gd151 => 150.920356f64,
            Self::Gd152 => 151.9197995f64,
            Self::Gd153 => 152.921758f64,
            Self::Gd154 => 153.9208741f64,
            Self::Gd155 => 154.9226305f64,
            Self::Gd156 => 155.9221312f64,
            Self::Gd157 => 156.9239686f64,
            Self::Gd158 => 157.9241123f64,
            Self::Gd159 => 158.926397f64,
            Self::Gd160 => 159.9270624f64,
            Self::Gd161 => 160.9296775f64,
            Self::Gd162 => 161.930993f64,
            Self::Gd163 => 162.9341769f64,
            Self::Gd164 => 163.93583f64,
            Self::Gd165 => 164.93936f64,
            Self::Gd166 => 165.94146f64,
            Self::Gd167 => 166.94545f64,
            Self::Gd168 => 167.94808f64,
            Self::Gd169 => 168.9526f64,
        }
    }
}
impl super::ElementVariant for GadoliniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Gd
    }
}
impl super::MassNumber for GadoliniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Gd133 => 133u16,
            Self::Gd134 => 134u16,
            Self::Gd135 => 135u16,
            Self::Gd136 => 136u16,
            Self::Gd137 => 137u16,
            Self::Gd138 => 138u16,
            Self::Gd139 => 139u16,
            Self::Gd140 => 140u16,
            Self::Gd141 => 141u16,
            Self::Gd142 => 142u16,
            Self::Gd143 => 143u16,
            Self::Gd144 => 144u16,
            Self::Gd145 => 145u16,
            Self::Gd146 => 146u16,
            Self::Gd147 => 147u16,
            Self::Gd148 => 148u16,
            Self::Gd149 => 149u16,
            Self::Gd150 => 150u16,
            Self::Gd151 => 151u16,
            Self::Gd152 => 152u16,
            Self::Gd153 => 153u16,
            Self::Gd154 => 154u16,
            Self::Gd155 => 155u16,
            Self::Gd156 => 156u16,
            Self::Gd157 => 157u16,
            Self::Gd158 => 158u16,
            Self::Gd159 => 159u16,
            Self::Gd160 => 160u16,
            Self::Gd161 => 161u16,
            Self::Gd162 => 162u16,
            Self::Gd163 => 163u16,
            Self::Gd164 => 164u16,
            Self::Gd165 => 165u16,
            Self::Gd166 => 166u16,
            Self::Gd167 => 167u16,
            Self::Gd168 => 168u16,
            Self::Gd169 => 169u16,
        }
    }
}
impl super::IsotopicComposition for GadoliniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Gd152 => Some(0.002f64),
            Self::Gd154 => Some(0.0218f64),
            Self::Gd155 => Some(0.148f64),
            Self::Gd156 => Some(0.2047f64),
            Self::Gd157 => Some(0.1565f64),
            Self::Gd158 => Some(0.2484f64),
            Self::Gd160 => Some(0.2186f64),
            Self::Gd133
            | Self::Gd134
            | Self::Gd135
            | Self::Gd136
            | Self::Gd137
            | Self::Gd138
            | Self::Gd139
            | Self::Gd140
            | Self::Gd141
            | Self::Gd142
            | Self::Gd143
            | Self::Gd144
            | Self::Gd145
            | Self::Gd146
            | Self::Gd147
            | Self::Gd148
            | Self::Gd149
            | Self::Gd150
            | Self::Gd151
            | Self::Gd153
            | Self::Gd159
            | Self::Gd161
            | Self::Gd162
            | Self::Gd163
            | Self::Gd164
            | Self::Gd165
            | Self::Gd166
            | Self::Gd167
            | Self::Gd168
            | Self::Gd169 => None,
        }
    }
}
impl super::MostAbundantIsotope for GadoliniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Gd158
    }
}
impl From<GadoliniumIsotope> for crate::Isotope {
    fn from(isotope: GadoliniumIsotope) -> Self {
        crate::Isotope::Gd(isotope)
    }
}
impl From<GadoliniumIsotope> for crate::Element {
    fn from(_isotope: GadoliniumIsotope) -> Self {
        crate::Element::Gd
    }
}
impl TryFrom<u16> for GadoliniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            133u16 => Ok(Self::Gd133),
            134u16 => Ok(Self::Gd134),
            135u16 => Ok(Self::Gd135),
            136u16 => Ok(Self::Gd136),
            137u16 => Ok(Self::Gd137),
            138u16 => Ok(Self::Gd138),
            139u16 => Ok(Self::Gd139),
            140u16 => Ok(Self::Gd140),
            141u16 => Ok(Self::Gd141),
            142u16 => Ok(Self::Gd142),
            143u16 => Ok(Self::Gd143),
            144u16 => Ok(Self::Gd144),
            145u16 => Ok(Self::Gd145),
            146u16 => Ok(Self::Gd146),
            147u16 => Ok(Self::Gd147),
            148u16 => Ok(Self::Gd148),
            149u16 => Ok(Self::Gd149),
            150u16 => Ok(Self::Gd150),
            151u16 => Ok(Self::Gd151),
            152u16 => Ok(Self::Gd152),
            153u16 => Ok(Self::Gd153),
            154u16 => Ok(Self::Gd154),
            155u16 => Ok(Self::Gd155),
            156u16 => Ok(Self::Gd156),
            157u16 => Ok(Self::Gd157),
            158u16 => Ok(Self::Gd158),
            159u16 => Ok(Self::Gd159),
            160u16 => Ok(Self::Gd160),
            161u16 => Ok(Self::Gd161),
            162u16 => Ok(Self::Gd162),
            163u16 => Ok(Self::Gd163),
            164u16 => Ok(Self::Gd164),
            165u16 => Ok(Self::Gd165),
            166u16 => Ok(Self::Gd166),
            167u16 => Ok(Self::Gd167),
            168u16 => Ok(Self::Gd168),
            169u16 => Ok(Self::Gd169),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Gd, value)),
        }
    }
}
impl std::fmt::Display for GadoliniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gd133 => write!(f, "Gd133"),
            Self::Gd134 => write!(f, "Gd134"),
            Self::Gd135 => write!(f, "Gd135"),
            Self::Gd136 => write!(f, "Gd136"),
            Self::Gd137 => write!(f, "Gd137"),
            Self::Gd138 => write!(f, "Gd138"),
            Self::Gd139 => write!(f, "Gd139"),
            Self::Gd140 => write!(f, "Gd140"),
            Self::Gd141 => write!(f, "Gd141"),
            Self::Gd142 => write!(f, "Gd142"),
            Self::Gd143 => write!(f, "Gd143"),
            Self::Gd144 => write!(f, "Gd144"),
            Self::Gd145 => write!(f, "Gd145"),
            Self::Gd146 => write!(f, "Gd146"),
            Self::Gd147 => write!(f, "Gd147"),
            Self::Gd148 => write!(f, "Gd148"),
            Self::Gd149 => write!(f, "Gd149"),
            Self::Gd150 => write!(f, "Gd150"),
            Self::Gd151 => write!(f, "Gd151"),
            Self::Gd152 => write!(f, "Gd152"),
            Self::Gd153 => write!(f, "Gd153"),
            Self::Gd154 => write!(f, "Gd154"),
            Self::Gd155 => write!(f, "Gd155"),
            Self::Gd156 => write!(f, "Gd156"),
            Self::Gd157 => write!(f, "Gd157"),
            Self::Gd158 => write!(f, "Gd158"),
            Self::Gd159 => write!(f, "Gd159"),
            Self::Gd160 => write!(f, "Gd160"),
            Self::Gd161 => write!(f, "Gd161"),
            Self::Gd162 => write!(f, "Gd162"),
            Self::Gd163 => write!(f, "Gd163"),
            Self::Gd164 => write!(f, "Gd164"),
            Self::Gd165 => write!(f, "Gd165"),
            Self::Gd166 => write!(f, "Gd166"),
            Self::Gd167 => write!(f, "Gd167"),
            Self::Gd168 => write!(f, "Gd168"),
            Self::Gd169 => write!(f, "Gd169"),
        }
    }
}
