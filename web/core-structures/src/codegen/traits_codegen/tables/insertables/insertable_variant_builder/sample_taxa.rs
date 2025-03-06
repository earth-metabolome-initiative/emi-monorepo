#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleTaxaBuilder
{
    type Row = crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSampleTaxa;
}
