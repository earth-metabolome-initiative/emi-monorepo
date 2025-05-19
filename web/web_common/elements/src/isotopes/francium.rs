//! Isotopes of the element Francium
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
/// Isotopes of the element Francium
pub enum FranciumIsotope {
    /// Isotope Fr199 of Francium
    Fr199,
    /// Isotope Fr200 of Francium
    Fr200,
    /// Isotope Fr201 of Francium
    Fr201,
    /// Isotope Fr202 of Francium
    Fr202,
    /// Isotope Fr203 of Francium
    Fr203,
    /// Isotope Fr204 of Francium
    Fr204,
    /// Isotope Fr205 of Francium
    Fr205,
    /// Isotope Fr206 of Francium
    Fr206,
    /// Isotope Fr207 of Francium
    Fr207,
    /// Isotope Fr208 of Francium
    Fr208,
    /// Isotope Fr209 of Francium
    Fr209,
    /// Isotope Fr210 of Francium
    Fr210,
    /// Isotope Fr211 of Francium
    Fr211,
    /// Isotope Fr212 of Francium
    Fr212,
    /// Isotope Fr213 of Francium
    Fr213,
    /// Isotope Fr214 of Francium
    Fr214,
    /// Isotope Fr215 of Francium
    Fr215,
    /// Isotope Fr216 of Francium
    Fr216,
    /// Isotope Fr217 of Francium
    Fr217,
    /// Isotope Fr218 of Francium
    Fr218,
    /// Isotope Fr219 of Francium
    Fr219,
    /// Isotope Fr220 of Francium
    Fr220,
    /// Isotope Fr221 of Francium
    Fr221,
    /// Isotope Fr222 of Francium
    Fr222,
    /// Isotope Fr223 of Francium
    Fr223,
    /// Isotope Fr224 of Francium
    Fr224,
    /// Isotope Fr225 of Francium
    Fr225,
    /// Isotope Fr226 of Francium
    Fr226,
    /// Isotope Fr227 of Francium
    Fr227,
    /// Isotope Fr228 of Francium
    Fr228,
    /// Isotope Fr229 of Francium
    Fr229,
    /// Isotope Fr230 of Francium
    Fr230,
    /// Isotope Fr231 of Francium
    Fr231,
    /// Isotope Fr232 of Francium
    Fr232,
    /// Isotope Fr233 of Francium
    Fr233,
}
impl super::RelativeAtomicMass for FranciumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Fr199 => 199.007259f64,
            Self::Fr200 => 200.006586f64,
            Self::Fr201 => 201.003867f64,
            Self::Fr202 => 202.00332f64,
            Self::Fr203 => 203.0009407f64,
            Self::Fr204 => 204.000652f64,
            Self::Fr205 => 204.9985939f64,
            Self::Fr206 => 205.998666f64,
            Self::Fr207 => 206.996946f64,
            Self::Fr208 => 207.997138f64,
            Self::Fr209 => 208.995955f64,
            Self::Fr210 => 209.996422f64,
            Self::Fr211 => 210.995556f64,
            Self::Fr212 => 211.9962257f64,
            Self::Fr213 => 212.996186f64,
            Self::Fr214 => 213.9989713f64,
            Self::Fr215 => 215.0003418f64,
            Self::Fr216 => 216.0031899f64,
            Self::Fr217 => 217.0046323f64,
            Self::Fr218 => 218.0075787f64,
            Self::Fr219 => 219.0092524f64,
            Self::Fr220 => 220.0123277f64,
            Self::Fr221 => 221.0142552f64,
            Self::Fr222 => 222.017552f64,
            Self::Fr223 => 223.019736f64,
            Self::Fr224 => 224.023398f64,
            Self::Fr225 => 225.025573f64,
            Self::Fr226 => 226.029566f64,
            Self::Fr227 => 227.031869f64,
            Self::Fr228 => 228.035823f64,
            Self::Fr229 => 229.038298f64,
            Self::Fr230 => 230.042416f64,
            Self::Fr231 => 231.045158f64,
            Self::Fr232 => 232.04937f64,
            Self::Fr233 => 233.05264f64,
        }
    }
}
impl super::ElementVariant for FranciumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Fr
    }
}
impl super::MassNumber for FranciumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Fr199 => 199u16,
            Self::Fr200 => 200u16,
            Self::Fr201 => 201u16,
            Self::Fr202 => 202u16,
            Self::Fr203 => 203u16,
            Self::Fr204 => 204u16,
            Self::Fr205 => 205u16,
            Self::Fr206 => 206u16,
            Self::Fr207 => 207u16,
            Self::Fr208 => 208u16,
            Self::Fr209 => 209u16,
            Self::Fr210 => 210u16,
            Self::Fr211 => 211u16,
            Self::Fr212 => 212u16,
            Self::Fr213 => 213u16,
            Self::Fr214 => 214u16,
            Self::Fr215 => 215u16,
            Self::Fr216 => 216u16,
            Self::Fr217 => 217u16,
            Self::Fr218 => 218u16,
            Self::Fr219 => 219u16,
            Self::Fr220 => 220u16,
            Self::Fr221 => 221u16,
            Self::Fr222 => 222u16,
            Self::Fr223 => 223u16,
            Self::Fr224 => 224u16,
            Self::Fr225 => 225u16,
            Self::Fr226 => 226u16,
            Self::Fr227 => 227u16,
            Self::Fr228 => 228u16,
            Self::Fr229 => 229u16,
            Self::Fr230 => 230u16,
            Self::Fr231 => 231u16,
            Self::Fr232 => 232u16,
            Self::Fr233 => 233u16,
        }
    }
}
impl super::IsotopicComposition for FranciumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        None
    }
}
impl super::MostAbundantIsotope for FranciumIsotope {
    fn most_abundant_isotope() -> Self {
        Self::Fr233
    }
}
impl TryFrom<u16> for FranciumIsotope {
    type Error = crate::errors::Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            199u16 => Ok(Self::Fr199),
            200u16 => Ok(Self::Fr200),
            201u16 => Ok(Self::Fr201),
            202u16 => Ok(Self::Fr202),
            203u16 => Ok(Self::Fr203),
            204u16 => Ok(Self::Fr204),
            205u16 => Ok(Self::Fr205),
            206u16 => Ok(Self::Fr206),
            207u16 => Ok(Self::Fr207),
            208u16 => Ok(Self::Fr208),
            209u16 => Ok(Self::Fr209),
            210u16 => Ok(Self::Fr210),
            211u16 => Ok(Self::Fr211),
            212u16 => Ok(Self::Fr212),
            213u16 => Ok(Self::Fr213),
            214u16 => Ok(Self::Fr214),
            215u16 => Ok(Self::Fr215),
            216u16 => Ok(Self::Fr216),
            217u16 => Ok(Self::Fr217),
            218u16 => Ok(Self::Fr218),
            219u16 => Ok(Self::Fr219),
            220u16 => Ok(Self::Fr220),
            221u16 => Ok(Self::Fr221),
            222u16 => Ok(Self::Fr222),
            223u16 => Ok(Self::Fr223),
            224u16 => Ok(Self::Fr224),
            225u16 => Ok(Self::Fr225),
            226u16 => Ok(Self::Fr226),
            227u16 => Ok(Self::Fr227),
            228u16 => Ok(Self::Fr228),
            229u16 => Ok(Self::Fr229),
            230u16 => Ok(Self::Fr230),
            231u16 => Ok(Self::Fr231),
            232u16 => Ok(Self::Fr232),
            233u16 => Ok(Self::Fr233),
            _ => Err(crate::errors::Error::Isotope(crate::Element::Fr, value)),
        }
    }
}
impl std::fmt::Display for FranciumIsotope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fr199 => write!(f, "Fr199"),
            Self::Fr200 => write!(f, "Fr200"),
            Self::Fr201 => write!(f, "Fr201"),
            Self::Fr202 => write!(f, "Fr202"),
            Self::Fr203 => write!(f, "Fr203"),
            Self::Fr204 => write!(f, "Fr204"),
            Self::Fr205 => write!(f, "Fr205"),
            Self::Fr206 => write!(f, "Fr206"),
            Self::Fr207 => write!(f, "Fr207"),
            Self::Fr208 => write!(f, "Fr208"),
            Self::Fr209 => write!(f, "Fr209"),
            Self::Fr210 => write!(f, "Fr210"),
            Self::Fr211 => write!(f, "Fr211"),
            Self::Fr212 => write!(f, "Fr212"),
            Self::Fr213 => write!(f, "Fr213"),
            Self::Fr214 => write!(f, "Fr214"),
            Self::Fr215 => write!(f, "Fr215"),
            Self::Fr216 => write!(f, "Fr216"),
            Self::Fr217 => write!(f, "Fr217"),
            Self::Fr218 => write!(f, "Fr218"),
            Self::Fr219 => write!(f, "Fr219"),
            Self::Fr220 => write!(f, "Fr220"),
            Self::Fr221 => write!(f, "Fr221"),
            Self::Fr222 => write!(f, "Fr222"),
            Self::Fr223 => write!(f, "Fr223"),
            Self::Fr224 => write!(f, "Fr224"),
            Self::Fr225 => write!(f, "Fr225"),
            Self::Fr226 => write!(f, "Fr226"),
            Self::Fr227 => write!(f, "Fr227"),
            Self::Fr228 => write!(f, "Fr228"),
            Self::Fr229 => write!(f, "Fr229"),
            Self::Fr230 => write!(f, "Fr230"),
            Self::Fr231 => write!(f, "Fr231"),
            Self::Fr232 => write!(f, "Fr232"),
            Self::Fr233 => write!(f, "Fr233"),
        }
    }
}
