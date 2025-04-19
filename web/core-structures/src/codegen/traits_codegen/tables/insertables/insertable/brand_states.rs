#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::brand_states::BrandState
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableBrandState;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableBrandStateBuilder;
}
