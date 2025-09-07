impl From<crate::CommercialPackagingModel> for super::Row {
    fn from(value: crate::CommercialPackagingModel) -> Self {
        super::Row::CommercialPackagingModel(value)
    }
}
impl TryFrom<super::Row> for crate::CommercialPackagingModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::CommercialPackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
