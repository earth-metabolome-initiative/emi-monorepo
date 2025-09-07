impl From<crate::PackagingModel> for super::Rows {
    fn from(value: crate::PackagingModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::PackagingModel>> for super::Rows {
    fn from(value: Vec<crate::PackagingModel>) -> Self {
        super::Rows::PackagingModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::PackagingModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::PackagingModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
