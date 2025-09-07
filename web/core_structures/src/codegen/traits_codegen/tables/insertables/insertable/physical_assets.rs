impl web_common_traits::database::Insertable for crate::PhysicalAsset {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset;
}
