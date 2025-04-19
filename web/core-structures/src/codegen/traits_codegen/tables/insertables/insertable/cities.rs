#[cfg(feature = "postgres")]
impl web_common_traits::database::Insertable
    for crate::codegen::structs_codegen::tables::cities::City
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCity;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableCityBuilder;
}
