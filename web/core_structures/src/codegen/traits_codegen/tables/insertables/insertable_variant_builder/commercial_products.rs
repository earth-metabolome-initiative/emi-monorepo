#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductBuilder
{
    type Row = crate::codegen::structs_codegen::tables::commercial_products::CommercialProduct;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProduct;
}
