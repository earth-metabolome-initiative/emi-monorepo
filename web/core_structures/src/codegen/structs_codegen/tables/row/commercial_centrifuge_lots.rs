impl From<crate::CommercialCentrifugeLot> for super::Row {
    fn from(value: crate::CommercialCentrifugeLot) -> Self {
        super::Row::CommercialCentrifugeLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialCentrifugeLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCentrifugeLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
