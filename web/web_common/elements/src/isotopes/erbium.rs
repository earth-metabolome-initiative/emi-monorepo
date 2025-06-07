//! Isotopes of the element Erbium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Erbium
pub enum ErbiumIsotope {
    /// Isotope Er142 of Erbium
    Er142,
    /// Isotope Er143 of Erbium
    Er143,
    /// Isotope Er144 of Erbium
    Er144,
    /// Isotope Er145 of Erbium
    Er145,
    /// Isotope Er146 of Erbium
    Er146,
    /// Isotope Er147 of Erbium
    Er147,
    /// Isotope Er148 of Erbium
    Er148,
    /// Isotope Er149 of Erbium
    Er149,
    /// Isotope Er150 of Erbium
    Er150,
    /// Isotope Er151 of Erbium
    Er151,
    /// Isotope Er152 of Erbium
    Er152,
    /// Isotope Er153 of Erbium
    Er153,
    /// Isotope Er154 of Erbium
    Er154,
    /// Isotope Er155 of Erbium
    Er155,
    /// Isotope Er156 of Erbium
    Er156,
    /// Isotope Er157 of Erbium
    Er157,
    /// Isotope Er158 of Erbium
    Er158,
    /// Isotope Er159 of Erbium
    Er159,
    /// Isotope Er160 of Erbium
    Er160,
    /// Isotope Er161 of Erbium
    Er161,
    /// Isotope Er162 of Erbium
    Er162,
    /// Isotope Er163 of Erbium
    Er163,
    /// Isotope Er164 of Erbium
    Er164,
    /// Isotope Er165 of Erbium
    Er165,
    /// Isotope Er166 of Erbium
    Er166,
    /// Isotope Er167 of Erbium
    Er167,
    /// Isotope Er168 of Erbium
    Er168,
    /// Isotope Er169 of Erbium
    Er169,
    /// Isotope Er170 of Erbium
    Er170,
    /// Isotope Er171 of Erbium
    Er171,
    /// Isotope Er172 of Erbium
    Er172,
    /// Isotope Er173 of Erbium
    Er173,
    /// Isotope Er174 of Erbium
    Er174,
    /// Isotope Er175 of Erbium
    Er175,
    /// Isotope Er176 of Erbium
    Er176,
    /// Isotope Er177 of Erbium
    Er177,
}
impl super::RelativeAtomicMass for ErbiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Er142 => 141.9701f64,
            Self::Er143 => 142.96662f64,
            Self::Er144 => 143.9607f64,
            Self::Er145 => 144.95805f64,
            Self::Er146 => 145.9524184f64,
            Self::Er147 => 146.949964f64,
            Self::Er148 => 147.944735f64,
            Self::Er149 => 148.942306f64,
            Self::Er150 => 149.937916f64,
            Self::Er151 => 150.937449f64,
            Self::Er152 => 151.935057f64,
            Self::Er153 => 152.93508f64,
            Self::Er154 => 153.9327908f64,
            Self::Er155 => 154.9332159f64,
            Self::Er156 => 155.931067f64,
            Self::Er157 => 156.931949f64,
            Self::Er158 => 157.929893f64,
            Self::Er159 => 158.9306918f64,
            Self::Er160 => 159.929077f64,
            Self::Er161 => 160.9300046f64,
            Self::Er162 => 161.9287884f64,
            Self::Er163 => 162.9300408f64,
            Self::Er164 => 163.9292088f64,
            Self::Er165 => 164.9307345f64,
            Self::Er166 => 165.9302995f64,
            Self::Er167 => 166.9320546f64,
            Self::Er168 => 167.9323767f64,
            Self::Er169 => 168.9345968f64,
            Self::Er170 => 169.9354702f64,
            Self::Er171 => 170.9380357f64,
            Self::Er172 => 171.9393619f64,
            Self::Er173 => 172.9424f64,
            Self::Er174 => 173.94423f64,
            Self::Er175 => 174.94777f64,
            Self::Er176 => 175.94994f64,
            Self::Er177 => 176.95399f64,
        }
    }
}
impl super::ElementVariant for ErbiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Er
    }
}
impl super::MassNumber for ErbiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Er142 => 142u16,
            Self::Er143 => 143u16,
            Self::Er144 => 144u16,
            Self::Er145 => 145u16,
            Self::Er146 => 146u16,
            Self::Er147 => 147u16,
            Self::Er148 => 148u16,
            Self::Er149 => 149u16,
            Self::Er150 => 150u16,
            Self::Er151 => 151u16,
            Self::Er152 => 152u16,
            Self::Er153 => 153u16,
            Self::Er154 => 154u16,
            Self::Er155 => 155u16,
            Self::Er156 => 156u16,
            Self::Er157 => 157u16,
            Self::Er158 => 158u16,
            Self::Er159 => 159u16,
            Self::Er160 => 160u16,
            Self::Er161 => 161u16,
            Self::Er162 => 162u16,
            Self::Er163 => 163u16,
            Self::Er164 => 164u16,
            Self::Er165 => 165u16,
            Self::Er166 => 166u16,
            Self::Er167 => 167u16,
            Self::Er168 => 168u16,
            Self::Er169 => 169u16,
            Self::Er170 => 170u16,
            Self::Er171 => 171u16,
            Self::Er172 => 172u16,
            Self::Er173 => 173u16,
            Self::Er174 => 174u16,
            Self::Er175 => 175u16,
            Self::Er176 => 176u16,
            Self::Er177 => 177u16,
        }
    }
}
impl super::IsotopicComposition for ErbiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Er162 => Some(0.00139f64),
            Self::Er164 => Some(0.01601f64),
            Self::Er166 => Some(0.33503f64),
            Self::Er167 => Some(0.22869f64),
            Self::Er168 => Some(0.26978f64),
            Self::Er170 => Some(0.1491f64),
            Self::Er142
            | Self::Er143
            | Self::Er144
            | Self::Er145
            | Self::Er146
            | Self::Er147
            | Self::Er148
            | Self::Er149
            | Self::Er150
            | Self::Er151
            | Self::Er152
            | Self::Er153
            | Self::Er154
            | Self::Er155
            | Self::Er156
            | Self::Er157
            | Self::Er158
            | Self::Er159
            | Self::Er160
            | Self::Er161
            | Self::Er163
            | Self::Er165
            | Self::Er169
            | Self::Er171
            | Self::Er172
            | Self::Er173
            | Self::Er174
            | Self::Er175
            | Self::Er176
            | Self::Er177 => None,
        }
    }
}
impl super::MostAbundantIsotope for ErbiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Er166
    }
}
impl From<ErbiumIsotope> for crate::Isotope {
    fn from(isotope: ErbiumIsotope) -> Self {
        crate::Isotope::Er(isotope)
    }
}
impl From<ErbiumIsotope> for crate::Element {
    fn from(_isotope: ErbiumIsotope) -> Self {
        crate::Element::Er
    }
}
impl TryFrom<u16> for ErbiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            142u16 => Ok(Self::Er142),
            143u16 => Ok(Self::Er143),
            144u16 => Ok(Self::Er144),
            145u16 => Ok(Self::Er145),
            146u16 => Ok(Self::Er146),
            147u16 => Ok(Self::Er147),
            148u16 => Ok(Self::Er148),
            149u16 => Ok(Self::Er149),
            150u16 => Ok(Self::Er150),
            151u16 => Ok(Self::Er151),
            152u16 => Ok(Self::Er152),
            153u16 => Ok(Self::Er153),
            154u16 => Ok(Self::Er154),
            155u16 => Ok(Self::Er155),
            156u16 => Ok(Self::Er156),
            157u16 => Ok(Self::Er157),
            158u16 => Ok(Self::Er158),
            159u16 => Ok(Self::Er159),
            160u16 => Ok(Self::Er160),
            161u16 => Ok(Self::Er161),
            162u16 => Ok(Self::Er162),
            163u16 => Ok(Self::Er163),
            164u16 => Ok(Self::Er164),
            165u16 => Ok(Self::Er165),
            166u16 => Ok(Self::Er166),
            167u16 => Ok(Self::Er167),
            168u16 => Ok(Self::Er168),
            169u16 => Ok(Self::Er169),
            170u16 => Ok(Self::Er170),
            171u16 => Ok(Self::Er171),
            172u16 => Ok(Self::Er172),
            173u16 => Ok(Self::Er173),
            174u16 => Ok(Self::Er174),
            175u16 => Ok(Self::Er175),
            176u16 => Ok(Self::Er176),
            177u16 => Ok(Self::Er177),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Er, value)),
        }
    }
}
impl std::fmt::Display for ErbiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Er142 => write!(f, "Er142"),
            Self::Er143 => write!(f, "Er143"),
            Self::Er144 => write!(f, "Er144"),
            Self::Er145 => write!(f, "Er145"),
            Self::Er146 => write!(f, "Er146"),
            Self::Er147 => write!(f, "Er147"),
            Self::Er148 => write!(f, "Er148"),
            Self::Er149 => write!(f, "Er149"),
            Self::Er150 => write!(f, "Er150"),
            Self::Er151 => write!(f, "Er151"),
            Self::Er152 => write!(f, "Er152"),
            Self::Er153 => write!(f, "Er153"),
            Self::Er154 => write!(f, "Er154"),
            Self::Er155 => write!(f, "Er155"),
            Self::Er156 => write!(f, "Er156"),
            Self::Er157 => write!(f, "Er157"),
            Self::Er158 => write!(f, "Er158"),
            Self::Er159 => write!(f, "Er159"),
            Self::Er160 => write!(f, "Er160"),
            Self::Er161 => write!(f, "Er161"),
            Self::Er162 => write!(f, "Er162"),
            Self::Er163 => write!(f, "Er163"),
            Self::Er164 => write!(f, "Er164"),
            Self::Er165 => write!(f, "Er165"),
            Self::Er166 => write!(f, "Er166"),
            Self::Er167 => write!(f, "Er167"),
            Self::Er168 => write!(f, "Er168"),
            Self::Er169 => write!(f, "Er169"),
            Self::Er170 => write!(f, "Er170"),
            Self::Er171 => write!(f, "Er171"),
            Self::Er172 => write!(f, "Er172"),
            Self::Er173 => write!(f, "Er173"),
            Self::Er174 => write!(f, "Er174"),
            Self::Er175 => write!(f, "Er175"),
            Self::Er176 => write!(f, "Er176"),
            Self::Er177 => write!(f, "Er177"),
        }
    }
}
