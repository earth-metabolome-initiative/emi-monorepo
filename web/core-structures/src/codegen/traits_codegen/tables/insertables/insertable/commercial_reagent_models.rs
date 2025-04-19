#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentModel;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentModelBuilder;
}
