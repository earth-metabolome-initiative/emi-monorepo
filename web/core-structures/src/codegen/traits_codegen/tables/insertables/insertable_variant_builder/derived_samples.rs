#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableDerivedSampleBuilder
{
    type Row = crate::codegen::structs_codegen::tables::derived_samples::DerivedSample;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableDerivedSample;
}
