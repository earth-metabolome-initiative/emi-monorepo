//! Isotopes of the element Holmium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Holmium
pub enum HolmiumIsotope {
    /// Isotope Ho140 of Holmium
    Ho140,
    /// Isotope Ho141 of Holmium
    Ho141,
    /// Isotope Ho142 of Holmium
    Ho142,
    /// Isotope Ho143 of Holmium
    Ho143,
    /// Isotope Ho144 of Holmium
    Ho144,
    /// Isotope Ho145 of Holmium
    Ho145,
    /// Isotope Ho146 of Holmium
    Ho146,
    /// Isotope Ho147 of Holmium
    Ho147,
    /// Isotope Ho148 of Holmium
    Ho148,
    /// Isotope Ho149 of Holmium
    Ho149,
    /// Isotope Ho150 of Holmium
    Ho150,
    /// Isotope Ho151 of Holmium
    Ho151,
    /// Isotope Ho152 of Holmium
    Ho152,
    /// Isotope Ho153 of Holmium
    Ho153,
    /// Isotope Ho154 of Holmium
    Ho154,
    /// Isotope Ho155 of Holmium
    Ho155,
    /// Isotope Ho156 of Holmium
    Ho156,
    /// Isotope Ho157 of Holmium
    Ho157,
    /// Isotope Ho158 of Holmium
    Ho158,
    /// Isotope Ho159 of Holmium
    Ho159,
    /// Isotope Ho160 of Holmium
    Ho160,
    /// Isotope Ho161 of Holmium
    Ho161,
    /// Isotope Ho162 of Holmium
    Ho162,
    /// Isotope Ho163 of Holmium
    Ho163,
    /// Isotope Ho164 of Holmium
    Ho164,
    /// Isotope Ho165 of Holmium
    Ho165,
    /// Isotope Ho166 of Holmium
    Ho166,
    /// Isotope Ho167 of Holmium
    Ho167,
    /// Isotope Ho168 of Holmium
    Ho168,
    /// Isotope Ho169 of Holmium
    Ho169,
    /// Isotope Ho170 of Holmium
    Ho170,
    /// Isotope Ho171 of Holmium
    Ho171,
    /// Isotope Ho172 of Holmium
    Ho172,
    /// Isotope Ho173 of Holmium
    Ho173,
    /// Isotope Ho174 of Holmium
    Ho174,
    /// Isotope Ho175 of Holmium
    Ho175,
}
impl super::RelativeAtomicMass for HolmiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ho140 => 139.96859f64,
            Self::Ho141 => 140.96311f64,
            Self::Ho142 => 141.96001f64,
            Self::Ho143 => 142.95486f64,
            Self::Ho144 => 143.9521097f64,
            Self::Ho145 => 144.9472674f64,
            Self::Ho146 => 145.9449935f64,
            Self::Ho147 => 146.9401423f64,
            Self::Ho148 => 147.937744f64,
            Self::Ho149 => 148.933803f64,
            Self::Ho150 => 149.933498f64,
            Self::Ho151 => 150.9316983f64,
            Self::Ho152 => 151.931724f64,
            Self::Ho153 => 152.9302064f64,
            Self::Ho154 => 153.9306068f64,
            Self::Ho155 => 154.929104f64,
            Self::Ho156 => 155.929706f64,
            Self::Ho157 => 156.928254f64,
            Self::Ho158 => 157.928946f64,
            Self::Ho159 => 158.9277197f64,
            Self::Ho160 => 159.928737f64,
            Self::Ho161 => 160.9278615f64,
            Self::Ho162 => 161.9291023f64,
            Self::Ho163 => 162.928741f64,
            Self::Ho164 => 163.9302403f64,
            Self::Ho165 => 164.9303288f64,
            Self::Ho166 => 165.9322909f64,
            Self::Ho167 => 166.9331385f64,
            Self::Ho168 => 167.935522f64,
            Self::Ho169 => 168.936878f64,
            Self::Ho170 => 169.939625f64,
            Self::Ho171 => 170.94147f64,
            Self::Ho172 => 171.94473f64,
            Self::Ho173 => 172.94702f64,
            Self::Ho174 => 173.95095f64,
            Self::Ho175 => 174.95362f64,
        }
    }
}
impl super::ElementVariant for HolmiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ho
    }
}
impl super::MassNumber for HolmiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ho140 => 140u16,
            Self::Ho141 => 141u16,
            Self::Ho142 => 142u16,
            Self::Ho143 => 143u16,
            Self::Ho144 => 144u16,
            Self::Ho145 => 145u16,
            Self::Ho146 => 146u16,
            Self::Ho147 => 147u16,
            Self::Ho148 => 148u16,
            Self::Ho149 => 149u16,
            Self::Ho150 => 150u16,
            Self::Ho151 => 151u16,
            Self::Ho152 => 152u16,
            Self::Ho153 => 153u16,
            Self::Ho154 => 154u16,
            Self::Ho155 => 155u16,
            Self::Ho156 => 156u16,
            Self::Ho157 => 157u16,
            Self::Ho158 => 158u16,
            Self::Ho159 => 159u16,
            Self::Ho160 => 160u16,
            Self::Ho161 => 161u16,
            Self::Ho162 => 162u16,
            Self::Ho163 => 163u16,
            Self::Ho164 => 164u16,
            Self::Ho165 => 165u16,
            Self::Ho166 => 166u16,
            Self::Ho167 => 167u16,
            Self::Ho168 => 168u16,
            Self::Ho169 => 169u16,
            Self::Ho170 => 170u16,
            Self::Ho171 => 171u16,
            Self::Ho172 => 172u16,
            Self::Ho173 => 173u16,
            Self::Ho174 => 174u16,
            Self::Ho175 => 175u16,
        }
    }
}
impl super::IsotopicComposition for HolmiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ho165 => Some(1f64),
            Self::Ho140
            | Self::Ho141
            | Self::Ho142
            | Self::Ho143
            | Self::Ho144
            | Self::Ho145
            | Self::Ho146
            | Self::Ho147
            | Self::Ho148
            | Self::Ho149
            | Self::Ho150
            | Self::Ho151
            | Self::Ho152
            | Self::Ho153
            | Self::Ho154
            | Self::Ho155
            | Self::Ho156
            | Self::Ho157
            | Self::Ho158
            | Self::Ho159
            | Self::Ho160
            | Self::Ho161
            | Self::Ho162
            | Self::Ho163
            | Self::Ho164
            | Self::Ho166
            | Self::Ho167
            | Self::Ho168
            | Self::Ho169
            | Self::Ho170
            | Self::Ho171
            | Self::Ho172
            | Self::Ho173
            | Self::Ho174
            | Self::Ho175 => None,
        }
    }
}
impl super::MostAbundantIsotope for HolmiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ho165
    }
}
impl From<HolmiumIsotope> for crate::Isotope {
    fn from(isotope: HolmiumIsotope) -> Self {
        crate::Isotope::Ho(isotope)
    }
}
impl From<HolmiumIsotope> for crate::Element {
    fn from(_isotope: HolmiumIsotope) -> Self {
        crate::Element::Ho
    }
}
impl TryFrom<u16> for HolmiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            140u16 => Ok(Self::Ho140),
            141u16 => Ok(Self::Ho141),
            142u16 => Ok(Self::Ho142),
            143u16 => Ok(Self::Ho143),
            144u16 => Ok(Self::Ho144),
            145u16 => Ok(Self::Ho145),
            146u16 => Ok(Self::Ho146),
            147u16 => Ok(Self::Ho147),
            148u16 => Ok(Self::Ho148),
            149u16 => Ok(Self::Ho149),
            150u16 => Ok(Self::Ho150),
            151u16 => Ok(Self::Ho151),
            152u16 => Ok(Self::Ho152),
            153u16 => Ok(Self::Ho153),
            154u16 => Ok(Self::Ho154),
            155u16 => Ok(Self::Ho155),
            156u16 => Ok(Self::Ho156),
            157u16 => Ok(Self::Ho157),
            158u16 => Ok(Self::Ho158),
            159u16 => Ok(Self::Ho159),
            160u16 => Ok(Self::Ho160),
            161u16 => Ok(Self::Ho161),
            162u16 => Ok(Self::Ho162),
            163u16 => Ok(Self::Ho163),
            164u16 => Ok(Self::Ho164),
            165u16 => Ok(Self::Ho165),
            166u16 => Ok(Self::Ho166),
            167u16 => Ok(Self::Ho167),
            168u16 => Ok(Self::Ho168),
            169u16 => Ok(Self::Ho169),
            170u16 => Ok(Self::Ho170),
            171u16 => Ok(Self::Ho171),
            172u16 => Ok(Self::Ho172),
            173u16 => Ok(Self::Ho173),
            174u16 => Ok(Self::Ho174),
            175u16 => Ok(Self::Ho175),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ho, value)),
        }
    }
}
impl std::fmt::Display for HolmiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ho140 => write!(f, "Ho140"),
            Self::Ho141 => write!(f, "Ho141"),
            Self::Ho142 => write!(f, "Ho142"),
            Self::Ho143 => write!(f, "Ho143"),
            Self::Ho144 => write!(f, "Ho144"),
            Self::Ho145 => write!(f, "Ho145"),
            Self::Ho146 => write!(f, "Ho146"),
            Self::Ho147 => write!(f, "Ho147"),
            Self::Ho148 => write!(f, "Ho148"),
            Self::Ho149 => write!(f, "Ho149"),
            Self::Ho150 => write!(f, "Ho150"),
            Self::Ho151 => write!(f, "Ho151"),
            Self::Ho152 => write!(f, "Ho152"),
            Self::Ho153 => write!(f, "Ho153"),
            Self::Ho154 => write!(f, "Ho154"),
            Self::Ho155 => write!(f, "Ho155"),
            Self::Ho156 => write!(f, "Ho156"),
            Self::Ho157 => write!(f, "Ho157"),
            Self::Ho158 => write!(f, "Ho158"),
            Self::Ho159 => write!(f, "Ho159"),
            Self::Ho160 => write!(f, "Ho160"),
            Self::Ho161 => write!(f, "Ho161"),
            Self::Ho162 => write!(f, "Ho162"),
            Self::Ho163 => write!(f, "Ho163"),
            Self::Ho164 => write!(f, "Ho164"),
            Self::Ho165 => write!(f, "Ho165"),
            Self::Ho166 => write!(f, "Ho166"),
            Self::Ho167 => write!(f, "Ho167"),
            Self::Ho168 => write!(f, "Ho168"),
            Self::Ho169 => write!(f, "Ho169"),
            Self::Ho170 => write!(f, "Ho170"),
            Self::Ho171 => write!(f, "Ho171"),
            Self::Ho172 => write!(f, "Ho172"),
            Self::Ho173 => write!(f, "Ho173"),
            Self::Ho174 => write!(f, "Ho174"),
            Self::Ho175 => write!(f, "Ho175"),
        }
    }
}
