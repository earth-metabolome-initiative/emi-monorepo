#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::centrifuge_steps::CentrifugeStep
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeStepBuilder;
}
