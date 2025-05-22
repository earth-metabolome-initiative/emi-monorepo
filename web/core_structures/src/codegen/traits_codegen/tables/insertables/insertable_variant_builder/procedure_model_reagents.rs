#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelReagentBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::procedure_model_reagents::ProcedureModelReagent;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelReagent;
}
