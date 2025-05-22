#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentModelBuilder {
    type Row = crate::codegen::structs_codegen::tables::commercial_reagent_models::CommercialReagentModel;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialReagentModel;
}
