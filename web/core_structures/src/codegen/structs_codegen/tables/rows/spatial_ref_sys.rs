impl From<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>,
    ) -> Self {
        super::Rows::SpatialRefSy(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SpatialRefSy(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
