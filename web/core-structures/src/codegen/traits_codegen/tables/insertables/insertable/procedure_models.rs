#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder;
}
