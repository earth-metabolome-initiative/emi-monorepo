#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::samples::Sample
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSample;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder;
}
