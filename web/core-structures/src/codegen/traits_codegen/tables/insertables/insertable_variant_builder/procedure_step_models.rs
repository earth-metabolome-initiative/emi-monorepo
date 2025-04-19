#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureStepModel;
}
