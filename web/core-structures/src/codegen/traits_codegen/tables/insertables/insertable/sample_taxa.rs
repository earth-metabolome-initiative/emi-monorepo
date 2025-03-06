#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleTaxa;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleTaxaBuilder;
}
