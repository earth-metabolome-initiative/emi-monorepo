#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum UraniumIsotope {
    U217,
    U218,
    U219,
    U220,
    U221,
    U222,
    U223,
    U224,
    U225,
    U226,
    U227,
    U228,
    U229,
    U230,
    U231,
    U232,
    U233,
    U234,
    U235,
    U236,
    U237,
    U238,
    U239,
    U240,
    U241,
    U242,
    U243,
}
impl super::RelativeAtomicMass for UraniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::U217 => 217.02466f64,
            Self::U218 => 218.023523f64,
            Self::U219 => 219.024999f64,
            Self::U220 => 220.02462f64,
            Self::U221 => 221.02628f64,
            Self::U222 => 222.026f64,
            Self::U223 => 223.027739f64,
            Self::U224 => 224.027605f64,
            Self::U225 => 225.029391f64,
            Self::U226 => 226.029339f64,
            Self::U227 => 227.031157f64,
            Self::U228 => 228.031371f64,
            Self::U229 => 229.0335063f64,
            Self::U230 => 230.0339401f64,
            Self::U231 => 231.0362939f64,
            Self::U232 => 232.0371563f64,
            Self::U233 => 233.0396355f64,
            Self::U234 => 234.0409523f64,
            Self::U235 => 235.0439301f64,
            Self::U236 => 236.0455682f64,
            Self::U237 => 237.0487304f64,
            Self::U238 => 238.0507884f64,
            Self::U239 => 239.0542935f64,
            Self::U240 => 240.0565934f64,
            Self::U241 => 241.06033f64,
            Self::U242 => 242.06293f64,
            Self::U243 => 243.06699f64,
        }
    }
}
impl super::ElementVariant for UraniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::U
    }
}
impl super::MassNumber for UraniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::U217 => 217u16,
            Self::U218 => 218u16,
            Self::U219 => 219u16,
            Self::U220 => 220u16,
            Self::U221 => 221u16,
            Self::U222 => 222u16,
            Self::U223 => 223u16,
            Self::U224 => 224u16,
            Self::U225 => 225u16,
            Self::U226 => 226u16,
            Self::U227 => 227u16,
            Self::U228 => 228u16,
            Self::U229 => 229u16,
            Self::U230 => 230u16,
            Self::U231 => 231u16,
            Self::U232 => 232u16,
            Self::U233 => 233u16,
            Self::U234 => 234u16,
            Self::U235 => 235u16,
            Self::U236 => 236u16,
            Self::U237 => 237u16,
            Self::U238 => 238u16,
            Self::U239 => 239u16,
            Self::U240 => 240u16,
            Self::U241 => 241u16,
            Self::U242 => 242u16,
            Self::U243 => 243u16,
        }
    }
}
impl super::IsotopicComposition for UraniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::U217 => None,
            Self::U218 => None,
            Self::U219 => None,
            Self::U220 => None,
            Self::U221 => None,
            Self::U222 => None,
            Self::U223 => None,
            Self::U224 => None,
            Self::U225 => None,
            Self::U226 => None,
            Self::U227 => None,
            Self::U228 => None,
            Self::U229 => None,
            Self::U230 => None,
            Self::U231 => None,
            Self::U232 => None,
            Self::U233 => None,
            Self::U234 => Some(0.000054f64),
            Self::U235 => Some(0.007204f64),
            Self::U236 => None,
            Self::U237 => None,
            Self::U238 => Some(0.992742f64),
            Self::U239 => None,
            Self::U240 => None,
            Self::U241 => None,
            Self::U242 => None,
            Self::U243 => None,
        }
    }
}
impl super::MostCommonIsotope for UraniumIsotope {
    fn most_common_isotope() -> Self {
        Self::U238
    }
}
