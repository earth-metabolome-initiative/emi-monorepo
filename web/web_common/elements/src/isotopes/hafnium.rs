//! Isotopes of the element Hafnium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Hafnium
pub enum HafniumIsotope {
    /// Isotope Hf153 of Hafnium
    Hf153,
    /// Isotope Hf154 of Hafnium
    Hf154,
    /// Isotope Hf155 of Hafnium
    Hf155,
    /// Isotope Hf156 of Hafnium
    Hf156,
    /// Isotope Hf157 of Hafnium
    Hf157,
    /// Isotope Hf158 of Hafnium
    Hf158,
    /// Isotope Hf159 of Hafnium
    Hf159,
    /// Isotope Hf160 of Hafnium
    Hf160,
    /// Isotope Hf161 of Hafnium
    Hf161,
    /// Isotope Hf162 of Hafnium
    Hf162,
    /// Isotope Hf163 of Hafnium
    Hf163,
    /// Isotope Hf164 of Hafnium
    Hf164,
    /// Isotope Hf165 of Hafnium
    Hf165,
    /// Isotope Hf166 of Hafnium
    Hf166,
    /// Isotope Hf167 of Hafnium
    Hf167,
    /// Isotope Hf168 of Hafnium
    Hf168,
    /// Isotope Hf169 of Hafnium
    Hf169,
    /// Isotope Hf170 of Hafnium
    Hf170,
    /// Isotope Hf171 of Hafnium
    Hf171,
    /// Isotope Hf172 of Hafnium
    Hf172,
    /// Isotope Hf173 of Hafnium
    Hf173,
    /// Isotope Hf174 of Hafnium
    Hf174,
    /// Isotope Hf175 of Hafnium
    Hf175,
    /// Isotope Hf176 of Hafnium
    Hf176,
    /// Isotope Hf177 of Hafnium
    Hf177,
    /// Isotope Hf178 of Hafnium
    Hf178,
    /// Isotope Hf179 of Hafnium
    Hf179,
    /// Isotope Hf180 of Hafnium
    Hf180,
    /// Isotope Hf181 of Hafnium
    Hf181,
    /// Isotope Hf182 of Hafnium
    Hf182,
    /// Isotope Hf183 of Hafnium
    Hf183,
    /// Isotope Hf184 of Hafnium
    Hf184,
    /// Isotope Hf185 of Hafnium
    Hf185,
    /// Isotope Hf186 of Hafnium
    Hf186,
    /// Isotope Hf187 of Hafnium
    Hf187,
    /// Isotope Hf188 of Hafnium
    Hf188,
    /// Isotope Hf189 of Hafnium
    Hf189,
}
impl super::RelativeAtomicMass for HafniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Hf153 => 152.97069f64,
            Self::Hf154 => 153.96486f64,
            Self::Hf155 => 154.96311f64,
            Self::Hf156 => 155.95935f64,
            Self::Hf157 => 156.95824f64,
            Self::Hf158 => 157.954801f64,
            Self::Hf159 => 158.953996f64,
            Self::Hf160 => 159.950691f64,
            Self::Hf161 => 160.950278f64,
            Self::Hf162 => 161.9472148f64,
            Self::Hf163 => 162.947113f64,
            Self::Hf164 => 163.944371f64,
            Self::Hf165 => 164.944567f64,
            Self::Hf166 => 165.94218f64,
            Self::Hf167 => 166.9426f64,
            Self::Hf168 => 167.940568f64,
            Self::Hf169 => 168.941259f64,
            Self::Hf170 => 169.939609f64,
            Self::Hf171 => 170.940492f64,
            Self::Hf172 => 171.93945f64,
            Self::Hf173 => 172.940513f64,
            Self::Hf174 => 173.9400461f64,
            Self::Hf175 => 174.9415092f64,
            Self::Hf176 => 175.9414076f64,
            Self::Hf177 => 176.9432277f64,
            Self::Hf178 => 177.9437058f64,
            Self::Hf179 => 178.9458232f64,
            Self::Hf180 => 179.946557f64,
            Self::Hf181 => 180.9491083f64,
            Self::Hf182 => 181.9505612f64,
            Self::Hf183 => 182.95353f64,
            Self::Hf184 => 183.955446f64,
            Self::Hf185 => 184.958862f64,
            Self::Hf186 => 185.960897f64,
            Self::Hf187 => 186.96477f64,
            Self::Hf188 => 187.96685f64,
            Self::Hf189 => 188.97084f64,
        }
    }
}
impl super::ElementVariant for HafniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Hf
    }
}
impl super::MassNumber for HafniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Hf153 => 153u16,
            Self::Hf154 => 154u16,
            Self::Hf155 => 155u16,
            Self::Hf156 => 156u16,
            Self::Hf157 => 157u16,
            Self::Hf158 => 158u16,
            Self::Hf159 => 159u16,
            Self::Hf160 => 160u16,
            Self::Hf161 => 161u16,
            Self::Hf162 => 162u16,
            Self::Hf163 => 163u16,
            Self::Hf164 => 164u16,
            Self::Hf165 => 165u16,
            Self::Hf166 => 166u16,
            Self::Hf167 => 167u16,
            Self::Hf168 => 168u16,
            Self::Hf169 => 169u16,
            Self::Hf170 => 170u16,
            Self::Hf171 => 171u16,
            Self::Hf172 => 172u16,
            Self::Hf173 => 173u16,
            Self::Hf174 => 174u16,
            Self::Hf175 => 175u16,
            Self::Hf176 => 176u16,
            Self::Hf177 => 177u16,
            Self::Hf178 => 178u16,
            Self::Hf179 => 179u16,
            Self::Hf180 => 180u16,
            Self::Hf181 => 181u16,
            Self::Hf182 => 182u16,
            Self::Hf183 => 183u16,
            Self::Hf184 => 184u16,
            Self::Hf185 => 185u16,
            Self::Hf186 => 186u16,
            Self::Hf187 => 187u16,
            Self::Hf188 => 188u16,
            Self::Hf189 => 189u16,
        }
    }
}
impl super::IsotopicComposition for HafniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Hf174 => Some(0.0016f64),
            Self::Hf176 => Some(0.0526f64),
            Self::Hf177 => Some(0.186f64),
            Self::Hf178 => Some(0.2728f64),
            Self::Hf179 => Some(0.1362f64),
            Self::Hf180 => Some(0.3508f64),
            Self::Hf153
            | Self::Hf154
            | Self::Hf155
            | Self::Hf156
            | Self::Hf157
            | Self::Hf158
            | Self::Hf159
            | Self::Hf160
            | Self::Hf161
            | Self::Hf162
            | Self::Hf163
            | Self::Hf164
            | Self::Hf165
            | Self::Hf166
            | Self::Hf167
            | Self::Hf168
            | Self::Hf169
            | Self::Hf170
            | Self::Hf171
            | Self::Hf172
            | Self::Hf173
            | Self::Hf175
            | Self::Hf181
            | Self::Hf182
            | Self::Hf183
            | Self::Hf184
            | Self::Hf185
            | Self::Hf186
            | Self::Hf187
            | Self::Hf188
            | Self::Hf189 => None,
        }
    }
}
impl super::MostAbundantIsotope for HafniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Hf180
    }
}
impl TryFrom<u16> for HafniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            153u16 => Ok(Self::Hf153),
            154u16 => Ok(Self::Hf154),
            155u16 => Ok(Self::Hf155),
            156u16 => Ok(Self::Hf156),
            157u16 => Ok(Self::Hf157),
            158u16 => Ok(Self::Hf158),
            159u16 => Ok(Self::Hf159),
            160u16 => Ok(Self::Hf160),
            161u16 => Ok(Self::Hf161),
            162u16 => Ok(Self::Hf162),
            163u16 => Ok(Self::Hf163),
            164u16 => Ok(Self::Hf164),
            165u16 => Ok(Self::Hf165),
            166u16 => Ok(Self::Hf166),
            167u16 => Ok(Self::Hf167),
            168u16 => Ok(Self::Hf168),
            169u16 => Ok(Self::Hf169),
            170u16 => Ok(Self::Hf170),
            171u16 => Ok(Self::Hf171),
            172u16 => Ok(Self::Hf172),
            173u16 => Ok(Self::Hf173),
            174u16 => Ok(Self::Hf174),
            175u16 => Ok(Self::Hf175),
            176u16 => Ok(Self::Hf176),
            177u16 => Ok(Self::Hf177),
            178u16 => Ok(Self::Hf178),
            179u16 => Ok(Self::Hf179),
            180u16 => Ok(Self::Hf180),
            181u16 => Ok(Self::Hf181),
            182u16 => Ok(Self::Hf182),
            183u16 => Ok(Self::Hf183),
            184u16 => Ok(Self::Hf184),
            185u16 => Ok(Self::Hf185),
            186u16 => Ok(Self::Hf186),
            187u16 => Ok(Self::Hf187),
            188u16 => Ok(Self::Hf188),
            189u16 => Ok(Self::Hf189),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Hf, value)),
        }
    }
}
impl std::fmt::Display for HafniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hf153 => write!(f, "Hf153"),
            Self::Hf154 => write!(f, "Hf154"),
            Self::Hf155 => write!(f, "Hf155"),
            Self::Hf156 => write!(f, "Hf156"),
            Self::Hf157 => write!(f, "Hf157"),
            Self::Hf158 => write!(f, "Hf158"),
            Self::Hf159 => write!(f, "Hf159"),
            Self::Hf160 => write!(f, "Hf160"),
            Self::Hf161 => write!(f, "Hf161"),
            Self::Hf162 => write!(f, "Hf162"),
            Self::Hf163 => write!(f, "Hf163"),
            Self::Hf164 => write!(f, "Hf164"),
            Self::Hf165 => write!(f, "Hf165"),
            Self::Hf166 => write!(f, "Hf166"),
            Self::Hf167 => write!(f, "Hf167"),
            Self::Hf168 => write!(f, "Hf168"),
            Self::Hf169 => write!(f, "Hf169"),
            Self::Hf170 => write!(f, "Hf170"),
            Self::Hf171 => write!(f, "Hf171"),
            Self::Hf172 => write!(f, "Hf172"),
            Self::Hf173 => write!(f, "Hf173"),
            Self::Hf174 => write!(f, "Hf174"),
            Self::Hf175 => write!(f, "Hf175"),
            Self::Hf176 => write!(f, "Hf176"),
            Self::Hf177 => write!(f, "Hf177"),
            Self::Hf178 => write!(f, "Hf178"),
            Self::Hf179 => write!(f, "Hf179"),
            Self::Hf180 => write!(f, "Hf180"),
            Self::Hf181 => write!(f, "Hf181"),
            Self::Hf182 => write!(f, "Hf182"),
            Self::Hf183 => write!(f, "Hf183"),
            Self::Hf184 => write!(f, "Hf184"),
            Self::Hf185 => write!(f, "Hf185"),
            Self::Hf186 => write!(f, "Hf186"),
            Self::Hf187 => write!(f, "Hf187"),
            Self::Hf188 => write!(f, "Hf188"),
            Self::Hf189 => write!(f, "Hf189"),
        }
    }
}
