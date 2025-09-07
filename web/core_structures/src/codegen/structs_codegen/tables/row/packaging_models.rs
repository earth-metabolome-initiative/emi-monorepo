impl From<crate::PackagingModel> for super::Row {
    fn from(value: crate::PackagingModel) -> Self {
        super::Row::PackagingModel(value)
    }
}
impl TryFrom<super::Row> for crate::PackagingModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::PackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
