impl From<crate::CommercialProductLot> for super::Rows {
    fn from(value: crate::CommercialProductLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialProductLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialProductLot>) -> Self {
        super::Rows::CommercialProductLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialProductLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialProductLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
