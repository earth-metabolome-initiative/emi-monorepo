#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::procedures::Procedure
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedure;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder;
}
