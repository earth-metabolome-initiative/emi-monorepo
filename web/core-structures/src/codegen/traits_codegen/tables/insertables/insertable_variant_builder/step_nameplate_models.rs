#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepNameplateModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::step_nameplate_models::StepNameplateModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepNameplateModel;
}
