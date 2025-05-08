#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
pub enum VanadiumIsotope {
    V40,
    V41,
    V42,
    V43,
    V44,
    V45,
    V46,
    V47,
    V48,
    V49,
    V50,
    V51,
    V52,
    V53,
    V54,
    V55,
    V56,
    V57,
    V58,
    V59,
    V60,
    V61,
    V62,
    V63,
    V64,
    V65,
    V66,
}
impl super::RelativeAtomicMass for VanadiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::V40 => 40.01276f64,
            Self::V41 => 41.00021f64,
            Self::V42 => 41.99182f64,
            Self::V43 => 42.980766f64,
            Self::V44 => 43.97411f64,
            Self::V45 => 44.9657748f64,
            Self::V46 => 45.96019878f64,
            Self::V47 => 46.95490491f64,
            Self::V48 => 47.9522522f64,
            Self::V49 => 48.9485118f64,
            Self::V50 => 49.94715601f64,
            Self::V51 => 50.94395704f64,
            Self::V52 => 51.94477301f64,
            Self::V53 => 52.9443367f64,
            Self::V54 => 53.946439f64,
            Self::V55 => 54.94724f64,
            Self::V56 => 55.95048f64,
            Self::V57 => 56.95252f64,
            Self::V58 => 57.95672f64,
            Self::V59 => 58.95939f64,
            Self::V60 => 59.96431f64,
            Self::V61 => 60.96725f64,
            Self::V62 => 61.97265f64,
            Self::V63 => 62.97639f64,
            Self::V64 => 63.98264f64,
            Self::V65 => 64.9875f64,
            Self::V66 => 65.99398f64,
        }
    }
}
impl super::ElementVariant for VanadiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::V
    }
}
impl super::MassNumber for VanadiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::V40 => 40u16,
            Self::V41 => 41u16,
            Self::V42 => 42u16,
            Self::V43 => 43u16,
            Self::V44 => 44u16,
            Self::V45 => 45u16,
            Self::V46 => 46u16,
            Self::V47 => 47u16,
            Self::V48 => 48u16,
            Self::V49 => 49u16,
            Self::V50 => 50u16,
            Self::V51 => 51u16,
            Self::V52 => 52u16,
            Self::V53 => 53u16,
            Self::V54 => 54u16,
            Self::V55 => 55u16,
            Self::V56 => 56u16,
            Self::V57 => 57u16,
            Self::V58 => 58u16,
            Self::V59 => 59u16,
            Self::V60 => 60u16,
            Self::V61 => 61u16,
            Self::V62 => 62u16,
            Self::V63 => 63u16,
            Self::V64 => 64u16,
            Self::V65 => 65u16,
            Self::V66 => 66u16,
        }
    }
}
impl super::IsotopicComposition for VanadiumIsotope {
    fn isotopic_composition(&self) -> Option<f64> {
        match self {
            Self::V40 => None,
            Self::V41 => None,
            Self::V42 => None,
            Self::V43 => None,
            Self::V44 => None,
            Self::V45 => None,
            Self::V46 => None,
            Self::V47 => None,
            Self::V48 => None,
            Self::V49 => None,
            Self::V50 => Some(0.0025f64),
            Self::V51 => Some(0.9975f64),
            Self::V52 => None,
            Self::V53 => None,
            Self::V54 => None,
            Self::V55 => None,
            Self::V56 => None,
            Self::V57 => None,
            Self::V58 => None,
            Self::V59 => None,
            Self::V60 => None,
            Self::V61 => None,
            Self::V62 => None,
            Self::V63 => None,
            Self::V64 => None,
            Self::V65 => None,
            Self::V66 => None,
        }
    }
}
impl super::MostCommonIsotope for VanadiumIsotope {
    fn most_common_isotope() -> Self {
        Self::V51
    }
}
