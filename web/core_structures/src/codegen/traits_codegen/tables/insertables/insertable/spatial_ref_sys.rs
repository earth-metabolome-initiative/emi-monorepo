#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder;
}
