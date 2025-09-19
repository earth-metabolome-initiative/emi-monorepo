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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::SpatialRefSy(v) => Some(v),
            _ => None,
        }
    }
}
