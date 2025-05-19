//! Isotopes of the element Tantalum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Tantalum
pub enum TantalumIsotope {
    /// Isotope Ta155 of Tantalum
    Ta155,
    /// Isotope Ta156 of Tantalum
    Ta156,
    /// Isotope Ta157 of Tantalum
    Ta157,
    /// Isotope Ta158 of Tantalum
    Ta158,
    /// Isotope Ta159 of Tantalum
    Ta159,
    /// Isotope Ta160 of Tantalum
    Ta160,
    /// Isotope Ta161 of Tantalum
    Ta161,
    /// Isotope Ta162 of Tantalum
    Ta162,
    /// Isotope Ta163 of Tantalum
    Ta163,
    /// Isotope Ta164 of Tantalum
    Ta164,
    /// Isotope Ta165 of Tantalum
    Ta165,
    /// Isotope Ta166 of Tantalum
    Ta166,
    /// Isotope Ta167 of Tantalum
    Ta167,
    /// Isotope Ta168 of Tantalum
    Ta168,
    /// Isotope Ta169 of Tantalum
    Ta169,
    /// Isotope Ta170 of Tantalum
    Ta170,
    /// Isotope Ta171 of Tantalum
    Ta171,
    /// Isotope Ta172 of Tantalum
    Ta172,
    /// Isotope Ta173 of Tantalum
    Ta173,
    /// Isotope Ta174 of Tantalum
    Ta174,
    /// Isotope Ta175 of Tantalum
    Ta175,
    /// Isotope Ta176 of Tantalum
    Ta176,
    /// Isotope Ta177 of Tantalum
    Ta177,
    /// Isotope Ta178 of Tantalum
    Ta178,
    /// Isotope Ta179 of Tantalum
    Ta179,
    /// Isotope Ta180 of Tantalum
    Ta180,
    /// Isotope Ta181 of Tantalum
    Ta181,
    /// Isotope Ta182 of Tantalum
    Ta182,
    /// Isotope Ta183 of Tantalum
    Ta183,
    /// Isotope Ta184 of Tantalum
    Ta184,
    /// Isotope Ta185 of Tantalum
    Ta185,
    /// Isotope Ta186 of Tantalum
    Ta186,
    /// Isotope Ta187 of Tantalum
    Ta187,
    /// Isotope Ta188 of Tantalum
    Ta188,
    /// Isotope Ta189 of Tantalum
    Ta189,
    /// Isotope Ta190 of Tantalum
    Ta190,
    /// Isotope Ta191 of Tantalum
    Ta191,
    /// Isotope Ta192 of Tantalum
    Ta192,
}
impl super::RelativeAtomicMass for TantalumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ta155 => 154.97424f64,
            Self::Ta156 => 155.97203f64,
            Self::Ta157 => 156.96818f64,
            Self::Ta158 => 157.96654f64,
            Self::Ta159 => 158.963023f64,
            Self::Ta160 => 159.961488f64,
            Self::Ta161 => 160.958452f64,
            Self::Ta162 => 161.957294f64,
            Self::Ta163 => 162.954337f64,
            Self::Ta164 => 163.953534f64,
            Self::Ta165 => 164.950781f64,
            Self::Ta166 => 165.950512f64,
            Self::Ta167 => 166.948093f64,
            Self::Ta168 => 167.948047f64,
            Self::Ta169 => 168.946011f64,
            Self::Ta170 => 169.946175f64,
            Self::Ta171 => 170.944476f64,
            Self::Ta172 => 171.944895f64,
            Self::Ta173 => 172.94375f64,
            Self::Ta174 => 173.944454f64,
            Self::Ta175 => 174.943737f64,
            Self::Ta176 => 175.944857f64,
            Self::Ta177 => 176.9444795f64,
            Self::Ta178 => 177.945678f64,
            Self::Ta179 => 178.9459366f64,
            Self::Ta180 => 179.9474648f64,
            Self::Ta181 => 180.9479958f64,
            Self::Ta182 => 181.9501519f64,
            Self::Ta183 => 182.9513726f64,
            Self::Ta184 => 183.954008f64,
            Self::Ta185 => 184.955559f64,
            Self::Ta186 => 185.958551f64,
            Self::Ta187 => 186.960386f64,
            Self::Ta188 => 187.963916f64,
            Self::Ta189 => 188.96583f64,
            Self::Ta190 => 189.96939f64,
            Self::Ta191 => 190.97156f64,
            Self::Ta192 => 191.97514f64,
        }
    }
}
impl super::ElementVariant for TantalumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ta
    }
}
impl super::MassNumber for TantalumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ta155 => 155u16,
            Self::Ta156 => 156u16,
            Self::Ta157 => 157u16,
            Self::Ta158 => 158u16,
            Self::Ta159 => 159u16,
            Self::Ta160 => 160u16,
            Self::Ta161 => 161u16,
            Self::Ta162 => 162u16,
            Self::Ta163 => 163u16,
            Self::Ta164 => 164u16,
            Self::Ta165 => 165u16,
            Self::Ta166 => 166u16,
            Self::Ta167 => 167u16,
            Self::Ta168 => 168u16,
            Self::Ta169 => 169u16,
            Self::Ta170 => 170u16,
            Self::Ta171 => 171u16,
            Self::Ta172 => 172u16,
            Self::Ta173 => 173u16,
            Self::Ta174 => 174u16,
            Self::Ta175 => 175u16,
            Self::Ta176 => 176u16,
            Self::Ta177 => 177u16,
            Self::Ta178 => 178u16,
            Self::Ta179 => 179u16,
            Self::Ta180 => 180u16,
            Self::Ta181 => 181u16,
            Self::Ta182 => 182u16,
            Self::Ta183 => 183u16,
            Self::Ta184 => 184u16,
            Self::Ta185 => 185u16,
            Self::Ta186 => 186u16,
            Self::Ta187 => 187u16,
            Self::Ta188 => 188u16,
            Self::Ta189 => 189u16,
            Self::Ta190 => 190u16,
            Self::Ta191 => 191u16,
            Self::Ta192 => 192u16,
        }
    }
}
impl super::IsotopicComposition for TantalumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ta180 => Some(0.0001201f64),
            Self::Ta181 => Some(0.9998799f64),
            Self::Ta155
            | Self::Ta156
            | Self::Ta157
            | Self::Ta158
            | Self::Ta159
            | Self::Ta160
            | Self::Ta161
            | Self::Ta162
            | Self::Ta163
            | Self::Ta164
            | Self::Ta165
            | Self::Ta166
            | Self::Ta167
            | Self::Ta168
            | Self::Ta169
            | Self::Ta170
            | Self::Ta171
            | Self::Ta172
            | Self::Ta173
            | Self::Ta174
            | Self::Ta175
            | Self::Ta176
            | Self::Ta177
            | Self::Ta178
            | Self::Ta179
            | Self::Ta182
            | Self::Ta183
            | Self::Ta184
            | Self::Ta185
            | Self::Ta186
            | Self::Ta187
            | Self::Ta188
            | Self::Ta189
            | Self::Ta190
            | Self::Ta191
            | Self::Ta192 => None,
        }
    }
}
impl super::MostAbundantIsotope for TantalumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Ta181
    }
}
impl TryFrom<u16> for TantalumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            155u16 => Ok(Self::Ta155),
            156u16 => Ok(Self::Ta156),
            157u16 => Ok(Self::Ta157),
            158u16 => Ok(Self::Ta158),
            159u16 => Ok(Self::Ta159),
            160u16 => Ok(Self::Ta160),
            161u16 => Ok(Self::Ta161),
            162u16 => Ok(Self::Ta162),
            163u16 => Ok(Self::Ta163),
            164u16 => Ok(Self::Ta164),
            165u16 => Ok(Self::Ta165),
            166u16 => Ok(Self::Ta166),
            167u16 => Ok(Self::Ta167),
            168u16 => Ok(Self::Ta168),
            169u16 => Ok(Self::Ta169),
            170u16 => Ok(Self::Ta170),
            171u16 => Ok(Self::Ta171),
            172u16 => Ok(Self::Ta172),
            173u16 => Ok(Self::Ta173),
            174u16 => Ok(Self::Ta174),
            175u16 => Ok(Self::Ta175),
            176u16 => Ok(Self::Ta176),
            177u16 => Ok(Self::Ta177),
            178u16 => Ok(Self::Ta178),
            179u16 => Ok(Self::Ta179),
            180u16 => Ok(Self::Ta180),
            181u16 => Ok(Self::Ta181),
            182u16 => Ok(Self::Ta182),
            183u16 => Ok(Self::Ta183),
            184u16 => Ok(Self::Ta184),
            185u16 => Ok(Self::Ta185),
            186u16 => Ok(Self::Ta186),
            187u16 => Ok(Self::Ta187),
            188u16 => Ok(Self::Ta188),
            189u16 => Ok(Self::Ta189),
            190u16 => Ok(Self::Ta190),
            191u16 => Ok(Self::Ta191),
            192u16 => Ok(Self::Ta192),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Ta, value)),
        }
    }
}
impl std::fmt::Display for TantalumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ta155 => write!(f, "Ta155"),
            Self::Ta156 => write!(f, "Ta156"),
            Self::Ta157 => write!(f, "Ta157"),
            Self::Ta158 => write!(f, "Ta158"),
            Self::Ta159 => write!(f, "Ta159"),
            Self::Ta160 => write!(f, "Ta160"),
            Self::Ta161 => write!(f, "Ta161"),
            Self::Ta162 => write!(f, "Ta162"),
            Self::Ta163 => write!(f, "Ta163"),
            Self::Ta164 => write!(f, "Ta164"),
            Self::Ta165 => write!(f, "Ta165"),
            Self::Ta166 => write!(f, "Ta166"),
            Self::Ta167 => write!(f, "Ta167"),
            Self::Ta168 => write!(f, "Ta168"),
            Self::Ta169 => write!(f, "Ta169"),
            Self::Ta170 => write!(f, "Ta170"),
            Self::Ta171 => write!(f, "Ta171"),
            Self::Ta172 => write!(f, "Ta172"),
            Self::Ta173 => write!(f, "Ta173"),
            Self::Ta174 => write!(f, "Ta174"),
            Self::Ta175 => write!(f, "Ta175"),
            Self::Ta176 => write!(f, "Ta176"),
            Self::Ta177 => write!(f, "Ta177"),
            Self::Ta178 => write!(f, "Ta178"),
            Self::Ta179 => write!(f, "Ta179"),
            Self::Ta180 => write!(f, "Ta180"),
            Self::Ta181 => write!(f, "Ta181"),
            Self::Ta182 => write!(f, "Ta182"),
            Self::Ta183 => write!(f, "Ta183"),
            Self::Ta184 => write!(f, "Ta184"),
            Self::Ta185 => write!(f, "Ta185"),
            Self::Ta186 => write!(f, "Ta186"),
            Self::Ta187 => write!(f, "Ta187"),
            Self::Ta188 => write!(f, "Ta188"),
            Self::Ta189 => write!(f, "Ta189"),
            Self::Ta190 => write!(f, "Ta190"),
            Self::Ta191 => write!(f, "Ta191"),
            Self::Ta192 => write!(f, "Ta192"),
        }
    }
}
