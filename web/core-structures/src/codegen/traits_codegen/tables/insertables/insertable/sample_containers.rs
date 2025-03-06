#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::sample_containers::SampleContainer
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainer;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainerBuilder;
}
