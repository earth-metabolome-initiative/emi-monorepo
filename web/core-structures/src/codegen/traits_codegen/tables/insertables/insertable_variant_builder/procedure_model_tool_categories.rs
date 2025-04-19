#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelToolCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::procedure_model_tool_categories::ProcedureModelToolCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelToolCategory;
}
