impl From<crate::CommercialProductLot> for super::Row {
    fn from(value: crate::CommercialProductLot) -> Self {
        super::Row::CommercialProductLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialProductLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialProductLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
