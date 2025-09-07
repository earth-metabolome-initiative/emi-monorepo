impl From<crate::SpatialRefSy> for super::Row {
    fn from(value: crate::SpatialRefSy) -> Self {
        super::Row::SpatialRefSy(value)
    }
}
impl TryFrom<super::Row> for crate::SpatialRefSy {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SpatialRefSy(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
