impl From<crate::CommercialBeadLot> for super::Row {
    fn from(value: crate::CommercialBeadLot) -> Self {
        super::Row::CommercialBeadLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialBeadLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialBeadLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
