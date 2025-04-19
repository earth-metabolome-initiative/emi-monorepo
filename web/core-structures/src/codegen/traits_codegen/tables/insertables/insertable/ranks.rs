#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::ranks::Rank
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableRank;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableRankBuilder;
}
