impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::projects::Project
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProject;
}
