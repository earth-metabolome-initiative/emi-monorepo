#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepModelCategoryBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_model_categories::StepModelCategory;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepModelCategory;
}
