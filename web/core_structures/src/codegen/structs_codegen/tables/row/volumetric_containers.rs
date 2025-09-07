impl From<crate::VolumetricContainer> for super::Row {
    fn from(value: crate::VolumetricContainer) -> Self {
        super::Row::VolumetricContainer(value)
    }
}
impl TryFrom<super::Row> for crate::VolumetricContainer {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::VolumetricContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
