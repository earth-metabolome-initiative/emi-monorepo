#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::project_states::ProjectState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProjectState;
}
