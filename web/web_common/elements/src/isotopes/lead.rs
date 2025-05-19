//! Isotopes of the element Lead
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Lead
pub enum LeadIsotope {
    /// Isotope Pb178 of Lead
    Pb178,
    /// Isotope Pb179 of Lead
    Pb179,
    /// Isotope Pb180 of Lead
    Pb180,
    /// Isotope Pb181 of Lead
    Pb181,
    /// Isotope Pb182 of Lead
    Pb182,
    /// Isotope Pb183 of Lead
    Pb183,
    /// Isotope Pb184 of Lead
    Pb184,
    /// Isotope Pb185 of Lead
    Pb185,
    /// Isotope Pb186 of Lead
    Pb186,
    /// Isotope Pb187 of Lead
    Pb187,
    /// Isotope Pb188 of Lead
    Pb188,
    /// Isotope Pb189 of Lead
    Pb189,
    /// Isotope Pb190 of Lead
    Pb190,
    /// Isotope Pb191 of Lead
    Pb191,
    /// Isotope Pb192 of Lead
    Pb192,
    /// Isotope Pb193 of Lead
    Pb193,
    /// Isotope Pb194 of Lead
    Pb194,
    /// Isotope Pb195 of Lead
    Pb195,
    /// Isotope Pb196 of Lead
    Pb196,
    /// Isotope Pb197 of Lead
    Pb197,
    /// Isotope Pb198 of Lead
    Pb198,
    /// Isotope Pb199 of Lead
    Pb199,
    /// Isotope Pb200 of Lead
    Pb200,
    /// Isotope Pb201 of Lead
    Pb201,
    /// Isotope Pb202 of Lead
    Pb202,
    /// Isotope Pb203 of Lead
    Pb203,
    /// Isotope Pb204 of Lead
    Pb204,
    /// Isotope Pb205 of Lead
    Pb205,
    /// Isotope Pb206 of Lead
    Pb206,
    /// Isotope Pb207 of Lead
    Pb207,
    /// Isotope Pb208 of Lead
    Pb208,
    /// Isotope Pb209 of Lead
    Pb209,
    /// Isotope Pb210 of Lead
    Pb210,
    /// Isotope Pb211 of Lead
    Pb211,
    /// Isotope Pb212 of Lead
    Pb212,
    /// Isotope Pb213 of Lead
    Pb213,
    /// Isotope Pb214 of Lead
    Pb214,
    /// Isotope Pb215 of Lead
    Pb215,
    /// Isotope Pb216 of Lead
    Pb216,
    /// Isotope Pb217 of Lead
    Pb217,
    /// Isotope Pb218 of Lead
    Pb218,
    /// Isotope Pb219 of Lead
    Pb219,
    /// Isotope Pb220 of Lead
    Pb220,
}
impl super::RelativeAtomicMass for LeadIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Pb178 => 178.003831f64,
            Self::Pb179 => 179.002201f64,
            Self::Pb180 => 179.997928f64,
            Self::Pb181 => 180.996653f64,
            Self::Pb182 => 181.992672f64,
            Self::Pb183 => 182.991872f64,
            Self::Pb184 => 183.988136f64,
            Self::Pb185 => 184.98761f64,
            Self::Pb186 => 185.984238f64,
            Self::Pb187 => 186.9839109f64,
            Self::Pb188 => 187.980875f64,
            Self::Pb189 => 188.980807f64,
            Self::Pb190 => 189.978082f64,
            Self::Pb191 => 190.978276f64,
            Self::Pb192 => 191.975775f64,
            Self::Pb193 => 192.976173f64,
            Self::Pb194 => 193.974012f64,
            Self::Pb195 => 194.974543f64,
            Self::Pb196 => 195.972774f64,
            Self::Pb197 => 196.9734312f64,
            Self::Pb198 => 197.972034f64,
            Self::Pb199 => 198.972913f64,
            Self::Pb200 => 199.971819f64,
            Self::Pb201 => 200.972883f64,
            Self::Pb202 => 201.972152f64,
            Self::Pb203 => 202.9733911f64,
            Self::Pb204 => 203.973044f64,
            Self::Pb205 => 204.9744822f64,
            Self::Pb206 => 205.9744657f64,
            Self::Pb207 => 206.9758973f64,
            Self::Pb208 => 207.9766525f64,
            Self::Pb209 => 208.9810905f64,
            Self::Pb210 => 209.9841889f64,
            Self::Pb211 => 210.9887371f64,
            Self::Pb212 => 211.9918977f64,
            Self::Pb213 => 212.9965629f64,
            Self::Pb214 => 213.9998059f64,
            Self::Pb215 => 215.00474f64,
            Self::Pb216 => 216.00803f64,
            Self::Pb217 => 217.01314f64,
            Self::Pb218 => 218.01659f64,
            Self::Pb219 => 219.02177f64,
            Self::Pb220 => 220.02541f64,
        }
    }
}
impl super::ElementVariant for LeadIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Pb
    }
}
impl super::MassNumber for LeadIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Pb178 => 178u16,
            Self::Pb179 => 179u16,
            Self::Pb180 => 180u16,
            Self::Pb181 => 181u16,
            Self::Pb182 => 182u16,
            Self::Pb183 => 183u16,
            Self::Pb184 => 184u16,
            Self::Pb185 => 185u16,
            Self::Pb186 => 186u16,
            Self::Pb187 => 187u16,
            Self::Pb188 => 188u16,
            Self::Pb189 => 189u16,
            Self::Pb190 => 190u16,
            Self::Pb191 => 191u16,
            Self::Pb192 => 192u16,
            Self::Pb193 => 193u16,
            Self::Pb194 => 194u16,
            Self::Pb195 => 195u16,
            Self::Pb196 => 196u16,
            Self::Pb197 => 197u16,
            Self::Pb198 => 198u16,
            Self::Pb199 => 199u16,
            Self::Pb200 => 200u16,
            Self::Pb201 => 201u16,
            Self::Pb202 => 202u16,
            Self::Pb203 => 203u16,
            Self::Pb204 => 204u16,
            Self::Pb205 => 205u16,
            Self::Pb206 => 206u16,
            Self::Pb207 => 207u16,
            Self::Pb208 => 208u16,
            Self::Pb209 => 209u16,
            Self::Pb210 => 210u16,
            Self::Pb211 => 211u16,
            Self::Pb212 => 212u16,
            Self::Pb213 => 213u16,
            Self::Pb214 => 214u16,
            Self::Pb215 => 215u16,
            Self::Pb216 => 216u16,
            Self::Pb217 => 217u16,
            Self::Pb218 => 218u16,
            Self::Pb219 => 219u16,
            Self::Pb220 => 220u16,
        }
    }
}
impl super::IsotopicComposition for LeadIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Pb204 => Some(0.014f64),
            Self::Pb206 => Some(0.241f64),
            Self::Pb207 => Some(0.221f64),
            Self::Pb208 => Some(0.524f64),
            Self::Pb178
            | Self::Pb179
            | Self::Pb180
            | Self::Pb181
            | Self::Pb182
            | Self::Pb183
            | Self::Pb184
            | Self::Pb185
            | Self::Pb186
            | Self::Pb187
            | Self::Pb188
            | Self::Pb189
            | Self::Pb190
            | Self::Pb191
            | Self::Pb192
            | Self::Pb193
            | Self::Pb194
            | Self::Pb195
            | Self::Pb196
            | Self::Pb197
            | Self::Pb198
            | Self::Pb199
            | Self::Pb200
            | Self::Pb201
            | Self::Pb202
            | Self::Pb203
            | Self::Pb205
            | Self::Pb209
            | Self::Pb210
            | Self::Pb211
            | Self::Pb212
            | Self::Pb213
            | Self::Pb214
            | Self::Pb215
            | Self::Pb216
            | Self::Pb217
            | Self::Pb218
            | Self::Pb219
            | Self::Pb220 => None,
        }
    }
}
impl super::MostAbundantIsotope for LeadIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Pb208
    }
}
impl TryFrom<u16> for LeadIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            178u16 => Ok(Self::Pb178),
            179u16 => Ok(Self::Pb179),
            180u16 => Ok(Self::Pb180),
            181u16 => Ok(Self::Pb181),
            182u16 => Ok(Self::Pb182),
            183u16 => Ok(Self::Pb183),
            184u16 => Ok(Self::Pb184),
            185u16 => Ok(Self::Pb185),
            186u16 => Ok(Self::Pb186),
            187u16 => Ok(Self::Pb187),
            188u16 => Ok(Self::Pb188),
            189u16 => Ok(Self::Pb189),
            190u16 => Ok(Self::Pb190),
            191u16 => Ok(Self::Pb191),
            192u16 => Ok(Self::Pb192),
            193u16 => Ok(Self::Pb193),
            194u16 => Ok(Self::Pb194),
            195u16 => Ok(Self::Pb195),
            196u16 => Ok(Self::Pb196),
            197u16 => Ok(Self::Pb197),
            198u16 => Ok(Self::Pb198),
            199u16 => Ok(Self::Pb199),
            200u16 => Ok(Self::Pb200),
            201u16 => Ok(Self::Pb201),
            202u16 => Ok(Self::Pb202),
            203u16 => Ok(Self::Pb203),
            204u16 => Ok(Self::Pb204),
            205u16 => Ok(Self::Pb205),
            206u16 => Ok(Self::Pb206),
            207u16 => Ok(Self::Pb207),
            208u16 => Ok(Self::Pb208),
            209u16 => Ok(Self::Pb209),
            210u16 => Ok(Self::Pb210),
            211u16 => Ok(Self::Pb211),
            212u16 => Ok(Self::Pb212),
            213u16 => Ok(Self::Pb213),
            214u16 => Ok(Self::Pb214),
            215u16 => Ok(Self::Pb215),
            216u16 => Ok(Self::Pb216),
            217u16 => Ok(Self::Pb217),
            218u16 => Ok(Self::Pb218),
            219u16 => Ok(Self::Pb219),
            220u16 => Ok(Self::Pb220),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Pb, value)),
        }
    }
}
impl std::fmt::Display for LeadIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pb178 => write!(f, "Pb178"),
            Self::Pb179 => write!(f, "Pb179"),
            Self::Pb180 => write!(f, "Pb180"),
            Self::Pb181 => write!(f, "Pb181"),
            Self::Pb182 => write!(f, "Pb182"),
            Self::Pb183 => write!(f, "Pb183"),
            Self::Pb184 => write!(f, "Pb184"),
            Self::Pb185 => write!(f, "Pb185"),
            Self::Pb186 => write!(f, "Pb186"),
            Self::Pb187 => write!(f, "Pb187"),
            Self::Pb188 => write!(f, "Pb188"),
            Self::Pb189 => write!(f, "Pb189"),
            Self::Pb190 => write!(f, "Pb190"),
            Self::Pb191 => write!(f, "Pb191"),
            Self::Pb192 => write!(f, "Pb192"),
            Self::Pb193 => write!(f, "Pb193"),
            Self::Pb194 => write!(f, "Pb194"),
            Self::Pb195 => write!(f, "Pb195"),
            Self::Pb196 => write!(f, "Pb196"),
            Self::Pb197 => write!(f, "Pb197"),
            Self::Pb198 => write!(f, "Pb198"),
            Self::Pb199 => write!(f, "Pb199"),
            Self::Pb200 => write!(f, "Pb200"),
            Self::Pb201 => write!(f, "Pb201"),
            Self::Pb202 => write!(f, "Pb202"),
            Self::Pb203 => write!(f, "Pb203"),
            Self::Pb204 => write!(f, "Pb204"),
            Self::Pb205 => write!(f, "Pb205"),
            Self::Pb206 => write!(f, "Pb206"),
            Self::Pb207 => write!(f, "Pb207"),
            Self::Pb208 => write!(f, "Pb208"),
            Self::Pb209 => write!(f, "Pb209"),
            Self::Pb210 => write!(f, "Pb210"),
            Self::Pb211 => write!(f, "Pb211"),
            Self::Pb212 => write!(f, "Pb212"),
            Self::Pb213 => write!(f, "Pb213"),
            Self::Pb214 => write!(f, "Pb214"),
            Self::Pb215 => write!(f, "Pb215"),
            Self::Pb216 => write!(f, "Pb216"),
            Self::Pb217 => write!(f, "Pb217"),
            Self::Pb218 => write!(f, "Pb218"),
            Self::Pb219 => write!(f, "Pb219"),
            Self::Pb220 => write!(f, "Pb220"),
        }
    }
}
