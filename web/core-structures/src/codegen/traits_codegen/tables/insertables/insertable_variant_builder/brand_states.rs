#[cfg(feature = "postgres")]
impl web_common_traits::database::InsertableBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableBrandStateBuilder
{
    type Row = crate::codegen::structs_codegen::tables::brand_states::BrandState;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableBrandState;
}
