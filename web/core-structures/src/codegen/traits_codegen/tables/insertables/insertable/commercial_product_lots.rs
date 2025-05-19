#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder;
}
