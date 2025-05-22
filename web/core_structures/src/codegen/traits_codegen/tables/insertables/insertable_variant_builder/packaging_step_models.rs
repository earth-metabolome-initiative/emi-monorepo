#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingStepModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingStepModel;
}
