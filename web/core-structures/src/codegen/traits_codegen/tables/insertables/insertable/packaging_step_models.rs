#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingStepModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingStepModelBuilder;
}
