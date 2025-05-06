#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::centrifuge_step_models::CentrifugeStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepModelBuilder;
}
