#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder;
}
