#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::derived_samples::DerivedSample
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDerivedSample;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDerivedSampleBuilder;
}
