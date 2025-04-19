#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProjectBuilder
{
    type Row = crate::codegen::structs_codegen::tables::projects::Project;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProject;
}
