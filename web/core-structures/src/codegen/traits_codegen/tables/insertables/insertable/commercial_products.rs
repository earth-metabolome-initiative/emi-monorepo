#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProduct;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder;
}
