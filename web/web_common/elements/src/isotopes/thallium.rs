//! Isotopes of the element Thallium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Thallium
pub enum ThalliumIsotope {
    /// Isotope Tl176 of Thallium
    Tl176,
    /// Isotope Tl177 of Thallium
    Tl177,
    /// Isotope Tl178 of Thallium
    Tl178,
    /// Isotope Tl179 of Thallium
    Tl179,
    /// Isotope Tl180 of Thallium
    Tl180,
    /// Isotope Tl181 of Thallium
    Tl181,
    /// Isotope Tl182 of Thallium
    Tl182,
    /// Isotope Tl183 of Thallium
    Tl183,
    /// Isotope Tl184 of Thallium
    Tl184,
    /// Isotope Tl185 of Thallium
    Tl185,
    /// Isotope Tl186 of Thallium
    Tl186,
    /// Isotope Tl187 of Thallium
    Tl187,
    /// Isotope Tl188 of Thallium
    Tl188,
    /// Isotope Tl189 of Thallium
    Tl189,
    /// Isotope Tl190 of Thallium
    Tl190,
    /// Isotope Tl191 of Thallium
    Tl191,
    /// Isotope Tl192 of Thallium
    Tl192,
    /// Isotope Tl193 of Thallium
    Tl193,
    /// Isotope Tl194 of Thallium
    Tl194,
    /// Isotope Tl195 of Thallium
    Tl195,
    /// Isotope Tl196 of Thallium
    Tl196,
    /// Isotope Tl197 of Thallium
    Tl197,
    /// Isotope Tl198 of Thallium
    Tl198,
    /// Isotope Tl199 of Thallium
    Tl199,
    /// Isotope Tl200 of Thallium
    Tl200,
    /// Isotope Tl201 of Thallium
    Tl201,
    /// Isotope Tl202 of Thallium
    Tl202,
    /// Isotope Tl203 of Thallium
    Tl203,
    /// Isotope Tl204 of Thallium
    Tl204,
    /// Isotope Tl205 of Thallium
    Tl205,
    /// Isotope Tl206 of Thallium
    Tl206,
    /// Isotope Tl207 of Thallium
    Tl207,
    /// Isotope Tl208 of Thallium
    Tl208,
    /// Isotope Tl209 of Thallium
    Tl209,
    /// Isotope Tl210 of Thallium
    Tl210,
    /// Isotope Tl211 of Thallium
    Tl211,
    /// Isotope Tl212 of Thallium
    Tl212,
    /// Isotope Tl213 of Thallium
    Tl213,
    /// Isotope Tl214 of Thallium
    Tl214,
    /// Isotope Tl215 of Thallium
    Tl215,
    /// Isotope Tl216 of Thallium
    Tl216,
    /// Isotope Tl217 of Thallium
    Tl217,
    /// Isotope Tl218 of Thallium
    Tl218,
}
impl super::RelativeAtomicMass for ThalliumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Tl176 => 176.000624f64,
            Self::Tl177 => 176.996431f64,
            Self::Tl178 => 177.99485f64,
            Self::Tl179 => 178.991111f64,
            Self::Tl180 => 179.990057f64,
            Self::Tl181 => 180.98626f64,
            Self::Tl182 => 181.985713f64,
            Self::Tl183 => 182.982193f64,
            Self::Tl184 => 183.981886f64,
            Self::Tl185 => 184.978789f64,
            Self::Tl186 => 185.978651f64,
            Self::Tl187 => 186.9759063f64,
            Self::Tl188 => 187.976021f64,
            Self::Tl189 => 188.973588f64,
            Self::Tl190 => 189.973828f64,
            Self::Tl191 => 190.9717842f64,
            Self::Tl192 => 191.972225f64,
            Self::Tl193 => 192.970502f64,
            Self::Tl194 => 193.971081f64,
            Self::Tl195 => 194.969774f64,
            Self::Tl196 => 195.970481f64,
            Self::Tl197 => 196.969576f64,
            Self::Tl198 => 197.970483f64,
            Self::Tl199 => 198.969877f64,
            Self::Tl200 => 199.9709633f64,
            Self::Tl201 => 200.970822f64,
            Self::Tl202 => 201.972102f64,
            Self::Tl203 => 202.9723446f64,
            Self::Tl204 => 203.9738639f64,
            Self::Tl205 => 204.9744278f64,
            Self::Tl206 => 205.9761106f64,
            Self::Tl207 => 206.9774197f64,
            Self::Tl208 => 207.982019f64,
            Self::Tl209 => 208.9853594f64,
            Self::Tl210 => 209.990074f64,
            Self::Tl211 => 210.993475f64,
            Self::Tl212 => 211.99834f64,
            Self::Tl213 => 213.001915f64,
            Self::Tl214 => 214.00694f64,
            Self::Tl215 => 215.01064f64,
            Self::Tl216 => 216.0158f64,
            Self::Tl217 => 217.01966f64,
            Self::Tl218 => 218.02479f64,
        }
    }
}
impl super::ElementVariant for ThalliumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Tl
    }
}
impl super::MassNumber for ThalliumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Tl176 => 176u16,
            Self::Tl177 => 177u16,
            Self::Tl178 => 178u16,
            Self::Tl179 => 179u16,
            Self::Tl180 => 180u16,
            Self::Tl181 => 181u16,
            Self::Tl182 => 182u16,
            Self::Tl183 => 183u16,
            Self::Tl184 => 184u16,
            Self::Tl185 => 185u16,
            Self::Tl186 => 186u16,
            Self::Tl187 => 187u16,
            Self::Tl188 => 188u16,
            Self::Tl189 => 189u16,
            Self::Tl190 => 190u16,
            Self::Tl191 => 191u16,
            Self::Tl192 => 192u16,
            Self::Tl193 => 193u16,
            Self::Tl194 => 194u16,
            Self::Tl195 => 195u16,
            Self::Tl196 => 196u16,
            Self::Tl197 => 197u16,
            Self::Tl198 => 198u16,
            Self::Tl199 => 199u16,
            Self::Tl200 => 200u16,
            Self::Tl201 => 201u16,
            Self::Tl202 => 202u16,
            Self::Tl203 => 203u16,
            Self::Tl204 => 204u16,
            Self::Tl205 => 205u16,
            Self::Tl206 => 206u16,
            Self::Tl207 => 207u16,
            Self::Tl208 => 208u16,
            Self::Tl209 => 209u16,
            Self::Tl210 => 210u16,
            Self::Tl211 => 211u16,
            Self::Tl212 => 212u16,
            Self::Tl213 => 213u16,
            Self::Tl214 => 214u16,
            Self::Tl215 => 215u16,
            Self::Tl216 => 216u16,
            Self::Tl217 => 217u16,
            Self::Tl218 => 218u16,
        }
    }
}
impl super::IsotopicComposition for ThalliumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Tl203 => Some(0.2952f64),
            Self::Tl205 => Some(0.7048f64),
            Self::Tl176
            | Self::Tl177
            | Self::Tl178
            | Self::Tl179
            | Self::Tl180
            | Self::Tl181
            | Self::Tl182
            | Self::Tl183
            | Self::Tl184
            | Self::Tl185
            | Self::Tl186
            | Self::Tl187
            | Self::Tl188
            | Self::Tl189
            | Self::Tl190
            | Self::Tl191
            | Self::Tl192
            | Self::Tl193
            | Self::Tl194
            | Self::Tl195
            | Self::Tl196
            | Self::Tl197
            | Self::Tl198
            | Self::Tl199
            | Self::Tl200
            | Self::Tl201
            | Self::Tl202
            | Self::Tl204
            | Self::Tl206
            | Self::Tl207
            | Self::Tl208
            | Self::Tl209
            | Self::Tl210
            | Self::Tl211
            | Self::Tl212
            | Self::Tl213
            | Self::Tl214
            | Self::Tl215
            | Self::Tl216
            | Self::Tl217
            | Self::Tl218 => None,
        }
    }
}
impl super::MostAbundantIsotope for ThalliumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Tl205
    }
}
impl TryFrom<u16> for ThalliumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            176u16 => Ok(Self::Tl176),
            177u16 => Ok(Self::Tl177),
            178u16 => Ok(Self::Tl178),
            179u16 => Ok(Self::Tl179),
            180u16 => Ok(Self::Tl180),
            181u16 => Ok(Self::Tl181),
            182u16 => Ok(Self::Tl182),
            183u16 => Ok(Self::Tl183),
            184u16 => Ok(Self::Tl184),
            185u16 => Ok(Self::Tl185),
            186u16 => Ok(Self::Tl186),
            187u16 => Ok(Self::Tl187),
            188u16 => Ok(Self::Tl188),
            189u16 => Ok(Self::Tl189),
            190u16 => Ok(Self::Tl190),
            191u16 => Ok(Self::Tl191),
            192u16 => Ok(Self::Tl192),
            193u16 => Ok(Self::Tl193),
            194u16 => Ok(Self::Tl194),
            195u16 => Ok(Self::Tl195),
            196u16 => Ok(Self::Tl196),
            197u16 => Ok(Self::Tl197),
            198u16 => Ok(Self::Tl198),
            199u16 => Ok(Self::Tl199),
            200u16 => Ok(Self::Tl200),
            201u16 => Ok(Self::Tl201),
            202u16 => Ok(Self::Tl202),
            203u16 => Ok(Self::Tl203),
            204u16 => Ok(Self::Tl204),
            205u16 => Ok(Self::Tl205),
            206u16 => Ok(Self::Tl206),
            207u16 => Ok(Self::Tl207),
            208u16 => Ok(Self::Tl208),
            209u16 => Ok(Self::Tl209),
            210u16 => Ok(Self::Tl210),
            211u16 => Ok(Self::Tl211),
            212u16 => Ok(Self::Tl212),
            213u16 => Ok(Self::Tl213),
            214u16 => Ok(Self::Tl214),
            215u16 => Ok(Self::Tl215),
            216u16 => Ok(Self::Tl216),
            217u16 => Ok(Self::Tl217),
            218u16 => Ok(Self::Tl218),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Tl, value)),
        }
    }
}
impl std::fmt::Display for ThalliumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tl176 => write!(f, "Tl176"),
            Self::Tl177 => write!(f, "Tl177"),
            Self::Tl178 => write!(f, "Tl178"),
            Self::Tl179 => write!(f, "Tl179"),
            Self::Tl180 => write!(f, "Tl180"),
            Self::Tl181 => write!(f, "Tl181"),
            Self::Tl182 => write!(f, "Tl182"),
            Self::Tl183 => write!(f, "Tl183"),
            Self::Tl184 => write!(f, "Tl184"),
            Self::Tl185 => write!(f, "Tl185"),
            Self::Tl186 => write!(f, "Tl186"),
            Self::Tl187 => write!(f, "Tl187"),
            Self::Tl188 => write!(f, "Tl188"),
            Self::Tl189 => write!(f, "Tl189"),
            Self::Tl190 => write!(f, "Tl190"),
            Self::Tl191 => write!(f, "Tl191"),
            Self::Tl192 => write!(f, "Tl192"),
            Self::Tl193 => write!(f, "Tl193"),
            Self::Tl194 => write!(f, "Tl194"),
            Self::Tl195 => write!(f, "Tl195"),
            Self::Tl196 => write!(f, "Tl196"),
            Self::Tl197 => write!(f, "Tl197"),
            Self::Tl198 => write!(f, "Tl198"),
            Self::Tl199 => write!(f, "Tl199"),
            Self::Tl200 => write!(f, "Tl200"),
            Self::Tl201 => write!(f, "Tl201"),
            Self::Tl202 => write!(f, "Tl202"),
            Self::Tl203 => write!(f, "Tl203"),
            Self::Tl204 => write!(f, "Tl204"),
            Self::Tl205 => write!(f, "Tl205"),
            Self::Tl206 => write!(f, "Tl206"),
            Self::Tl207 => write!(f, "Tl207"),
            Self::Tl208 => write!(f, "Tl208"),
            Self::Tl209 => write!(f, "Tl209"),
            Self::Tl210 => write!(f, "Tl210"),
            Self::Tl211 => write!(f, "Tl211"),
            Self::Tl212 => write!(f, "Tl212"),
            Self::Tl213 => write!(f, "Tl213"),
            Self::Tl214 => write!(f, "Tl214"),
            Self::Tl215 => write!(f, "Tl215"),
            Self::Tl216 => write!(f, "Tl216"),
            Self::Tl217 => write!(f, "Tl217"),
            Self::Tl218 => write!(f, "Tl218"),
        }
    }
}
