impl From<crate::SpatialRefSy> for super::Rows {
    fn from(value: crate::SpatialRefSy) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::SpatialRefSy>> for super::Rows {
    fn from(value: Vec<crate::SpatialRefSy>) -> Self {
        super::Rows::SpatialRefSy(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::SpatialRefSy> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SpatialRefSy(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
