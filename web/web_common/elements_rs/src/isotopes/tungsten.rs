//! Isotopes of the element Tungsten
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Tungsten
pub enum TungstenIsotope {
    /// Isotope W157 of Tungsten
    W157,
    /// Isotope W158 of Tungsten
    W158,
    /// Isotope W159 of Tungsten
    W159,
    /// Isotope W160 of Tungsten
    W160,
    /// Isotope W161 of Tungsten
    W161,
    /// Isotope W162 of Tungsten
    W162,
    /// Isotope W163 of Tungsten
    W163,
    /// Isotope W164 of Tungsten
    W164,
    /// Isotope W165 of Tungsten
    W165,
    /// Isotope W166 of Tungsten
    W166,
    /// Isotope W167 of Tungsten
    W167,
    /// Isotope W168 of Tungsten
    W168,
    /// Isotope W169 of Tungsten
    W169,
    /// Isotope W170 of Tungsten
    W170,
    /// Isotope W171 of Tungsten
    W171,
    /// Isotope W172 of Tungsten
    W172,
    /// Isotope W173 of Tungsten
    W173,
    /// Isotope W174 of Tungsten
    W174,
    /// Isotope W175 of Tungsten
    W175,
    /// Isotope W176 of Tungsten
    W176,
    /// Isotope W177 of Tungsten
    W177,
    /// Isotope W178 of Tungsten
    W178,
    /// Isotope W179 of Tungsten
    W179,
    /// Isotope W180 of Tungsten
    W180,
    /// Isotope W181 of Tungsten
    W181,
    /// Isotope W182 of Tungsten
    W182,
    /// Isotope W183 of Tungsten
    W183,
    /// Isotope W184 of Tungsten
    W184,
    /// Isotope W185 of Tungsten
    W185,
    /// Isotope W186 of Tungsten
    W186,
    /// Isotope W187 of Tungsten
    W187,
    /// Isotope W188 of Tungsten
    W188,
    /// Isotope W189 of Tungsten
    W189,
    /// Isotope W190 of Tungsten
    W190,
    /// Isotope W191 of Tungsten
    W191,
    /// Isotope W192 of Tungsten
    W192,
    /// Isotope W193 of Tungsten
    W193,
    /// Isotope W194 of Tungsten
    W194,
}
impl super::RelativeAtomicMass for TungstenIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::W157 => 156.97884f64,
            Self::W158 => 157.97456f64,
            Self::W159 => 158.97264f64,
            Self::W160 => 159.96846f64,
            Self::W161 => 160.9672f64,
            Self::W162 => 161.963499f64,
            Self::W163 => 162.962524f64,
            Self::W164 => 163.958961f64,
            Self::W165 => 164.958281f64,
            Self::W166 => 165.955031f64,
            Self::W167 => 166.954805f64,
            Self::W168 => 167.951806f64,
            Self::W169 => 168.951779f64,
            Self::W170 => 169.949232f64,
            Self::W171 => 170.949451f64,
            Self::W172 => 171.947292f64,
            Self::W173 => 172.947689f64,
            Self::W174 => 173.946079f64,
            Self::W175 => 174.946717f64,
            Self::W176 => 175.945634f64,
            Self::W177 => 176.946643f64,
            Self::W178 => 177.945883f64,
            Self::W179 => 178.947077f64,
            Self::W180 => 179.9467108f64,
            Self::W181 => 180.9481978f64,
            Self::W182 => 181.94820394f64,
            Self::W183 => 182.95022275f64,
            Self::W184 => 183.95093092f64,
            Self::W185 => 184.95341897f64,
            Self::W186 => 185.9543628f64,
            Self::W187 => 186.9571588f64,
            Self::W188 => 187.9584862f64,
            Self::W189 => 188.961763f64,
            Self::W190 => 189.963091f64,
            Self::W191 => 190.966531f64,
            Self::W192 => 191.96817f64,
            Self::W193 => 192.97178f64,
            Self::W194 => 193.97367f64,
        }
    }
}
impl super::ElementVariant for TungstenIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::W
    }
}
impl super::MassNumber for TungstenIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::W157 => 157u16,
            Self::W158 => 158u16,
            Self::W159 => 159u16,
            Self::W160 => 160u16,
            Self::W161 => 161u16,
            Self::W162 => 162u16,
            Self::W163 => 163u16,
            Self::W164 => 164u16,
            Self::W165 => 165u16,
            Self::W166 => 166u16,
            Self::W167 => 167u16,
            Self::W168 => 168u16,
            Self::W169 => 169u16,
            Self::W170 => 170u16,
            Self::W171 => 171u16,
            Self::W172 => 172u16,
            Self::W173 => 173u16,
            Self::W174 => 174u16,
            Self::W175 => 175u16,
            Self::W176 => 176u16,
            Self::W177 => 177u16,
            Self::W178 => 178u16,
            Self::W179 => 179u16,
            Self::W180 => 180u16,
            Self::W181 => 181u16,
            Self::W182 => 182u16,
            Self::W183 => 183u16,
            Self::W184 => 184u16,
            Self::W185 => 185u16,
            Self::W186 => 186u16,
            Self::W187 => 187u16,
            Self::W188 => 188u16,
            Self::W189 => 189u16,
            Self::W190 => 190u16,
            Self::W191 => 191u16,
            Self::W192 => 192u16,
            Self::W193 => 193u16,
            Self::W194 => 194u16,
        }
    }
}
impl super::IsotopicComposition for TungstenIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::W180 => Some(0.0012f64),
            Self::W182 => Some(0.265f64),
            Self::W183 => Some(0.1431f64),
            Self::W184 => Some(0.3064f64),
            Self::W186 => Some(0.2843f64),
            Self::W157
            | Self::W158
            | Self::W159
            | Self::W160
            | Self::W161
            | Self::W162
            | Self::W163
            | Self::W164
            | Self::W165
            | Self::W166
            | Self::W167
            | Self::W168
            | Self::W169
            | Self::W170
            | Self::W171
            | Self::W172
            | Self::W173
            | Self::W174
            | Self::W175
            | Self::W176
            | Self::W177
            | Self::W178
            | Self::W179
            | Self::W181
            | Self::W185
            | Self::W187
            | Self::W188
            | Self::W189
            | Self::W190
            | Self::W191
            | Self::W192
            | Self::W193
            | Self::W194 => None,
        }
    }
}
impl super::MostAbundantIsotope for TungstenIsotope {
    fn most_abundant_isotope() -> Self {
        Self::W184
    }
}
impl From<TungstenIsotope> for crate::Isotope {
    fn from(isotope: TungstenIsotope) -> Self {
        crate::Isotope::W(isotope)
    }
}
impl From<TungstenIsotope> for crate::Element {
    fn from(_isotope: TungstenIsotope) -> Self {
        crate::Element::W
    }
}
impl TryFrom<u16> for TungstenIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            157u16 => Ok(Self::W157),
            158u16 => Ok(Self::W158),
            159u16 => Ok(Self::W159),
            160u16 => Ok(Self::W160),
            161u16 => Ok(Self::W161),
            162u16 => Ok(Self::W162),
            163u16 => Ok(Self::W163),
            164u16 => Ok(Self::W164),
            165u16 => Ok(Self::W165),
            166u16 => Ok(Self::W166),
            167u16 => Ok(Self::W167),
            168u16 => Ok(Self::W168),
            169u16 => Ok(Self::W169),
            170u16 => Ok(Self::W170),
            171u16 => Ok(Self::W171),
            172u16 => Ok(Self::W172),
            173u16 => Ok(Self::W173),
            174u16 => Ok(Self::W174),
            175u16 => Ok(Self::W175),
            176u16 => Ok(Self::W176),
            177u16 => Ok(Self::W177),
            178u16 => Ok(Self::W178),
            179u16 => Ok(Self::W179),
            180u16 => Ok(Self::W180),
            181u16 => Ok(Self::W181),
            182u16 => Ok(Self::W182),
            183u16 => Ok(Self::W183),
            184u16 => Ok(Self::W184),
            185u16 => Ok(Self::W185),
            186u16 => Ok(Self::W186),
            187u16 => Ok(Self::W187),
            188u16 => Ok(Self::W188),
            189u16 => Ok(Self::W189),
            190u16 => Ok(Self::W190),
            191u16 => Ok(Self::W191),
            192u16 => Ok(Self::W192),
            193u16 => Ok(Self::W193),
            194u16 => Ok(Self::W194),
            _ => Err(crate::errors::Error::Isotope(crate::Element::W, value)),
        }
    }
}
impl std::fmt::Display for TungstenIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W157 => write!(f, "W157"),
            Self::W158 => write!(f, "W158"),
            Self::W159 => write!(f, "W159"),
            Self::W160 => write!(f, "W160"),
            Self::W161 => write!(f, "W161"),
            Self::W162 => write!(f, "W162"),
            Self::W163 => write!(f, "W163"),
            Self::W164 => write!(f, "W164"),
            Self::W165 => write!(f, "W165"),
            Self::W166 => write!(f, "W166"),
            Self::W167 => write!(f, "W167"),
            Self::W168 => write!(f, "W168"),
            Self::W169 => write!(f, "W169"),
            Self::W170 => write!(f, "W170"),
            Self::W171 => write!(f, "W171"),
            Self::W172 => write!(f, "W172"),
            Self::W173 => write!(f, "W173"),
            Self::W174 => write!(f, "W174"),
            Self::W175 => write!(f, "W175"),
            Self::W176 => write!(f, "W176"),
            Self::W177 => write!(f, "W177"),
            Self::W178 => write!(f, "W178"),
            Self::W179 => write!(f, "W179"),
            Self::W180 => write!(f, "W180"),
            Self::W181 => write!(f, "W181"),
            Self::W182 => write!(f, "W182"),
            Self::W183 => write!(f, "W183"),
            Self::W184 => write!(f, "W184"),
            Self::W185 => write!(f, "W185"),
            Self::W186 => write!(f, "W186"),
            Self::W187 => write!(f, "W187"),
            Self::W188 => write!(f, "W188"),
            Self::W189 => write!(f, "W189"),
            Self::W190 => write!(f, "W190"),
            Self::W191 => write!(f, "W191"),
            Self::W192 => write!(f, "W192"),
            Self::W193 => write!(f, "W193"),
            Self::W194 => write!(f, "W194"),
        }
    }
}
