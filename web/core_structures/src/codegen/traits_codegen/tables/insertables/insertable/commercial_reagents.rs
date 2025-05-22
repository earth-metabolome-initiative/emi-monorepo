#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagent;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder;
}
