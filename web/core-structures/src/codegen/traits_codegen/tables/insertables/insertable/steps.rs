#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::steps::Step
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStep;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepBuilder;
}
