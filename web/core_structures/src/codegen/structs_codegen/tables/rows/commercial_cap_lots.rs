impl From<crate::CommercialCapLot> for super::Rows {
    fn from(value: crate::CommercialCapLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCapLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialCapLot>) -> Self {
        super::Rows::CommercialCapLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCapLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCapLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
