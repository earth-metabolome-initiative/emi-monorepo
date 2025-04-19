#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
for crate::codegen::structs_codegen::tables::procedure_model_nameplate_categories::ProcedureModelNameplateCategory {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelNameplateCategory;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelNameplateCategoryBuilder;
}
