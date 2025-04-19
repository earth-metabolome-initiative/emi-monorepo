#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableStepStorageContainerBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::step_storage_containers::StepStorageContainer;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableStepStorageContainer;
}
