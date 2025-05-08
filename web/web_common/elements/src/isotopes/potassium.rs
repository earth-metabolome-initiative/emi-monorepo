#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum PotassiumIsotope {
    K32,
    K33,
    K34,
    K35,
    K36,
    K37,
    K38,
    K39,
    K40,
    K41,
    K42,
    K43,
    K44,
    K45,
    K46,
    K47,
    K48,
    K49,
    K50,
    K51,
    K52,
    K53,
    K54,
    K55,
    K56,
}
impl super::RelativeAtomicMass for PotassiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::K32 => 32.02265f64,
            Self::K33 => 33.00756f64,
            Self::K34 => 33.99869f64,
            Self::K35 => 34.98800541f64,
            Self::K36 => 35.98130201f64,
            Self::K37 => 36.97337589f64,
            Self::K38 => 37.96908112f64,
            Self::K39 => 38.9637064864f64,
            Self::K40 => 39.963998166f64,
            Self::K41 => 40.9618252579f64,
            Self::K42 => 41.96240231f64,
            Self::K43 => 42.9607347f64,
            Self::K44 => 43.96158699f64,
            Self::K45 => 44.96069149f64,
            Self::K46 => 45.96198159f64,
            Self::K47 => 46.9616616f64,
            Self::K48 => 47.96534119f64,
            Self::K49 => 48.96821075f64,
            Self::K50 => 49.97238f64,
            Self::K51 => 50.975828f64,
            Self::K52 => 51.98224f64,
            Self::K53 => 52.98746f64,
            Self::K54 => 53.99463f64,
            Self::K55 => 55.00076f64,
            Self::K56 => 56.00851f64,
        }
    }
}
impl super::ElementVariant for PotassiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::K
    }
}
impl super::MassNumber for PotassiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::K32 => 32u16,
            Self::K33 => 33u16,
            Self::K34 => 34u16,
            Self::K35 => 35u16,
            Self::K36 => 36u16,
            Self::K37 => 37u16,
            Self::K38 => 38u16,
            Self::K39 => 39u16,
            Self::K40 => 40u16,
            Self::K41 => 41u16,
            Self::K42 => 42u16,
            Self::K43 => 43u16,
            Self::K44 => 44u16,
            Self::K45 => 45u16,
            Self::K46 => 46u16,
            Self::K47 => 47u16,
            Self::K48 => 48u16,
            Self::K49 => 49u16,
            Self::K50 => 50u16,
            Self::K51 => 51u16,
            Self::K52 => 52u16,
            Self::K53 => 53u16,
            Self::K54 => 54u16,
            Self::K55 => 55u16,
            Self::K56 => 56u16,
        }
    }
}
impl super::IsotopicComposition for PotassiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::K32 => None,
            Self::K33 => None,
            Self::K34 => None,
            Self::K35 => None,
            Self::K36 => None,
            Self::K37 => None,
            Self::K38 => None,
            Self::K39 => Some(0.932581f64),
            Self::K40 => Some(0.000117f64),
            Self::K41 => Some(0.067302f64),
            Self::K42 => None,
            Self::K43 => None,
            Self::K44 => None,
            Self::K45 => None,
            Self::K46 => None,
            Self::K47 => None,
            Self::K48 => None,
            Self::K49 => None,
            Self::K50 => None,
            Self::K51 => None,
            Self::K52 => None,
            Self::K53 => None,
            Self::K54 => None,
            Self::K55 => None,
            Self::K56 => None,
        }
    }
}
impl super::MostCommonIsotope for PotassiumIsotope {
    fn most_common_isotope() -> Self {
        Self::K39
    }
}
