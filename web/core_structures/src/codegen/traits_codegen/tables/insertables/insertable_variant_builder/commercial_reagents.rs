#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentBuilder
{
    type Row = crate::codegen::structs_codegen::tables::commercial_reagents::CommercialReagent;
    type Product =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagent;
}
