#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
{
    type Row = crate::codegen::structs_codegen::tables::brands::Brand;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableBrand;
}
