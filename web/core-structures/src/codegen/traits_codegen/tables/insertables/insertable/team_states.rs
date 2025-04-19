#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::team_states::TeamState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamStateBuilder;
}
