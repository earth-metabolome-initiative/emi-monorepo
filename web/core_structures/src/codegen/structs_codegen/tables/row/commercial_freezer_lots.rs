impl From<crate::CommercialFreezerLot> for super::Row {
    fn from(value: crate::CommercialFreezerLot) -> Self {
        super::Row::CommercialFreezerLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialFreezerLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialFreezerLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
