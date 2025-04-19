#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepNameplateModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepNameplateModelBuilder;
}
