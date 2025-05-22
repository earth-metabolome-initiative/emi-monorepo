#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepStorageContainer;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepStorageContainerBuilder;
}
