#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder;
}
