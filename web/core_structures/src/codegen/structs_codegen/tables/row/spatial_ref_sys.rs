impl From<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy> for super::Row {
    fn from(value: crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy) -> Self {
        super::Row::SpatialRefSy(value)
    }
}
impl From<super::Row>
    for Option<crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy>
{
    fn from(value: super::Row) -> Self {
        match value {
            super::Row::SpatialRefSy(v) => Some(v),
            _ => None,
        }
    }
}
