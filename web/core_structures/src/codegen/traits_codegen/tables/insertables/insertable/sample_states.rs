#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::sample_states::SampleState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleStateBuilder;
}
