#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder
{
    type Row =
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot;
}
