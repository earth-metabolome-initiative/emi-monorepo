impl From<crate::CommercialFreezeDryerModel> for super::Row {
    fn from(value: crate::CommercialFreezeDryerModel) -> Self {
        super::Row::CommercialFreezeDryerModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialFreezeDryerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialFreezeDryerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
