impl web_common_traits::database::Insertable for crate::DigitalAsset {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAsset;
}
