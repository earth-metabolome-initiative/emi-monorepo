#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder
{
    type Row = crate::codegen::structs_codegen::tables::samples::Sample;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSample;
}
