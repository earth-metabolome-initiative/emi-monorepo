impl From<crate::ContainerModel> for super::Row {
    fn from(value: crate::ContainerModel) -> Self {
        super::Row::ContainerModel(value)
    }
}
impl TryFrom<super::Row> for crate::ContainerModel {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::ContainerModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
