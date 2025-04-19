#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_states::TeamState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTeamState;
}
