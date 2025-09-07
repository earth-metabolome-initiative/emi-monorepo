impl From<crate::CommercialPipetteLot> for super::Row {
    fn from(value: crate::CommercialPipetteLot) -> Self {
        super::Row::CommercialPipetteLot(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPipetteLot {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPipetteLot(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
