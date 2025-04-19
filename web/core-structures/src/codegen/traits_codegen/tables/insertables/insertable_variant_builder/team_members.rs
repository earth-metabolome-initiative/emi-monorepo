#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_members::TeamMember;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember;
}
