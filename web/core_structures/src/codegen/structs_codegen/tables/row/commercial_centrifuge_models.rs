impl From<crate::CommercialCentrifugeModel> for super::Row {
    fn from(value: crate::CommercialCentrifugeModel) -> Self {
        super::Row::CommercialCentrifugeModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialCentrifugeModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialCentrifugeModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
