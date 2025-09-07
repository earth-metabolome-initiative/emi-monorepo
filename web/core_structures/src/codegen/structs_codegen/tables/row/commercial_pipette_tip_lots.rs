impl From<crate::CommercialPipetteTipLot> for super::Row {
    fn from(value: crate::CommercialPipetteTipLot) -> Self {
        super::Row::CommercialPipetteTipLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPipetteTipLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteTipLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
