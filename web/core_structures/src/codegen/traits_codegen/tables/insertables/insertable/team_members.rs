impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::team_members::TeamMember
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember;
}
