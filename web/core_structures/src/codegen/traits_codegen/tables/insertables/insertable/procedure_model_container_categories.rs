#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelContainerCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelContainerCategoryBuilder;
}
