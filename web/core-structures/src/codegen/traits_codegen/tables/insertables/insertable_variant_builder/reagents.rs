#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::reagents::Reagent;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableReagent;
}
