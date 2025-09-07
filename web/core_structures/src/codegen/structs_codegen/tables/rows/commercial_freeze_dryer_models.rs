impl From<crate::CommercialFreezeDryerModel> for super::Rows {
    fn from(value: crate::CommercialFreezeDryerModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::CommercialFreezeDryerModel>> for super::Rows {
    fn from(value: Vec<crate::CommercialFreezeDryerModel>) -> Self {
        super::Rows::CommercialFreezeDryerModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::CommercialFreezeDryerModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::CommercialFreezeDryerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
