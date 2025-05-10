//! Isotopes of the element Lutetium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Lutetium
pub enum LutetiumIsotope {
    /// Isotope Lu150 of Lutetium
    Lu150,
    /// Isotope Lu151 of Lutetium
    Lu151,
    /// Isotope Lu152 of Lutetium
    Lu152,
    /// Isotope Lu153 of Lutetium
    Lu153,
    /// Isotope Lu154 of Lutetium
    Lu154,
    /// Isotope Lu155 of Lutetium
    Lu155,
    /// Isotope Lu156 of Lutetium
    Lu156,
    /// Isotope Lu157 of Lutetium
    Lu157,
    /// Isotope Lu158 of Lutetium
    Lu158,
    /// Isotope Lu159 of Lutetium
    Lu159,
    /// Isotope Lu160 of Lutetium
    Lu160,
    /// Isotope Lu161 of Lutetium
    Lu161,
    /// Isotope Lu162 of Lutetium
    Lu162,
    /// Isotope Lu163 of Lutetium
    Lu163,
    /// Isotope Lu164 of Lutetium
    Lu164,
    /// Isotope Lu165 of Lutetium
    Lu165,
    /// Isotope Lu166 of Lutetium
    Lu166,
    /// Isotope Lu167 of Lutetium
    Lu167,
    /// Isotope Lu168 of Lutetium
    Lu168,
    /// Isotope Lu169 of Lutetium
    Lu169,
    /// Isotope Lu170 of Lutetium
    Lu170,
    /// Isotope Lu171 of Lutetium
    Lu171,
    /// Isotope Lu172 of Lutetium
    Lu172,
    /// Isotope Lu173 of Lutetium
    Lu173,
    /// Isotope Lu174 of Lutetium
    Lu174,
    /// Isotope Lu175 of Lutetium
    Lu175,
    /// Isotope Lu176 of Lutetium
    Lu176,
    /// Isotope Lu177 of Lutetium
    Lu177,
    /// Isotope Lu178 of Lutetium
    Lu178,
    /// Isotope Lu179 of Lutetium
    Lu179,
    /// Isotope Lu180 of Lutetium
    Lu180,
    /// Isotope Lu181 of Lutetium
    Lu181,
    /// Isotope Lu182 of Lutetium
    Lu182,
    /// Isotope Lu183 of Lutetium
    Lu183,
    /// Isotope Lu184 of Lutetium
    Lu184,
    /// Isotope Lu185 of Lutetium
    Lu185,
}
impl super::RelativeAtomicMass for LutetiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Lu150 => 149.97355f64,
            Self::Lu151 => 150.96768f64,
            Self::Lu152 => 151.96412f64,
            Self::Lu153 => 152.95875f64,
            Self::Lu154 => 153.95736f64,
            Self::Lu155 => 154.954321f64,
            Self::Lu156 => 155.953033f64,
            Self::Lu157 => 156.950127f64,
            Self::Lu158 => 157.949316f64,
            Self::Lu159 => 158.946636f64,
            Self::Lu160 => 159.946033f64,
            Self::Lu161 => 160.943572f64,
            Self::Lu162 => 161.943283f64,
            Self::Lu163 => 162.941179f64,
            Self::Lu164 => 163.941339f64,
            Self::Lu165 => 164.939407f64,
            Self::Lu166 => 165.939859f64,
            Self::Lu167 => 166.93827f64,
            Self::Lu168 => 167.938736f64,
            Self::Lu169 => 168.9376441f64,
            Self::Lu170 => 169.938478f64,
            Self::Lu171 => 170.937917f64,
            Self::Lu172 => 171.9390891f64,
            Self::Lu173 => 172.938934f64,
            Self::Lu174 => 173.9403409f64,
            Self::Lu175 => 174.9407752f64,
            Self::Lu176 => 175.9426897f64,
            Self::Lu177 => 176.9437615f64,
            Self::Lu178 => 177.945958f64,
            Self::Lu179 => 178.9473309f64,
            Self::Lu180 => 179.949888f64,
            Self::Lu181 => 180.95191f64,
            Self::Lu182 => 181.95504f64,
            Self::Lu183 => 182.957363f64,
            Self::Lu184 => 183.96091f64,
            Self::Lu185 => 184.96362f64,
        }
    }
}
impl super::ElementVariant for LutetiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Lu
    }
}
impl super::MassNumber for LutetiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Lu150 => 150u16,
            Self::Lu151 => 151u16,
            Self::Lu152 => 152u16,
            Self::Lu153 => 153u16,
            Self::Lu154 => 154u16,
            Self::Lu155 => 155u16,
            Self::Lu156 => 156u16,
            Self::Lu157 => 157u16,
            Self::Lu158 => 158u16,
            Self::Lu159 => 159u16,
            Self::Lu160 => 160u16,
            Self::Lu161 => 161u16,
            Self::Lu162 => 162u16,
            Self::Lu163 => 163u16,
            Self::Lu164 => 164u16,
            Self::Lu165 => 165u16,
            Self::Lu166 => 166u16,
            Self::Lu167 => 167u16,
            Self::Lu168 => 168u16,
            Self::Lu169 => 169u16,
            Self::Lu170 => 170u16,
            Self::Lu171 => 171u16,
            Self::Lu172 => 172u16,
            Self::Lu173 => 173u16,
            Self::Lu174 => 174u16,
            Self::Lu175 => 175u16,
            Self::Lu176 => 176u16,
            Self::Lu177 => 177u16,
            Self::Lu178 => 178u16,
            Self::Lu179 => 179u16,
            Self::Lu180 => 180u16,
            Self::Lu181 => 181u16,
            Self::Lu182 => 182u16,
            Self::Lu183 => 183u16,
            Self::Lu184 => 184u16,
            Self::Lu185 => 185u16,
        }
    }
}
impl super::IsotopicComposition for LutetiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Lu175 => Some(0.97401f64),
            Self::Lu176 => Some(0.02599f64),
            Self::Lu150
            | Self::Lu151
            | Self::Lu152
            | Self::Lu153
            | Self::Lu154
            | Self::Lu155
            | Self::Lu156
            | Self::Lu157
            | Self::Lu158
            | Self::Lu159
            | Self::Lu160
            | Self::Lu161
            | Self::Lu162
            | Self::Lu163
            | Self::Lu164
            | Self::Lu165
            | Self::Lu166
            | Self::Lu167
            | Self::Lu168
            | Self::Lu169
            | Self::Lu170
            | Self::Lu171
            | Self::Lu172
            | Self::Lu173
            | Self::Lu174
            | Self::Lu177
            | Self::Lu178
            | Self::Lu179
            | Self::Lu180
            | Self::Lu181
            | Self::Lu182
            | Self::Lu183
            | Self::Lu184
            | Self::Lu185 => None,
        }
    }
}
impl super::MostAbundantIsotope for LutetiumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Lu175
    }
}
impl TryFrom<u16> for LutetiumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            150u16 => Ok(Self::Lu150),
            151u16 => Ok(Self::Lu151),
            152u16 => Ok(Self::Lu152),
            153u16 => Ok(Self::Lu153),
            154u16 => Ok(Self::Lu154),
            155u16 => Ok(Self::Lu155),
            156u16 => Ok(Self::Lu156),
            157u16 => Ok(Self::Lu157),
            158u16 => Ok(Self::Lu158),
            159u16 => Ok(Self::Lu159),
            160u16 => Ok(Self::Lu160),
            161u16 => Ok(Self::Lu161),
            162u16 => Ok(Self::Lu162),
            163u16 => Ok(Self::Lu163),
            164u16 => Ok(Self::Lu164),
            165u16 => Ok(Self::Lu165),
            166u16 => Ok(Self::Lu166),
            167u16 => Ok(Self::Lu167),
            168u16 => Ok(Self::Lu168),
            169u16 => Ok(Self::Lu169),
            170u16 => Ok(Self::Lu170),
            171u16 => Ok(Self::Lu171),
            172u16 => Ok(Self::Lu172),
            173u16 => Ok(Self::Lu173),
            174u16 => Ok(Self::Lu174),
            175u16 => Ok(Self::Lu175),
            176u16 => Ok(Self::Lu176),
            177u16 => Ok(Self::Lu177),
            178u16 => Ok(Self::Lu178),
            179u16 => Ok(Self::Lu179),
            180u16 => Ok(Self::Lu180),
            181u16 => Ok(Self::Lu181),
            182u16 => Ok(Self::Lu182),
            183u16 => Ok(Self::Lu183),
            184u16 => Ok(Self::Lu184),
            185u16 => Ok(Self::Lu185),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Lu, value)),
        }
    }
}
impl std::fmt::Display for LutetiumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lu150 => write!(f, "Lu150"),
            Self::Lu151 => write!(f, "Lu151"),
            Self::Lu152 => write!(f, "Lu152"),
            Self::Lu153 => write!(f, "Lu153"),
            Self::Lu154 => write!(f, "Lu154"),
            Self::Lu155 => write!(f, "Lu155"),
            Self::Lu156 => write!(f, "Lu156"),
            Self::Lu157 => write!(f, "Lu157"),
            Self::Lu158 => write!(f, "Lu158"),
            Self::Lu159 => write!(f, "Lu159"),
            Self::Lu160 => write!(f, "Lu160"),
            Self::Lu161 => write!(f, "Lu161"),
            Self::Lu162 => write!(f, "Lu162"),
            Self::Lu163 => write!(f, "Lu163"),
            Self::Lu164 => write!(f, "Lu164"),
            Self::Lu165 => write!(f, "Lu165"),
            Self::Lu166 => write!(f, "Lu166"),
            Self::Lu167 => write!(f, "Lu167"),
            Self::Lu168 => write!(f, "Lu168"),
            Self::Lu169 => write!(f, "Lu169"),
            Self::Lu170 => write!(f, "Lu170"),
            Self::Lu171 => write!(f, "Lu171"),
            Self::Lu172 => write!(f, "Lu172"),
            Self::Lu173 => write!(f, "Lu173"),
            Self::Lu174 => write!(f, "Lu174"),
            Self::Lu175 => write!(f, "Lu175"),
            Self::Lu176 => write!(f, "Lu176"),
            Self::Lu177 => write!(f, "Lu177"),
            Self::Lu178 => write!(f, "Lu178"),
            Self::Lu179 => write!(f, "Lu179"),
            Self::Lu180 => write!(f, "Lu180"),
            Self::Lu181 => write!(f, "Lu181"),
            Self::Lu182 => write!(f, "Lu182"),
            Self::Lu183 => write!(f, "Lu183"),
            Self::Lu184 => write!(f, "Lu184"),
            Self::Lu185 => write!(f, "Lu185"),
        }
    }
}
