//! Isotopes of the element Thulium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Thulium
pub enum ThuliumIsotope {
    /// Isotope Tm144 of Thulium
    Tm144,
    /// Isotope Tm145 of Thulium
    Tm145,
    /// Isotope Tm146 of Thulium
    Tm146,
    /// Isotope Tm147 of Thulium
    Tm147,
    /// Isotope Tm148 of Thulium
    Tm148,
    /// Isotope Tm149 of Thulium
    Tm149,
    /// Isotope Tm150 of Thulium
    Tm150,
    /// Isotope Tm151 of Thulium
    Tm151,
    /// Isotope Tm152 of Thulium
    Tm152,
    /// Isotope Tm153 of Thulium
    Tm153,
    /// Isotope Tm154 of Thulium
    Tm154,
    /// Isotope Tm155 of Thulium
    Tm155,
    /// Isotope Tm156 of Thulium
    Tm156,
    /// Isotope Tm157 of Thulium
    Tm157,
    /// Isotope Tm158 of Thulium
    Tm158,
    /// Isotope Tm159 of Thulium
    Tm159,
    /// Isotope Tm160 of Thulium
    Tm160,
    /// Isotope Tm161 of Thulium
    Tm161,
    /// Isotope Tm162 of Thulium
    Tm162,
    /// Isotope Tm163 of Thulium
    Tm163,
    /// Isotope Tm164 of Thulium
    Tm164,
    /// Isotope Tm165 of Thulium
    Tm165,
    /// Isotope Tm166 of Thulium
    Tm166,
    /// Isotope Tm167 of Thulium
    Tm167,
    /// Isotope Tm168 of Thulium
    Tm168,
    /// Isotope Tm169 of Thulium
    Tm169,
    /// Isotope Tm170 of Thulium
    Tm170,
    /// Isotope Tm171 of Thulium
    Tm171,
    /// Isotope Tm172 of Thulium
    Tm172,
    /// Isotope Tm173 of Thulium
    Tm173,
    /// Isotope Tm174 of Thulium
    Tm174,
    /// Isotope Tm175 of Thulium
    Tm175,
    /// Isotope Tm176 of Thulium
    Tm176,
    /// Isotope Tm177 of Thulium
    Tm177,
    /// Isotope Tm178 of Thulium
    Tm178,
    /// Isotope Tm179 of Thulium
    Tm179,
}
impl super::RelativeAtomicMass for ThuliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Tm144 => 143.97628f64,
            Self::Tm145 => 144.97039f64,
            Self::Tm146 => 145.96684f64,
            Self::Tm147 => 146.9613799f64,
            Self::Tm148 => 147.958384f64,
            Self::Tm149 => 148.95289f64,
            Self::Tm150 => 149.95009f64,
            Self::Tm151 => 150.945488f64,
            Self::Tm152 => 151.944422f64,
            Self::Tm153 => 152.94204f64,
            Self::Tm154 => 153.94157f64,
            Self::Tm155 => 154.93921f64,
            Self::Tm156 => 155.938992f64,
            Self::Tm157 => 156.936944f64,
            Self::Tm158 => 157.93698f64,
            Self::Tm159 => 158.934975f64,
            Self::Tm160 => 159.935263f64,
            Self::Tm161 => 160.933549f64,
            Self::Tm162 => 161.934002f64,
            Self::Tm163 => 162.9326592f64,
            Self::Tm164 => 163.933544f64,
            Self::Tm165 => 164.9324431f64,
            Self::Tm166 => 165.933561f64,
            Self::Tm167 => 166.9328562f64,
            Self::Tm168 => 167.9341774f64,
            Self::Tm169 => 168.9342179f64,
            Self::Tm170 => 169.935806f64,
            Self::Tm171 => 170.9364339f64,
            Self::Tm172 => 171.9384055f64,
            Self::Tm173 => 172.9396084f64,
            Self::Tm174 => 173.942173f64,
            Self::Tm175 => 174.943841f64,
            Self::Tm176 => 175.947f64,
            Self::Tm177 => 176.94904f64,
            Self::Tm178 => 177.95264f64,
            Self::Tm179 => 178.95534f64,
        }
    }
}
impl super::ElementVariant for ThuliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Tm
    }
}
impl super::MassNumber for ThuliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Tm144 => 144u16,
            Self::Tm145 => 145u16,
            Self::Tm146 => 146u16,
            Self::Tm147 => 147u16,
            Self::Tm148 => 148u16,
            Self::Tm149 => 149u16,
            Self::Tm150 => 150u16,
            Self::Tm151 => 151u16,
            Self::Tm152 => 152u16,
            Self::Tm153 => 153u16,
            Self::Tm154 => 154u16,
            Self::Tm155 => 155u16,
            Self::Tm156 => 156u16,
            Self::Tm157 => 157u16,
            Self::Tm158 => 158u16,
            Self::Tm159 => 159u16,
            Self::Tm160 => 160u16,
            Self::Tm161 => 161u16,
            Self::Tm162 => 162u16,
            Self::Tm163 => 163u16,
            Self::Tm164 => 164u16,
            Self::Tm165 => 165u16,
            Self::Tm166 => 166u16,
            Self::Tm167 => 167u16,
            Self::Tm168 => 168u16,
            Self::Tm169 => 169u16,
            Self::Tm170 => 170u16,
            Self::Tm171 => 171u16,
            Self::Tm172 => 172u16,
            Self::Tm173 => 173u16,
            Self::Tm174 => 174u16,
            Self::Tm175 => 175u16,
            Self::Tm176 => 176u16,
            Self::Tm177 => 177u16,
            Self::Tm178 => 178u16,
            Self::Tm179 => 179u16,
        }
    }
}
impl super::IsotopicComposition for ThuliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Tm169 => Some(1f64),
            Self::Tm144
            | Self::Tm145
            | Self::Tm146
            | Self::Tm147
            | Self::Tm148
            | Self::Tm149
            | Self::Tm150
            | Self::Tm151
            | Self::Tm152
            | Self::Tm153
            | Self::Tm154
            | Self::Tm155
            | Self::Tm156
            | Self::Tm157
            | Self::Tm158
            | Self::Tm159
            | Self::Tm160
            | Self::Tm161
            | Self::Tm162
            | Self::Tm163
            | Self::Tm164
            | Self::Tm165
            | Self::Tm166
            | Self::Tm167
            | Self::Tm168
            | Self::Tm170
            | Self::Tm171
            | Self::Tm172
            | Self::Tm173
            | Self::Tm174
            | Self::Tm175
            | Self::Tm176
            | Self::Tm177
            | Self::Tm178
            | Self::Tm179 => None,
        }
    }
}
impl super::MostAbundantIsotope for ThuliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Tm169
    }
}
impl From<ThuliumIsotope> for crate::Isotope {
    fn from(isotope: ThuliumIsotope) -> Self {
        crate::Isotope::Tm(isotope)
    }
}
impl From<ThuliumIsotope> for crate::Element {
    fn from(_isotope: ThuliumIsotope) -> Self {
        crate::Element::Tm
    }
}
impl TryFrom<u16> for ThuliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            144u16 => Ok(Self::Tm144),
            145u16 => Ok(Self::Tm145),
            146u16 => Ok(Self::Tm146),
            147u16 => Ok(Self::Tm147),
            148u16 => Ok(Self::Tm148),
            149u16 => Ok(Self::Tm149),
            150u16 => Ok(Self::Tm150),
            151u16 => Ok(Self::Tm151),
            152u16 => Ok(Self::Tm152),
            153u16 => Ok(Self::Tm153),
            154u16 => Ok(Self::Tm154),
            155u16 => Ok(Self::Tm155),
            156u16 => Ok(Self::Tm156),
            157u16 => Ok(Self::Tm157),
            158u16 => Ok(Self::Tm158),
            159u16 => Ok(Self::Tm159),
            160u16 => Ok(Self::Tm160),
            161u16 => Ok(Self::Tm161),
            162u16 => Ok(Self::Tm162),
            163u16 => Ok(Self::Tm163),
            164u16 => Ok(Self::Tm164),
            165u16 => Ok(Self::Tm165),
            166u16 => Ok(Self::Tm166),
            167u16 => Ok(Self::Tm167),
            168u16 => Ok(Self::Tm168),
            169u16 => Ok(Self::Tm169),
            170u16 => Ok(Self::Tm170),
            171u16 => Ok(Self::Tm171),
            172u16 => Ok(Self::Tm172),
            173u16 => Ok(Self::Tm173),
            174u16 => Ok(Self::Tm174),
            175u16 => Ok(Self::Tm175),
            176u16 => Ok(Self::Tm176),
            177u16 => Ok(Self::Tm177),
            178u16 => Ok(Self::Tm178),
            179u16 => Ok(Self::Tm179),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Tm, value)),
        }
    }
}
impl std::fmt::Display for ThuliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tm144 => write!(f, "Tm144"),
            Self::Tm145 => write!(f, "Tm145"),
            Self::Tm146 => write!(f, "Tm146"),
            Self::Tm147 => write!(f, "Tm147"),
            Self::Tm148 => write!(f, "Tm148"),
            Self::Tm149 => write!(f, "Tm149"),
            Self::Tm150 => write!(f, "Tm150"),
            Self::Tm151 => write!(f, "Tm151"),
            Self::Tm152 => write!(f, "Tm152"),
            Self::Tm153 => write!(f, "Tm153"),
            Self::Tm154 => write!(f, "Tm154"),
            Self::Tm155 => write!(f, "Tm155"),
            Self::Tm156 => write!(f, "Tm156"),
            Self::Tm157 => write!(f, "Tm157"),
            Self::Tm158 => write!(f, "Tm158"),
            Self::Tm159 => write!(f, "Tm159"),
            Self::Tm160 => write!(f, "Tm160"),
            Self::Tm161 => write!(f, "Tm161"),
            Self::Tm162 => write!(f, "Tm162"),
            Self::Tm163 => write!(f, "Tm163"),
            Self::Tm164 => write!(f, "Tm164"),
            Self::Tm165 => write!(f, "Tm165"),
            Self::Tm166 => write!(f, "Tm166"),
            Self::Tm167 => write!(f, "Tm167"),
            Self::Tm168 => write!(f, "Tm168"),
            Self::Tm169 => write!(f, "Tm169"),
            Self::Tm170 => write!(f, "Tm170"),
            Self::Tm171 => write!(f, "Tm171"),
            Self::Tm172 => write!(f, "Tm172"),
            Self::Tm173 => write!(f, "Tm173"),
            Self::Tm174 => write!(f, "Tm174"),
            Self::Tm175 => write!(f, "Tm175"),
            Self::Tm176 => write!(f, "Tm176"),
            Self::Tm177 => write!(f, "Tm177"),
            Self::Tm178 => write!(f, "Tm178"),
            Self::Tm179 => write!(f, "Tm179"),
        }
    }
}
