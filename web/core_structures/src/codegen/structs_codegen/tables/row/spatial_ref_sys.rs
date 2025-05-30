impl From<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy) -> Self {
        super::Row::SpatialRefSy(value)
    }
}
impl TryFrom<super::Row>
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Row) -> Result<Self, Self::Error> {
        match value {
            super::Row::SpatialRefSy(v) => Ok(v),
            value => unreachable!("Unexpected variant in Row enum: {value:?}"),
        }
    }
}
