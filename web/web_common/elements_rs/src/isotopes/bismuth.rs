//! Isotopes of the element Bismuth
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Bismuth
pub enum BismuthIsotope {
    /// Isotope Bi184 of Bismuth
    Bi184,
    /// Isotope Bi185 of Bismuth
    Bi185,
    /// Isotope Bi186 of Bismuth
    Bi186,
    /// Isotope Bi187 of Bismuth
    Bi187,
    /// Isotope Bi188 of Bismuth
    Bi188,
    /// Isotope Bi189 of Bismuth
    Bi189,
    /// Isotope Bi190 of Bismuth
    Bi190,
    /// Isotope Bi191 of Bismuth
    Bi191,
    /// Isotope Bi192 of Bismuth
    Bi192,
    /// Isotope Bi193 of Bismuth
    Bi193,
    /// Isotope Bi194 of Bismuth
    Bi194,
    /// Isotope Bi195 of Bismuth
    Bi195,
    /// Isotope Bi196 of Bismuth
    Bi196,
    /// Isotope Bi197 of Bismuth
    Bi197,
    /// Isotope Bi198 of Bismuth
    Bi198,
    /// Isotope Bi199 of Bismuth
    Bi199,
    /// Isotope Bi200 of Bismuth
    Bi200,
    /// Isotope Bi201 of Bismuth
    Bi201,
    /// Isotope Bi202 of Bismuth
    Bi202,
    /// Isotope Bi203 of Bismuth
    Bi203,
    /// Isotope Bi204 of Bismuth
    Bi204,
    /// Isotope Bi205 of Bismuth
    Bi205,
    /// Isotope Bi206 of Bismuth
    Bi206,
    /// Isotope Bi207 of Bismuth
    Bi207,
    /// Isotope Bi208 of Bismuth
    Bi208,
    /// Isotope Bi209 of Bismuth
    Bi209,
    /// Isotope Bi210 of Bismuth
    Bi210,
    /// Isotope Bi211 of Bismuth
    Bi211,
    /// Isotope Bi212 of Bismuth
    Bi212,
    /// Isotope Bi213 of Bismuth
    Bi213,
    /// Isotope Bi214 of Bismuth
    Bi214,
    /// Isotope Bi215 of Bismuth
    Bi215,
    /// Isotope Bi216 of Bismuth
    Bi216,
    /// Isotope Bi217 of Bismuth
    Bi217,
    /// Isotope Bi218 of Bismuth
    Bi218,
    /// Isotope Bi219 of Bismuth
    Bi219,
    /// Isotope Bi220 of Bismuth
    Bi220,
    /// Isotope Bi221 of Bismuth
    Bi221,
    /// Isotope Bi222 of Bismuth
    Bi222,
    /// Isotope Bi223 of Bismuth
    Bi223,
    /// Isotope Bi224 of Bismuth
    Bi224,
}
impl super::RelativeAtomicMass for BismuthIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Bi184 => 184.001275f64,
            Self::Bi185 => 184.9976f64,
            Self::Bi186 => 185.996644f64,
            Self::Bi187 => 186.993147f64,
            Self::Bi188 => 187.992287f64,
            Self::Bi189 => 188.989195f64,
            Self::Bi190 => 189.988622f64,
            Self::Bi191 => 190.9857866f64,
            Self::Bi192 => 191.985469f64,
            Self::Bi193 => 192.98296f64,
            Self::Bi194 => 193.982785f64,
            Self::Bi195 => 194.9806488f64,
            Self::Bi196 => 195.980667f64,
            Self::Bi197 => 196.9788651f64,
            Self::Bi198 => 197.979206f64,
            Self::Bi199 => 198.977673f64,
            Self::Bi200 => 199.978131f64,
            Self::Bi201 => 200.97701f64,
            Self::Bi202 => 201.977734f64,
            Self::Bi203 => 202.976893f64,
            Self::Bi204 => 203.9778361f64,
            Self::Bi205 => 204.9773867f64,
            Self::Bi206 => 205.9784993f64,
            Self::Bi207 => 206.978471f64,
            Self::Bi208 => 207.9797425f64,
            Self::Bi209 => 208.9803991f64,
            Self::Bi210 => 209.9841207f64,
            Self::Bi211 => 210.9872697f64,
            Self::Bi212 => 211.991286f64,
            Self::Bi213 => 212.9943851f64,
            Self::Bi214 => 213.998712f64,
            Self::Bi215 => 215.00177f64,
            Self::Bi216 => 216.006306f64,
            Self::Bi217 => 217.009372f64,
            Self::Bi218 => 218.014188f64,
            Self::Bi219 => 219.01748f64,
            Self::Bi220 => 220.02235f64,
            Self::Bi221 => 221.02587f64,
            Self::Bi222 => 222.03078f64,
            Self::Bi223 => 223.0345f64,
            Self::Bi224 => 224.03947f64,
        }
    }
}
impl super::ElementVariant for BismuthIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Bi
    }
}
impl super::MassNumber for BismuthIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Bi184 => 184u16,
            Self::Bi185 => 185u16,
            Self::Bi186 => 186u16,
            Self::Bi187 => 187u16,
            Self::Bi188 => 188u16,
            Self::Bi189 => 189u16,
            Self::Bi190 => 190u16,
            Self::Bi191 => 191u16,
            Self::Bi192 => 192u16,
            Self::Bi193 => 193u16,
            Self::Bi194 => 194u16,
            Self::Bi195 => 195u16,
            Self::Bi196 => 196u16,
            Self::Bi197 => 197u16,
            Self::Bi198 => 198u16,
            Self::Bi199 => 199u16,
            Self::Bi200 => 200u16,
            Self::Bi201 => 201u16,
            Self::Bi202 => 202u16,
            Self::Bi203 => 203u16,
            Self::Bi204 => 204u16,
            Self::Bi205 => 205u16,
            Self::Bi206 => 206u16,
            Self::Bi207 => 207u16,
            Self::Bi208 => 208u16,
            Self::Bi209 => 209u16,
            Self::Bi210 => 210u16,
            Self::Bi211 => 211u16,
            Self::Bi212 => 212u16,
            Self::Bi213 => 213u16,
            Self::Bi214 => 214u16,
            Self::Bi215 => 215u16,
            Self::Bi216 => 216u16,
            Self::Bi217 => 217u16,
            Self::Bi218 => 218u16,
            Self::Bi219 => 219u16,
            Self::Bi220 => 220u16,
            Self::Bi221 => 221u16,
            Self::Bi222 => 222u16,
            Self::Bi223 => 223u16,
            Self::Bi224 => 224u16,
        }
    }
}
impl super::IsotopicComposition for BismuthIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Bi209 => Some(1f64),
            Self::Bi184
            | Self::Bi185
            | Self::Bi186
            | Self::Bi187
            | Self::Bi188
            | Self::Bi189
            | Self::Bi190
            | Self::Bi191
            | Self::Bi192
            | Self::Bi193
            | Self::Bi194
            | Self::Bi195
            | Self::Bi196
            | Self::Bi197
            | Self::Bi198
            | Self::Bi199
            | Self::Bi200
            | Self::Bi201
            | Self::Bi202
            | Self::Bi203
            | Self::Bi204
            | Self::Bi205
            | Self::Bi206
            | Self::Bi207
            | Self::Bi208
            | Self::Bi210
            | Self::Bi211
            | Self::Bi212
            | Self::Bi213
            | Self::Bi214
            | Self::Bi215
            | Self::Bi216
            | Self::Bi217
            | Self::Bi218
            | Self::Bi219
            | Self::Bi220
            | Self::Bi221
            | Self::Bi222
            | Self::Bi223
            | Self::Bi224 => None,
        }
    }
}
impl super::MostAbundantIsotope for BismuthIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Bi209
    }
}
impl From<BismuthIsotope> for crate::Isotope {
    fn from(isotope: BismuthIsotope) -> Self {
        crate::Isotope::Bi(isotope)
    }
}
impl From<BismuthIsotope> for crate::Element {
    fn from(_isotope: BismuthIsotope) -> Self {
        crate::Element::Bi
    }
}
impl TryFrom<u16> for BismuthIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            184u16 => Ok(Self::Bi184),
            185u16 => Ok(Self::Bi185),
            186u16 => Ok(Self::Bi186),
            187u16 => Ok(Self::Bi187),
            188u16 => Ok(Self::Bi188),
            189u16 => Ok(Self::Bi189),
            190u16 => Ok(Self::Bi190),
            191u16 => Ok(Self::Bi191),
            192u16 => Ok(Self::Bi192),
            193u16 => Ok(Self::Bi193),
            194u16 => Ok(Self::Bi194),
            195u16 => Ok(Self::Bi195),
            196u16 => Ok(Self::Bi196),
            197u16 => Ok(Self::Bi197),
            198u16 => Ok(Self::Bi198),
            199u16 => Ok(Self::Bi199),
            200u16 => Ok(Self::Bi200),
            201u16 => Ok(Self::Bi201),
            202u16 => Ok(Self::Bi202),
            203u16 => Ok(Self::Bi203),
            204u16 => Ok(Self::Bi204),
            205u16 => Ok(Self::Bi205),
            206u16 => Ok(Self::Bi206),
            207u16 => Ok(Self::Bi207),
            208u16 => Ok(Self::Bi208),
            209u16 => Ok(Self::Bi209),
            210u16 => Ok(Self::Bi210),
            211u16 => Ok(Self::Bi211),
            212u16 => Ok(Self::Bi212),
            213u16 => Ok(Self::Bi213),
            214u16 => Ok(Self::Bi214),
            215u16 => Ok(Self::Bi215),
            216u16 => Ok(Self::Bi216),
            217u16 => Ok(Self::Bi217),
            218u16 => Ok(Self::Bi218),
            219u16 => Ok(Self::Bi219),
            220u16 => Ok(Self::Bi220),
            221u16 => Ok(Self::Bi221),
            222u16 => Ok(Self::Bi222),
            223u16 => Ok(Self::Bi223),
            224u16 => Ok(Self::Bi224),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Bi, value)),
        }
    }
}
impl std::fmt::Display for BismuthIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bi184 => write!(f, "Bi184"),
            Self::Bi185 => write!(f, "Bi185"),
            Self::Bi186 => write!(f, "Bi186"),
            Self::Bi187 => write!(f, "Bi187"),
            Self::Bi188 => write!(f, "Bi188"),
            Self::Bi189 => write!(f, "Bi189"),
            Self::Bi190 => write!(f, "Bi190"),
            Self::Bi191 => write!(f, "Bi191"),
            Self::Bi192 => write!(f, "Bi192"),
            Self::Bi193 => write!(f, "Bi193"),
            Self::Bi194 => write!(f, "Bi194"),
            Self::Bi195 => write!(f, "Bi195"),
            Self::Bi196 => write!(f, "Bi196"),
            Self::Bi197 => write!(f, "Bi197"),
            Self::Bi198 => write!(f, "Bi198"),
            Self::Bi199 => write!(f, "Bi199"),
            Self::Bi200 => write!(f, "Bi200"),
            Self::Bi201 => write!(f, "Bi201"),
            Self::Bi202 => write!(f, "Bi202"),
            Self::Bi203 => write!(f, "Bi203"),
            Self::Bi204 => write!(f, "Bi204"),
            Self::Bi205 => write!(f, "Bi205"),
            Self::Bi206 => write!(f, "Bi206"),
            Self::Bi207 => write!(f, "Bi207"),
            Self::Bi208 => write!(f, "Bi208"),
            Self::Bi209 => write!(f, "Bi209"),
            Self::Bi210 => write!(f, "Bi210"),
            Self::Bi211 => write!(f, "Bi211"),
            Self::Bi212 => write!(f, "Bi212"),
            Self::Bi213 => write!(f, "Bi213"),
            Self::Bi214 => write!(f, "Bi214"),
            Self::Bi215 => write!(f, "Bi215"),
            Self::Bi216 => write!(f, "Bi216"),
            Self::Bi217 => write!(f, "Bi217"),
            Self::Bi218 => write!(f, "Bi218"),
            Self::Bi219 => write!(f, "Bi219"),
            Self::Bi220 => write!(f, "Bi220"),
            Self::Bi221 => write!(f, "Bi221"),
            Self::Bi222 => write!(f, "Bi222"),
            Self::Bi223 => write!(f, "Bi223"),
            Self::Bi224 => write!(f, "Bi224"),
        }
    }
}
