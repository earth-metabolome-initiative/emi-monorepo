#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelToolCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelToolCategoryBuilder;
}
