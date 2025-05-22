#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
{
    type Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy;
}
