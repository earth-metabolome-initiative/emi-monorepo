#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder
{
    type Row = crate::codegen::structs_codegen::tables::packaging_models::PackagingModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModel;
}
