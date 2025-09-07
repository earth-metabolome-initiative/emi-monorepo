impl From<crate::CommercialBeadLot> for super::Rows {
    fn from(value: crate::CommercialBeadLot) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialBeadLot>> for super::Rows {
    fn from(value: Vec<crate::CommercialBeadLot>) -> Self {
        super::Rows::CommercialBeadLot(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialBeadLot> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialBeadLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
