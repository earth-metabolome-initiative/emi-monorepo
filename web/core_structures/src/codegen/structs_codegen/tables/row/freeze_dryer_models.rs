impl From<crate::FreezeDryerModel> for super::Row {
    fn from(value: crate::FreezeDryerModel) -> Self {
        super::Row::FreezeDryerModel(value)
    }
}
impl TryFrom<super::Row> for crate::FreezeDryerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::FreezeDryerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
