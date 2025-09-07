impl From<crate::CommercialFreezeDryerLot> for super::Row {
    fn from(value: crate::CommercialFreezeDryerLot) -> Self {
        super::Row::CommercialFreezeDryerLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialFreezeDryerLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialFreezeDryerLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
