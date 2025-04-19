#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::procedure_step_models::ProcedureStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureStepModelBuilder;
}
