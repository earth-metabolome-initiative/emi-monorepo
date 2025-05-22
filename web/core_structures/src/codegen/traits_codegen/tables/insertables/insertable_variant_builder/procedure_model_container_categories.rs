#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelContainerCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::procedure_model_container_categories::ProcedureModelContainerCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelContainerCategory;
}
