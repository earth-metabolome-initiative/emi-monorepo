impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::assets::Asset
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAsset;
}
