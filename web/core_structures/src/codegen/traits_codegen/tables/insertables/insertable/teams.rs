impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::teams::Team
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeam;
}
