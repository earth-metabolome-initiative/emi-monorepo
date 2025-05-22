#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelReagent;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelReagentBuilder;
}
