#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum ActiniumIsotope {
    Ac206,
    Ac207,
    Ac208,
    Ac209,
    Ac210,
    Ac211,
    Ac212,
    Ac213,
    Ac214,
    Ac215,
    Ac216,
    Ac217,
    Ac218,
    Ac219,
    Ac220,
    Ac221,
    Ac222,
    Ac223,
    Ac224,
    Ac225,
    Ac226,
    Ac227,
    Ac228,
    Ac229,
    Ac230,
    Ac231,
    Ac232,
    Ac233,
    Ac234,
    Ac235,
    Ac236,
    Ac237,
}
impl super::RelativeAtomicMass for ActiniumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Ac206 => 206.014452f64,
            Self::Ac207 => 207.011966f64,
            Self::Ac208 => 208.01155f64,
            Self::Ac209 => 209.009495f64,
            Self::Ac210 => 210.009436f64,
            Self::Ac211 => 211.007732f64,
            Self::Ac212 => 212.007813f64,
            Self::Ac213 => 213.006609f64,
            Self::Ac214 => 214.006918f64,
            Self::Ac215 => 215.006475f64,
            Self::Ac216 => 216.008743f64,
            Self::Ac217 => 217.009344f64,
            Self::Ac218 => 218.011642f64,
            Self::Ac219 => 219.012421f64,
            Self::Ac220 => 220.0147549f64,
            Self::Ac221 => 221.015592f64,
            Self::Ac222 => 222.0178442f64,
            Self::Ac223 => 223.0191377f64,
            Self::Ac224 => 224.0217232f64,
            Self::Ac225 => 225.02323f64,
            Self::Ac226 => 226.0260984f64,
            Self::Ac227 => 227.0277523f64,
            Self::Ac228 => 228.0310215f64,
            Self::Ac229 => 229.032956f64,
            Self::Ac230 => 230.036327f64,
            Self::Ac231 => 231.038393f64,
            Self::Ac232 => 232.042034f64,
            Self::Ac233 => 233.044346f64,
            Self::Ac234 => 234.048139f64,
            Self::Ac235 => 235.05084f64,
            Self::Ac236 => 236.054988f64,
            Self::Ac237 => 237.05827f64,
        }
    }
}
impl super::ElementVariant for ActiniumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Ac
    }
}
impl super::MassNumber for ActiniumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Ac206 => 206u16,
            Self::Ac207 => 207u16,
            Self::Ac208 => 208u16,
            Self::Ac209 => 209u16,
            Self::Ac210 => 210u16,
            Self::Ac211 => 211u16,
            Self::Ac212 => 212u16,
            Self::Ac213 => 213u16,
            Self::Ac214 => 214u16,
            Self::Ac215 => 215u16,
            Self::Ac216 => 216u16,
            Self::Ac217 => 217u16,
            Self::Ac218 => 218u16,
            Self::Ac219 => 219u16,
            Self::Ac220 => 220u16,
            Self::Ac221 => 221u16,
            Self::Ac222 => 222u16,
            Self::Ac223 => 223u16,
            Self::Ac224 => 224u16,
            Self::Ac225 => 225u16,
            Self::Ac226 => 226u16,
            Self::Ac227 => 227u16,
            Self::Ac228 => 228u16,
            Self::Ac229 => 229u16,
            Self::Ac230 => 230u16,
            Self::Ac231 => 231u16,
            Self::Ac232 => 232u16,
            Self::Ac233 => 233u16,
            Self::Ac234 => 234u16,
            Self::Ac235 => 235u16,
            Self::Ac236 => 236u16,
            Self::Ac237 => 237u16,
        }
    }
}
impl super::IsotopicComposition for ActiniumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::Ac206 => None,
            Self::Ac207 => None,
            Self::Ac208 => None,
            Self::Ac209 => None,
            Self::Ac210 => None,
            Self::Ac211 => None,
            Self::Ac212 => None,
            Self::Ac213 => None,
            Self::Ac214 => None,
            Self::Ac215 => None,
            Self::Ac216 => None,
            Self::Ac217 => None,
            Self::Ac218 => None,
            Self::Ac219 => None,
            Self::Ac220 => None,
            Self::Ac221 => None,
            Self::Ac222 => None,
            Self::Ac223 => None,
            Self::Ac224 => None,
            Self::Ac225 => None,
            Self::Ac226 => None,
            Self::Ac227 => None,
            Self::Ac228 => None,
            Self::Ac229 => None,
            Self::Ac230 => None,
            Self::Ac231 => None,
            Self::Ac232 => None,
            Self::Ac233 => None,
            Self::Ac234 => None,
            Self::Ac235 => None,
            Self::Ac236 => None,
            Self::Ac237 => None,
        }
    }
}
impl super::MostCommonIsotope for ActiniumIsotope {
    fn most_common_isotope() -> Self {
        Self::Ac237
    }
}
