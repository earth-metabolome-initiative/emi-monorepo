#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModel;
}
