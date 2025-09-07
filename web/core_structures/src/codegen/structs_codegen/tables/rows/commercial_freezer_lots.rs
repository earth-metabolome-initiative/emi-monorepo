impl From<crate::CommercialFreezerLot> for super::Rows {
    fn from(value: crate::CommercialFreezerLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialFreezerLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialFreezerLot>) -> Self {
        super::Rows::CommercialFreezerLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialFreezerLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialFreezerLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
