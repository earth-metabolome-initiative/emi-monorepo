#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum :: EnumIter)]
pub enum AluminiumIsotope {
    Al21,
    Al22,
    Al23,
    Al24,
    Al25,
    Al26,
    Al27,
    Al28,
    Al29,
    Al30,
    Al31,
    Al32,
    Al33,
    Al34,
    Al35,
    Al36,
    Al37,
    Al38,
    Al39,
    Al40,
    Al41,
    Al42,
    Al43,
}
impl super::RelativeAtomicMass for AluminiumIsotope {
    fn relative_atomic_mass(&self) -> f64 {
        match self {
            Self::Al21 => 21.02897f64,
            Self::Al22 => 22.01954f64,
            Self::Al23 => 23.00724435f64,
            Self::Al24 => 23.9999489f64,
            Self::Al25 => 24.9904281f64,
            Self::Al26 => 25.986891904f64,
            Self::Al27 => 26.98153853f64,
            Self::Al28 => 27.98191021f64,
            Self::Al29 => 28.9804565f64,
            Self::Al30 => 29.98296f64,
            Self::Al31 => 30.983945f64,
            Self::Al32 => 31.988085f64,
            Self::Al33 => 32.990909f64,
            Self::Al34 => 33.996705f64,
            Self::Al35 => 34.999764f64,
            Self::Al36 => 36.00639f64,
            Self::Al37 => 37.01053f64,
            Self::Al38 => 38.0174f64,
            Self::Al39 => 39.02254f64,
            Self::Al40 => 40.03003f64,
            Self::Al41 => 41.03638f64,
            Self::Al42 => 42.04384f64,
            Self::Al43 => 43.05147f64,
        }
    }
}
impl super::ElementVariant for AluminiumIsotope {
    fn element(&self) -> crate::Element {
        crate::Element::Al
    }
}
impl super::MassNumber for AluminiumIsotope {
    fn mass_number(&self) -> u16 {
        match self {
            Self::Al21 => 21u16,
            Self::Al22 => 22u16,
            Self::Al23 => 23u16,
            Self::Al24 => 24u16,
            Self::Al25 => 25u16,
            Self::Al26 => 26u16,
            Self::Al27 => 27u16,
            Self::Al28 => 28u16,
            Self::Al29 => 29u16,
            Self::Al30 => 30u16,
            Self::Al31 => 31u16,
            Self::Al32 => 32u16,
            Self::Al33 => 33u16,
            Self::Al34 => 34u16,
            Self::Al35 => 35u16,
            Self::Al36 => 36u16,
            Self::Al37 => 37u16,
            Self::Al38 => 38u16,
            Self::Al39 => 39u16,
            Self::Al40 => 40u16,
            Self::Al41 => 41u16,
            Self::Al42 => 42u16,
            Self::Al43 => 43u16,
        }
    }
}
