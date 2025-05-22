#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableTeamProjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::team_projects::TeamProject;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTeamProject;
}
