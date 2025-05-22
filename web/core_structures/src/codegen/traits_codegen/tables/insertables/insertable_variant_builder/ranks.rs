#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableRankBuilder
{
    type Row = crate::codegen::structs_codegen::tables::ranks::Rank;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableRank;
}
