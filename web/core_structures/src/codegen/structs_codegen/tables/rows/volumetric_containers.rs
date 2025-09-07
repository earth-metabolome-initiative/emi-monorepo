impl From<crate::VolumetricContainer> for super::Rows {
    fn from(value: crate::VolumetricContainer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::VolumetricContainer>> for super::Rows {
    fn from(value: Vec<crate::VolumetricContainer>) -> Self {
        super::Rows::VolumetricContainer(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::VolumetricContainer> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::VolumetricContainer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
