#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::project_states::ProjectState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectStateBuilder;
}
