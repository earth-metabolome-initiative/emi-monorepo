//! Isotopes of the element Polonium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Polonium
pub enum PoloniumIsotope {
    /// Isotope Po186 of Polonium
    Po186,
    /// Isotope Po187 of Polonium
    Po187,
    /// Isotope Po188 of Polonium
    Po188,
    /// Isotope Po189 of Polonium
    Po189,
    /// Isotope Po190 of Polonium
    Po190,
    /// Isotope Po191 of Polonium
    Po191,
    /// Isotope Po192 of Polonium
    Po192,
    /// Isotope Po193 of Polonium
    Po193,
    /// Isotope Po194 of Polonium
    Po194,
    /// Isotope Po195 of Polonium
    Po195,
    /// Isotope Po196 of Polonium
    Po196,
    /// Isotope Po197 of Polonium
    Po197,
    /// Isotope Po198 of Polonium
    Po198,
    /// Isotope Po199 of Polonium
    Po199,
    /// Isotope Po200 of Polonium
    Po200,
    /// Isotope Po201 of Polonium
    Po201,
    /// Isotope Po202 of Polonium
    Po202,
    /// Isotope Po203 of Polonium
    Po203,
    /// Isotope Po204 of Polonium
    Po204,
    /// Isotope Po205 of Polonium
    Po205,
    /// Isotope Po206 of Polonium
    Po206,
    /// Isotope Po207 of Polonium
    Po207,
    /// Isotope Po208 of Polonium
    Po208,
    /// Isotope Po209 of Polonium
    Po209,
    /// Isotope Po210 of Polonium
    Po210,
    /// Isotope Po211 of Polonium
    Po211,
    /// Isotope Po212 of Polonium
    Po212,
    /// Isotope Po213 of Polonium
    Po213,
    /// Isotope Po214 of Polonium
    Po214,
    /// Isotope Po215 of Polonium
    Po215,
    /// Isotope Po216 of Polonium
    Po216,
    /// Isotope Po217 of Polonium
    Po217,
    /// Isotope Po218 of Polonium
    Po218,
    /// Isotope Po219 of Polonium
    Po219,
    /// Isotope Po220 of Polonium
    Po220,
    /// Isotope Po221 of Polonium
    Po221,
    /// Isotope Po222 of Polonium
    Po222,
    /// Isotope Po223 of Polonium
    Po223,
    /// Isotope Po224 of Polonium
    Po224,
    /// Isotope Po225 of Polonium
    Po225,
    /// Isotope Po226 of Polonium
    Po226,
    /// Isotope Po227 of Polonium
    Po227,
}
impl super::RelativeAtomicMass for PoloniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Po186 => 186.004393f64,
            Self::Po187 => 187.003041f64,
            Self::Po188 => 187.999416f64,
            Self::Po189 => 188.998473f64,
            Self::Po190 => 189.995101f64,
            Self::Po191 => 190.9945585f64,
            Self::Po192 => 191.991336f64,
            Self::Po193 => 192.991026f64,
            Self::Po194 => 193.988186f64,
            Self::Po195 => 194.988126f64,
            Self::Po196 => 195.985526f64,
            Self::Po197 => 196.98566f64,
            Self::Po198 => 197.983389f64,
            Self::Po199 => 198.983667f64,
            Self::Po200 => 199.981799f64,
            Self::Po201 => 200.9822598f64,
            Self::Po202 => 201.980758f64,
            Self::Po203 => 202.9814161f64,
            Self::Po204 => 203.98031f64,
            Self::Po205 => 204.981203f64,
            Self::Po206 => 205.980474f64,
            Self::Po207 => 206.9815938f64,
            Self::Po208 => 207.9812461f64,
            Self::Po209 => 208.9824308f64,
            Self::Po210 => 209.9828741f64,
            Self::Po211 => 210.9866536f64,
            Self::Po212 => 211.9888684f64,
            Self::Po213 => 212.9928576f64,
            Self::Po214 => 213.9952017f64,
            Self::Po215 => 214.9994201f64,
            Self::Po216 => 216.0019152f64,
            Self::Po217 => 217.0063182f64,
            Self::Po218 => 218.0089735f64,
            Self::Po219 => 219.013614f64,
            Self::Po220 => 220.016386f64,
            Self::Po221 => 221.021228f64,
            Self::Po222 => 222.02414f64,
            Self::Po223 => 223.02907f64,
            Self::Po224 => 224.03211f64,
            Self::Po225 => 225.03707f64,
            Self::Po226 => 226.04031f64,
            Self::Po227 => 227.04539f64,
        }
    }
}
impl super::ElementVariant for PoloniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Po
    }
}
impl super::MassNumber for PoloniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Po186 => 186u16,
            Self::Po187 => 187u16,
            Self::Po188 => 188u16,
            Self::Po189 => 189u16,
            Self::Po190 => 190u16,
            Self::Po191 => 191u16,
            Self::Po192 => 192u16,
            Self::Po193 => 193u16,
            Self::Po194 => 194u16,
            Self::Po195 => 195u16,
            Self::Po196 => 196u16,
            Self::Po197 => 197u16,
            Self::Po198 => 198u16,
            Self::Po199 => 199u16,
            Self::Po200 => 200u16,
            Self::Po201 => 201u16,
            Self::Po202 => 202u16,
            Self::Po203 => 203u16,
            Self::Po204 => 204u16,
            Self::Po205 => 205u16,
            Self::Po206 => 206u16,
            Self::Po207 => 207u16,
            Self::Po208 => 208u16,
            Self::Po209 => 209u16,
            Self::Po210 => 210u16,
            Self::Po211 => 211u16,
            Self::Po212 => 212u16,
            Self::Po213 => 213u16,
            Self::Po214 => 214u16,
            Self::Po215 => 215u16,
            Self::Po216 => 216u16,
            Self::Po217 => 217u16,
            Self::Po218 => 218u16,
            Self::Po219 => 219u16,
            Self::Po220 => 220u16,
            Self::Po221 => 221u16,
            Self::Po222 => 222u16,
            Self::Po223 => 223u16,
            Self::Po224 => 224u16,
            Self::Po225 => 225u16,
            Self::Po226 => 226u16,
            Self::Po227 => 227u16,
        }
    }
}
impl super::IsotopicComposition for PoloniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for PoloniumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Po227
    }
}
impl From<PoloniumIsotope> for crate::Isotope {
    fn from(isotope: PoloniumIsotope) -> Self {
        crate::Isotope::Po(isotope)
    }
}
impl From<PoloniumIsotope> for crate::Element {
    fn from(_isotope: PoloniumIsotope) -> Self {
        crate::Element::Po
    }
}
impl TryFrom<u16> for PoloniumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            186u16 => Ok(Self::Po186),
            187u16 => Ok(Self::Po187),
            188u16 => Ok(Self::Po188),
            189u16 => Ok(Self::Po189),
            190u16 => Ok(Self::Po190),
            191u16 => Ok(Self::Po191),
            192u16 => Ok(Self::Po192),
            193u16 => Ok(Self::Po193),
            194u16 => Ok(Self::Po194),
            195u16 => Ok(Self::Po195),
            196u16 => Ok(Self::Po196),
            197u16 => Ok(Self::Po197),
            198u16 => Ok(Self::Po198),
            199u16 => Ok(Self::Po199),
            200u16 => Ok(Self::Po200),
            201u16 => Ok(Self::Po201),
            202u16 => Ok(Self::Po202),
            203u16 => Ok(Self::Po203),
            204u16 => Ok(Self::Po204),
            205u16 => Ok(Self::Po205),
            206u16 => Ok(Self::Po206),
            207u16 => Ok(Self::Po207),
            208u16 => Ok(Self::Po208),
            209u16 => Ok(Self::Po209),
            210u16 => Ok(Self::Po210),
            211u16 => Ok(Self::Po211),
            212u16 => Ok(Self::Po212),
            213u16 => Ok(Self::Po213),
            214u16 => Ok(Self::Po214),
            215u16 => Ok(Self::Po215),
            216u16 => Ok(Self::Po216),
            217u16 => Ok(Self::Po217),
            218u16 => Ok(Self::Po218),
            219u16 => Ok(Self::Po219),
            220u16 => Ok(Self::Po220),
            221u16 => Ok(Self::Po221),
            222u16 => Ok(Self::Po222),
            223u16 => Ok(Self::Po223),
            224u16 => Ok(Self::Po224),
            225u16 => Ok(Self::Po225),
            226u16 => Ok(Self::Po226),
            227u16 => Ok(Self::Po227),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Po, value)),
        }
    }
}
impl std::fmt::Display for PoloniumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Po186 => write!(f, "Po186"),
            Self::Po187 => write!(f, "Po187"),
            Self::Po188 => write!(f, "Po188"),
            Self::Po189 => write!(f, "Po189"),
            Self::Po190 => write!(f, "Po190"),
            Self::Po191 => write!(f, "Po191"),
            Self::Po192 => write!(f, "Po192"),
            Self::Po193 => write!(f, "Po193"),
            Self::Po194 => write!(f, "Po194"),
            Self::Po195 => write!(f, "Po195"),
            Self::Po196 => write!(f, "Po196"),
            Self::Po197 => write!(f, "Po197"),
            Self::Po198 => write!(f, "Po198"),
            Self::Po199 => write!(f, "Po199"),
            Self::Po200 => write!(f, "Po200"),
            Self::Po201 => write!(f, "Po201"),
            Self::Po202 => write!(f, "Po202"),
            Self::Po203 => write!(f, "Po203"),
            Self::Po204 => write!(f, "Po204"),
            Self::Po205 => write!(f, "Po205"),
            Self::Po206 => write!(f, "Po206"),
            Self::Po207 => write!(f, "Po207"),
            Self::Po208 => write!(f, "Po208"),
            Self::Po209 => write!(f, "Po209"),
            Self::Po210 => write!(f, "Po210"),
            Self::Po211 => write!(f, "Po211"),
            Self::Po212 => write!(f, "Po212"),
            Self::Po213 => write!(f, "Po213"),
            Self::Po214 => write!(f, "Po214"),
            Self::Po215 => write!(f, "Po215"),
            Self::Po216 => write!(f, "Po216"),
            Self::Po217 => write!(f, "Po217"),
            Self::Po218 => write!(f, "Po218"),
            Self::Po219 => write!(f, "Po219"),
            Self::Po220 => write!(f, "Po220"),
            Self::Po221 => write!(f, "Po221"),
            Self::Po222 => write!(f, "Po222"),
            Self::Po223 => write!(f, "Po223"),
            Self::Po224 => write!(f, "Po224"),
            Self::Po225 => write!(f, "Po225"),
            Self::Po226 => write!(f, "Po226"),
            Self::Po227 => write!(f, "Po227"),
        }
    }
}
