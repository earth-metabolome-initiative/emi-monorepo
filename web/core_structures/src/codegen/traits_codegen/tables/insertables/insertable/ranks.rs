impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::ranks::Rank
{
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableRankBuilder;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableRank;
}
