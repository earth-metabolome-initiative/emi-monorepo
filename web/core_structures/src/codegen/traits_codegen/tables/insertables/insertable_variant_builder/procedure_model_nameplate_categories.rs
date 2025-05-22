#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelNameplateCategoryBuilder {
    type Row = crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelNameplateCategory;
}
