impl From<crate::ContainerModel> for super::Rows {
    fn from(value: crate::ContainerModel) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::ContainerModel>> for super::Rows {
    fn from(value: Vec<crate::ContainerModel>) -> Self {
        super::Rows::ContainerModel(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::ContainerModel> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::ContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
