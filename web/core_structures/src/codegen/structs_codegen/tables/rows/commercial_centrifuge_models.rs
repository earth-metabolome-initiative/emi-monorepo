impl From<crate::CommercialCentrifugeModel> for super::Rows {
    fn from(value: crate::CommercialCentrifugeModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialCentrifugeModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialCentrifugeModel>) -> Self {
        super::Rows::CommercialCentrifugeModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialCentrifugeModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialCentrifugeModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
