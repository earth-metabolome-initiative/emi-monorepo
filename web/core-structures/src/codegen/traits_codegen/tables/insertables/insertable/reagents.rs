#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::reagents::Reagent
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableReagent;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder;
}
