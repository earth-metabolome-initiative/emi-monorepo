#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::projects::Project
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProject;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder;
}
