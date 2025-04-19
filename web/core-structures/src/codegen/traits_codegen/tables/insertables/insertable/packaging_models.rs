#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::packaging_models::PackagingModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModel;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder;
}
