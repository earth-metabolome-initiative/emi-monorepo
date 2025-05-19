//! Isotopes of the element Samarium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Samarium
pub enum SamariumIsotope {
    /// Isotope Sm128 of Samarium
    Sm128,
    /// Isotope Sm129 of Samarium
    Sm129,
    /// Isotope Sm130 of Samarium
    Sm130,
    /// Isotope Sm131 of Samarium
    Sm131,
    /// Isotope Sm132 of Samarium
    Sm132,
    /// Isotope Sm133 of Samarium
    Sm133,
    /// Isotope Sm134 of Samarium
    Sm134,
    /// Isotope Sm135 of Samarium
    Sm135,
    /// Isotope Sm136 of Samarium
    Sm136,
    /// Isotope Sm137 of Samarium
    Sm137,
    /// Isotope Sm138 of Samarium
    Sm138,
    /// Isotope Sm139 of Samarium
    Sm139,
    /// Isotope Sm140 of Samarium
    Sm140,
    /// Isotope Sm141 of Samarium
    Sm141,
    /// Isotope Sm142 of Samarium
    Sm142,
    /// Isotope Sm143 of Samarium
    Sm143,
    /// Isotope Sm144 of Samarium
    Sm144,
    /// Isotope Sm145 of Samarium
    Sm145,
    /// Isotope Sm146 of Samarium
    Sm146,
    /// Isotope Sm147 of Samarium
    Sm147,
    /// Isotope Sm148 of Samarium
    Sm148,
    /// Isotope Sm149 of Samarium
    Sm149,
    /// Isotope Sm150 of Samarium
    Sm150,
    /// Isotope Sm151 of Samarium
    Sm151,
    /// Isotope Sm152 of Samarium
    Sm152,
    /// Isotope Sm153 of Samarium
    Sm153,
    /// Isotope Sm154 of Samarium
    Sm154,
    /// Isotope Sm155 of Samarium
    Sm155,
    /// Isotope Sm156 of Samarium
    Sm156,
    /// Isotope Sm157 of Samarium
    Sm157,
    /// Isotope Sm158 of Samarium
    Sm158,
    /// Isotope Sm159 of Samarium
    Sm159,
    /// Isotope Sm160 of Samarium
    Sm160,
    /// Isotope Sm161 of Samarium
    Sm161,
    /// Isotope Sm162 of Samarium
    Sm162,
    /// Isotope Sm163 of Samarium
    Sm163,
    /// Isotope Sm164 of Samarium
    Sm164,
    /// Isotope Sm165 of Samarium
    Sm165,
}
impl super::RelativeAtomicMass for SamariumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Sm128 => 127.95842f64,
            Self::Sm129 => 128.95476f64,
            Self::Sm130 => 129.949f64,
            Self::Sm131 => 130.94618f64,
            Self::Sm132 => 131.94087f64,
            Self::Sm133 => 132.93856f64,
            Self::Sm134 => 133.93411f64,
            Self::Sm135 => 134.93252f64,
            Self::Sm136 => 135.928276f64,
            Self::Sm137 => 136.926971f64,
            Self::Sm138 => 137.923244f64,
            Self::Sm139 => 138.922297f64,
            Self::Sm140 => 139.918995f64,
            Self::Sm141 => 140.9184816f64,
            Self::Sm142 => 141.9152044f64,
            Self::Sm143 => 142.9146353f64,
            Self::Sm144 => 143.9120065f64,
            Self::Sm145 => 144.9134173f64,
            Self::Sm146 => 145.913047f64,
            Self::Sm147 => 146.9149044f64,
            Self::Sm148 => 147.9148292f64,
            Self::Sm149 => 148.9171921f64,
            Self::Sm150 => 149.9172829f64,
            Self::Sm151 => 150.9199398f64,
            Self::Sm152 => 151.9197397f64,
            Self::Sm153 => 152.9221047f64,
            Self::Sm154 => 153.9222169f64,
            Self::Sm155 => 154.9246477f64,
            Self::Sm156 => 155.925536f64,
            Self::Sm157 => 156.9284187f64,
            Self::Sm158 => 157.929951f64,
            Self::Sm159 => 158.9332172f64,
            Self::Sm160 => 159.9353353f64,
            Self::Sm161 => 160.9391602f64,
            Self::Sm162 => 161.94146f64,
            Self::Sm163 => 162.94555f64,
            Self::Sm164 => 163.94836f64,
            Self::Sm165 => 164.95297f64,
        }
    }
}
impl super::ElementVariant for SamariumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Sm
    }
}
impl super::MassNumber for SamariumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Sm128 => 128u16,
            Self::Sm129 => 129u16,
            Self::Sm130 => 130u16,
            Self::Sm131 => 131u16,
            Self::Sm132 => 132u16,
            Self::Sm133 => 133u16,
            Self::Sm134 => 134u16,
            Self::Sm135 => 135u16,
            Self::Sm136 => 136u16,
            Self::Sm137 => 137u16,
            Self::Sm138 => 138u16,
            Self::Sm139 => 139u16,
            Self::Sm140 => 140u16,
            Self::Sm141 => 141u16,
            Self::Sm142 => 142u16,
            Self::Sm143 => 143u16,
            Self::Sm144 => 144u16,
            Self::Sm145 => 145u16,
            Self::Sm146 => 146u16,
            Self::Sm147 => 147u16,
            Self::Sm148 => 148u16,
            Self::Sm149 => 149u16,
            Self::Sm150 => 150u16,
            Self::Sm151 => 151u16,
            Self::Sm152 => 152u16,
            Self::Sm153 => 153u16,
            Self::Sm154 => 154u16,
            Self::Sm155 => 155u16,
            Self::Sm156 => 156u16,
            Self::Sm157 => 157u16,
            Self::Sm158 => 158u16,
            Self::Sm159 => 159u16,
            Self::Sm160 => 160u16,
            Self::Sm161 => 161u16,
            Self::Sm162 => 162u16,
            Self::Sm163 => 163u16,
            Self::Sm164 => 164u16,
            Self::Sm165 => 165u16,
        }
    }
}
impl super::IsotopicComposition for SamariumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Sm144 => Some(0.0307f64),
            Self::Sm147 => Some(0.1499f64),
            Self::Sm148 => Some(0.1124f64),
            Self::Sm149 => Some(0.1382f64),
            Self::Sm150 => Some(0.0738f64),
            Self::Sm152 => Some(0.2675f64),
            Self::Sm154 => Some(0.2275f64),
            Self::Sm128
            | Self::Sm129
            | Self::Sm130
            | Self::Sm131
            | Self::Sm132
            | Self::Sm133
            | Self::Sm134
            | Self::Sm135
            | Self::Sm136
            | Self::Sm137
            | Self::Sm138
            | Self::Sm139
            | Self::Sm140
            | Self::Sm141
            | Self::Sm142
            | Self::Sm143
            | Self::Sm145
            | Self::Sm146
            | Self::Sm151
            | Self::Sm153
            | Self::Sm155
            | Self::Sm156
            | Self::Sm157
            | Self::Sm158
            | Self::Sm159
            | Self::Sm160
            | Self::Sm161
            | Self::Sm162
            | Self::Sm163
            | Self::Sm164
            | Self::Sm165 => None,
        }
    }
}
impl super::MostAbundantIsotope for SamariumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Sm152
    }
}
impl TryFrom<u16> for SamariumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            128u16 => Ok(Self::Sm128),
            129u16 => Ok(Self::Sm129),
            130u16 => Ok(Self::Sm130),
            131u16 => Ok(Self::Sm131),
            132u16 => Ok(Self::Sm132),
            133u16 => Ok(Self::Sm133),
            134u16 => Ok(Self::Sm134),
            135u16 => Ok(Self::Sm135),
            136u16 => Ok(Self::Sm136),
            137u16 => Ok(Self::Sm137),
            138u16 => Ok(Self::Sm138),
            139u16 => Ok(Self::Sm139),
            140u16 => Ok(Self::Sm140),
            141u16 => Ok(Self::Sm141),
            142u16 => Ok(Self::Sm142),
            143u16 => Ok(Self::Sm143),
            144u16 => Ok(Self::Sm144),
            145u16 => Ok(Self::Sm145),
            146u16 => Ok(Self::Sm146),
            147u16 => Ok(Self::Sm147),
            148u16 => Ok(Self::Sm148),
            149u16 => Ok(Self::Sm149),
            150u16 => Ok(Self::Sm150),
            151u16 => Ok(Self::Sm151),
            152u16 => Ok(Self::Sm152),
            153u16 => Ok(Self::Sm153),
            154u16 => Ok(Self::Sm154),
            155u16 => Ok(Self::Sm155),
            156u16 => Ok(Self::Sm156),
            157u16 => Ok(Self::Sm157),
            158u16 => Ok(Self::Sm158),
            159u16 => Ok(Self::Sm159),
            160u16 => Ok(Self::Sm160),
            161u16 => Ok(Self::Sm161),
            162u16 => Ok(Self::Sm162),
            163u16 => Ok(Self::Sm163),
            164u16 => Ok(Self::Sm164),
            165u16 => Ok(Self::Sm165),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Sm, value)),
        }
    }
}
impl std::fmt::Display for SamariumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sm128 => write!(f, "Sm128"),
            Self::Sm129 => write!(f, "Sm129"),
            Self::Sm130 => write!(f, "Sm130"),
            Self::Sm131 => write!(f, "Sm131"),
            Self::Sm132 => write!(f, "Sm132"),
            Self::Sm133 => write!(f, "Sm133"),
            Self::Sm134 => write!(f, "Sm134"),
            Self::Sm135 => write!(f, "Sm135"),
            Self::Sm136 => write!(f, "Sm136"),
            Self::Sm137 => write!(f, "Sm137"),
            Self::Sm138 => write!(f, "Sm138"),
            Self::Sm139 => write!(f, "Sm139"),
            Self::Sm140 => write!(f, "Sm140"),
            Self::Sm141 => write!(f, "Sm141"),
            Self::Sm142 => write!(f, "Sm142"),
            Self::Sm143 => write!(f, "Sm143"),
            Self::Sm144 => write!(f, "Sm144"),
            Self::Sm145 => write!(f, "Sm145"),
            Self::Sm146 => write!(f, "Sm146"),
            Self::Sm147 => write!(f, "Sm147"),
            Self::Sm148 => write!(f, "Sm148"),
            Self::Sm149 => write!(f, "Sm149"),
            Self::Sm150 => write!(f, "Sm150"),
            Self::Sm151 => write!(f, "Sm151"),
            Self::Sm152 => write!(f, "Sm152"),
            Self::Sm153 => write!(f, "Sm153"),
            Self::Sm154 => write!(f, "Sm154"),
            Self::Sm155 => write!(f, "Sm155"),
            Self::Sm156 => write!(f, "Sm156"),
            Self::Sm157 => write!(f, "Sm157"),
            Self::Sm158 => write!(f, "Sm158"),
            Self::Sm159 => write!(f, "Sm159"),
            Self::Sm160 => write!(f, "Sm160"),
            Self::Sm161 => write!(f, "Sm161"),
            Self::Sm162 => write!(f, "Sm162"),
            Self::Sm163 => write!(f, "Sm163"),
            Self::Sm164 => write!(f, "Sm164"),
            Self::Sm165 => write!(f, "Sm165"),
        }
    }
}
