#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::sample_states::SampleState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableSampleState;
}
