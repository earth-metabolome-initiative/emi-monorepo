#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModelBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableParentProcedureModel;
}
