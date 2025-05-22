#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
{
    type Row = crate::codegen::structs_codegen::tables::procedures::Procedure;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedure;
}
